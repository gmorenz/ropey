use std;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::sync::Arc;

use iter::{Bytes, Chars, Chunks, Lines};
use rope::Rope;
use str_utils::{
    byte_to_char_idx, byte_to_line_idx, char_to_byte_idx, char_to_line_idx, count_chars,
    count_line_breaks, line_to_byte_idx, line_to_char_idx,
};
use tree::{Count, Node};

/// An immutable view into part of a `Rope`.
///
/// Just like standard `&str` slices, `RopeSlice`s behave as if the text in
/// their range is the only text that exists.  All indexing is relative to
/// the start of their range, and all iterators and methods that return text
/// content truncate that content to the range of the slice.
///
/// In other words, the behavior of a `RopeSlice` is always identical to that
/// of a full `Rope` with the same content.
#[derive(Copy, Clone)]
pub struct RopeSlice<'a>(RSEnum<'a>);

#[derive(Copy, Clone)]
enum RSEnum<'a> {
    Full {
        node: &'a Arc<Node>,
        start_byte: Count,
        end_byte: Count,
        start_char: Count,
        end_char: Count,
        start_line_break: Count,
        end_line_break: Count,
    },
    Light {
        text: &'a str,
        char_count: Count,
        line_break_count: Count,
    },
}

impl<'a> RopeSlice<'a> {
    pub(crate) fn new_with_range(node: &'a Arc<Node>, start: usize, end: usize) -> Self {
        assert!(start <= end);
        assert!(end <= node.text_info().chars as usize);

        // Early-out shortcut for taking a slice of the full thing.
        if start == 0 && end == node.char_count() {
            if node.is_leaf() {
                let text = node.leaf_text();
                return RopeSlice(RSEnum::Light {
                    text: text,
                    char_count: (end - start) as Count,
                    line_break_count: count_line_breaks(text) as Count,
                });
            } else {
                return RopeSlice(RSEnum::Full {
                    node: node,
                    start_byte: 0,
                    end_byte: node.byte_count() as Count,
                    start_char: 0,
                    end_char: node.char_count() as Count,
                    start_line_break: 0,
                    end_line_break: node.line_break_count() as Count,
                });
            }
        }

        // Find the deepest node that still contains the full range given.
        let mut n_start = start;
        let mut n_end = end;
        let mut node = node;
        'outer: loop {
            match *(node as &Node) {
                // Early out if we reach a leaf, because we can do the
                // simpler lightweight slice then.
                Node::Leaf(ref text) => {
                    let start_byte = char_to_byte_idx(&text, n_start);
                    let end_byte =
                        start_byte + char_to_byte_idx(&text[start_byte..], n_end - n_start);
                    return RopeSlice(RSEnum::Light {
                        text: &text[start_byte..end_byte],
                        char_count: (n_end - n_start) as Count,
                        line_break_count: count_line_breaks(&text[start_byte..end_byte]) as Count,
                    });
                }

                Node::Internal(ref children) => {
                    let mut start_char = 0;
                    for (i, inf) in children.info().iter().enumerate() {
                        if n_start >= start_char && n_end < (start_char + inf.chars as usize) {
                            n_start -= start_char;
                            n_end -= start_char;
                            node = &children.nodes()[i];
                            continue 'outer;
                        }
                        start_char += inf.chars as usize;
                    }
                    break;
                }
            }
        }

        // Create the slice
        let (start_byte, start_line) = node.char_to_byte_and_line(n_start);
        let (end_byte, end_line) = node.char_to_byte_and_line(n_end);
        RopeSlice(RSEnum::Full {
            node: node,
            start_byte: start_byte as Count,
            end_byte: end_byte as Count,
            start_char: n_start as Count,
            end_char: n_end as Count,
            start_line_break: start_line as Count,
            end_line_break: end_line as Count,
        })
    }

    //-----------------------------------------------------------------------
    // Informational methods

    /// Total number of bytes in the `RopeSlice`.
    ///
    /// Runs in O(1) time.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        match *self {
            RopeSlice(RSEnum::Full {
                end_byte,
                start_byte,
                ..
            }) => (end_byte - start_byte) as usize,
            RopeSlice(RSEnum::Light { text, .. }) => text.len(),
        }
    }

    /// Total number of chars in the `RopeSlice`.
    ///
    /// Runs in O(1) time.
    #[inline]
    pub fn len_chars(&self) -> usize {
        match *self {
            RopeSlice(RSEnum::Full {
                end_char,
                start_char,
                ..
            }) => (end_char - start_char) as usize,
            RopeSlice(RSEnum::Light { char_count, .. }) => char_count as usize,
        }
    }

    /// Total number of lines in the `RopeSlice`.
    ///
    /// Runs in O(1) time.
    #[inline]
    pub fn len_lines(&self) -> usize {
        match *self {
            RopeSlice(RSEnum::Full {
                end_line_break,
                start_line_break,
                ..
            }) => (end_line_break - start_line_break) as usize + 1,
            RopeSlice(RSEnum::Light {
                line_break_count, ..
            }) => line_break_count as usize + 1,
        }
    }

    //-----------------------------------------------------------------------
    // Index conversion methods

    /// Returns the char index of the given byte.
    ///
    /// Notes:
    ///
    /// - If the byte is in the middle of a multi-byte char, returns the
    ///   index of the char that the byte belongs to.
    /// - `byte_idx` can be one-past-the-end, which will return one-past-the-end
    ///   char index.
    ///
    /// # Panics
    ///
    /// Panics if `byte_idx` is out of bounds (i.e. `byte_idx > len_bytes()`).
    #[inline]
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        // Bounds check
        assert!(
            byte_idx <= self.len_bytes(),
            "Attempt to index past end of slice: byte index {}, slice byte length {}",
            byte_idx,
            self.len_bytes()
        );

        let (chunk, b, c, _) = self.chunk_at_byte(byte_idx);
        c + byte_to_char_idx(chunk, byte_idx - b)
    }

    /// Returns the line index of the given byte.
    ///
    /// Notes:
    ///
    /// - Lines are zero-indexed.  This is functionally equivalent to
    ///   counting the line endings before the specified byte.
    /// - `byte_idx` can be one-past-the-end, which will return the
    ///   last line index.
    ///
    /// # Panics
    ///
    /// Panics if `byte_idx` is out of bounds (i.e. `byte_idx > len_bytes()`).
    #[inline]
    pub fn byte_to_line(&self, byte_idx: usize) -> usize {
        // Bounds check
        assert!(
            byte_idx <= self.len_bytes(),
            "Attempt to index past end of slice: byte index {}, slice byte length {}",
            byte_idx,
            self.len_bytes()
        );

        let (chunk, b, _, l) = self.chunk_at_byte(byte_idx);
        l + byte_to_line_idx(chunk, byte_idx - b)
    }

    /// Returns the byte index of the given char.
    ///
    /// Notes:
    ///
    /// - `char_idx` can be one-past-the-end, which will return
    ///   one-past-the-end byte index.
    ///
    /// # Panics
    ///
    /// Panics if `char_idx` is out of bounds (i.e. `char_idx > len_chars()`).
    #[inline]
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        // Bounds check
        assert!(
            char_idx <= self.len_chars(),
            "Attempt to index past end of slice: char index {}, slice char length {}",
            char_idx,
            self.len_chars()
        );

        let (chunk, b, c, _) = self.chunk_at_char(char_idx);
        b + char_to_byte_idx(chunk, char_idx - c)
    }

    /// Returns the line index of the given char.
    ///
    /// Notes:
    ///
    /// - Lines are zero-indexed.  This is functionally equivalent to
    ///   counting the line endings before the specified char.
    /// - `char_idx` can be one-past-the-end, which will return the
    ///   last line index.
    ///
    /// # Panics
    ///
    /// Panics if `char_idx` is out of bounds (i.e. `char_idx > len_chars()`).
    #[inline]
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        // Bounds check
        assert!(
            char_idx <= self.len_chars(),
            "Attempt to index past end of slice: char index {}, slice char length {}",
            char_idx,
            self.len_chars()
        );

        let (chunk, _, c, l) = self.chunk_at_char(char_idx);
        l + char_to_line_idx(chunk, char_idx - c)
    }

    /// Returns the byte index of the start of the given line.
    ///
    /// Notes:
    ///
    /// - Lines are zero-indexed.
    /// - `line_idx` can be one-past-the-end, which will return
    ///   one-past-the-end byte index.
    ///
    /// # Panics
    ///
    /// Panics if `line_idx` is out of bounds (i.e. `line_idx > len_lines()`).
    #[inline]
    pub fn line_to_byte(&self, line_idx: usize) -> usize {
        // Bounds check
        assert!(
            line_idx <= self.len_lines(),
            "Attempt to index past end of slice: line index {}, slice line length {}",
            line_idx,
            self.len_lines()
        );

        if line_idx == self.len_lines() {
            self.len_bytes()
        } else {
            let (chunk, b, _, l) = self.chunk_at_line_break(line_idx);
            b + line_to_byte_idx(chunk, line_idx - l)
        }
    }

    /// Returns the char index of the start of the given line.
    ///
    /// Notes:
    ///
    /// - Lines are zero-indexed.
    /// - `line_idx` can be one-past-the-end, which will return
    ///   one-past-the-end char index.
    ///
    /// # Panics
    ///
    /// Panics if `line_idx` is out of bounds (i.e. `line_idx > len_lines()`).
    #[inline]
    pub fn line_to_char(&self, line_idx: usize) -> usize {
        // Bounds check
        assert!(
            line_idx <= self.len_lines(),
            "Attempt to index past end of slice: line index {}, slice line length {}",
            line_idx,
            self.len_lines()
        );

        if line_idx == self.len_lines() {
            self.len_chars()
        } else {
            let (chunk, _, c, l) = self.chunk_at_line_break(line_idx);
            c + line_to_char_idx(chunk, line_idx - l)
        }
    }

    //-----------------------------------------------------------------------
    // Fetch methods

    /// Returns the char at `char_idx`.
    ///
    /// # Panics
    ///
    /// Panics if `char_idx` is out of bounds (i.e. `char_idx >= len_chars()`).
    #[inline]
    pub fn char(&self, char_idx: usize) -> char {
        // Bounds check
        assert!(
            char_idx < self.len_chars(),
            "Attempt to index past end of slice: char index {}, slice char length {}",
            char_idx,
            self.len_chars()
        );

        let (chunk, _, chunk_char_idx, _) = self.chunk_at_char(char_idx);
        let byte_idx = char_to_byte_idx(chunk, char_idx - chunk_char_idx);
        chunk[byte_idx..].chars().nth(0).unwrap()
    }

    /// Returns the line at `line_idx`.
    ///
    /// Note: lines are zero-indexed.
    ///
    /// # Panics
    ///
    /// Panics if `line_idx` is out of bounds (i.e. `line_idx >= len_lines()`).
    #[inline]
    pub fn line(&self, line_idx: usize) -> RopeSlice<'a> {
        // Bounds check
        assert!(
            line_idx < self.len_lines(),
            "Attempt to index past end of slice: line index {}, slice line length {}",
            line_idx,
            self.len_lines()
        );

        let (chunk_1, _, c1, l1) = self.chunk_at_line_break(line_idx);
        let (chunk_2, _, c2, l2) = self.chunk_at_line_break(line_idx + 1);
        if c1 == c2 {
            let text1 = &chunk_1[line_to_byte_idx(chunk_1, line_idx - l1)..];
            let text2 = &text1[..line_to_byte_idx(text1, 1)];
            RopeSlice::from_str(text2)
        } else {
            let start = c1 + line_to_char_idx(chunk_1, line_idx - l1);
            let end = c2 + line_to_char_idx(chunk_2, line_idx + 1 - l2);
            self.slice(start..end)
        }
    }

    /// Returns the chunk containing the given byte index.
    ///
    /// Also returns the byte and char indices of the beginning of the chunk
    /// and the index of the line that the chunk starts on.
    ///
    /// The return value is organized as `(chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx)`.
    ///
    /// # Panics
    ///
    /// Panics if `byte_idx` is out of bounds (i.e. `byte_idx > len_bytes()`).
    pub fn chunk_at_byte(&self, byte_idx: usize) -> (&'a str, usize, usize, usize) {
        // Bounds check
        assert!(
            byte_idx <= self.len_bytes(),
            "Attempt to index past end of slice: byte index {}, slice byte length {}",
            byte_idx,
            self.len_bytes()
        );

        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_byte,
                end_byte,
                start_char,
                start_line_break,
                ..
            }) => {
                // Get the chunk.
                let (chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx) =
                    node.get_chunk_at_byte(byte_idx + start_byte as usize);

                // Calculate clipped start/end byte indices within the chunk.
                let chunk_start_byte_idx =
                    start_byte as usize - chunk_byte_idx.min(start_byte as usize);
                let chunk_end_byte_idx = chunk.len().min(end_byte as usize - chunk_byte_idx);

                // Return the clipped chunk and byte offset.
                (
                    &chunk[chunk_start_byte_idx..chunk_end_byte_idx],
                    chunk_byte_idx - (start_byte as usize).min(chunk_byte_idx),
                    chunk_char_idx - (start_char as usize).min(chunk_char_idx),
                    chunk_line_idx - (start_line_break as usize).min(chunk_line_idx),
                )
            }
            RopeSlice(RSEnum::Light { text, .. }) => (text, 0, 0, 0),
        }
    }

    /// Returns the chunk containing the given char index.
    ///
    /// Also returns the byte and char indices of the beginning of the chunk
    /// and the index of the line that the chunk starts on.
    ///
    /// The return value is organized as `(chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx)`.
    ///
    /// # Panics
    ///
    /// Panics if `char_idx` is out of bounds (i.e. `char_idx > len_chars()`).
    pub fn chunk_at_char(&self, char_idx: usize) -> (&'a str, usize, usize, usize) {
        // Bounds check
        assert!(
            char_idx <= self.len_chars(),
            "Attempt to index past end of slice: char index {}, slice char length {}",
            char_idx,
            self.len_chars()
        );

        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_byte,
                end_byte,
                start_char,
                start_line_break,
                ..
            }) => {
                // Get the chunk.
                let (chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx) =
                    node.get_chunk_at_char(char_idx + start_char as usize);

                // Calculate clipped start/end byte indices within the chunk.
                let chunk_start_byte_idx =
                    start_byte as usize - chunk_byte_idx.min(start_byte as usize);
                let chunk_end_byte_idx = chunk.len().min(end_byte as usize - chunk_byte_idx);

                // Return the clipped chunk and byte offset.
                (
                    &chunk[chunk_start_byte_idx..chunk_end_byte_idx],
                    chunk_byte_idx - (start_byte as usize).min(chunk_byte_idx),
                    chunk_char_idx - (start_char as usize).min(chunk_char_idx),
                    chunk_line_idx - (start_line_break as usize).min(chunk_line_idx),
                )
            }
            RopeSlice(RSEnum::Light { text, .. }) => (text, 0, 0, 0),
        }
    }

    /// Returns the chunk containing the given line break.
    ///
    /// Also returns the byte and char indices of the beginning of the chunk
    /// and the index of the line that the chunk starts on.
    ///
    /// Note: for convenience, both the beginning and end of the slice are
    /// considered line breaks for the purposes of indexing.  For example, in
    /// the string `"Hello \n world!"` 0 would give the first chunk, 1 would
    /// give the chunk containing the newline character, and 2 would give the
    /// last chunk.
    ///
    /// The return value is organized as `(chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx)`.
    ///
    /// # Panics
    ///
    /// Panics if `line_break_idx` is out of bounds (i.e. `line_break_idx > len_lines()`).
    pub fn chunk_at_line_break(&self, line_break_idx: usize) -> (&'a str, usize, usize, usize) {
        // Bounds check
        assert!(
            line_break_idx <= self.len_lines(),
            "Attempt to index past end of Rope: line break index {}, max index {}",
            line_break_idx,
            self.len_lines()
        );

        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_byte,
                end_byte,
                start_char,
                start_line_break,
                ..
            }) => {
                // Get the chunk.
                let (chunk, chunk_byte_idx, chunk_char_idx, chunk_line_idx) = if line_break_idx == 0
                {
                    node.get_chunk_at_byte(start_byte as usize)
                } else if line_break_idx == self.len_lines() {
                    node.get_chunk_at_byte(end_byte as usize)
                } else {
                    node.get_chunk_at_line_break(line_break_idx + start_line_break as usize)
                };

                // Calculate clipped start/end byte indices within the chunk.
                let chunk_start_byte_idx =
                    start_byte as usize - chunk_byte_idx.min(start_byte as usize);
                let chunk_end_byte_idx = chunk.len().min(end_byte as usize - chunk_byte_idx);

                // Return the clipped chunk and byte offset.
                (
                    &chunk[chunk_start_byte_idx..chunk_end_byte_idx],
                    chunk_byte_idx - (start_byte as usize).min(chunk_byte_idx),
                    chunk_char_idx - (start_char as usize).min(chunk_char_idx),
                    chunk_line_idx - (start_line_break as usize).min(chunk_line_idx),
                )
            }
            RopeSlice(RSEnum::Light { text, .. }) => (text, 0, 0, 0),
        }
    }

    /// Returns the entire contents of the `RopeSlice` as a `&str` if
    /// possible.
    ///
    /// This is useful for optimizing cases where the slice is only a few
    /// characters or words, and therefore has a high chance of being
    /// contiguous in memory.
    ///
    /// For large slices this method will typically fail and return `None`
    /// because large slices usually cross chunk boundaries in the rope.
    ///
    /// Runs in O(1) time.
    #[inline]
    pub fn as_str(&self) -> Option<&'a str> {
        match *self {
            RopeSlice(RSEnum::Full { .. }) => None,
            RopeSlice(RSEnum::Light { text, .. }) => Some(text),
        }
    }

    //-----------------------------------------------------------------------
    // Slice creation

    /// Creates a `RopeSlice` directly from a string slice.
    ///
    /// Despite its straightforward name, the useful applications of this
    /// method are somewhat narrow.  It is intended primarily as an aid when
    /// implementing additional functionality on top of Ropey, where you may
    /// already have access to a rope chunk and want to directly create a
    /// `RopeSlice` from it, avoiding the overhead of going through the
    /// slicing APIs.
    ///
    /// Although it is possible to use this to create `RopeSlice`s from
    /// arbitrary strings, doing so is not especially useful.  For example,
    /// `Rope`s and `RopeSlice`s can already be directly compared for
    /// equality with strings and string slices.
    ///
    /// Runs in O(N) time, where N is the length of the string slice.
    #[inline]
    pub fn from_str(text: &str) -> RopeSlice {
        RopeSlice(RSEnum::Light {
            text: text,
            char_count: count_chars(text) as Count,
            line_break_count: count_line_breaks(text) as Count,
        })
    }

    /// Returns a sub-slice of the `RopeSlice` in the given char index range.
    ///
    /// Uses range syntax, e.g. `2..7`, `2..`, etc.
    ///
    /// # Panics
    ///
    /// Panics if the start of the range is greater than the end, or the end
    /// is out of bounds (i.e. `end > len_chars()`).
    pub fn slice<R: CharIdxRange>(&self, char_range: R) -> Self {
        // Early-out shortcut for taking a slice of the full thing.
        if char_range.start() == None && char_range.end() == None {
            return *self;
        }

        let start = char_range.start().unwrap_or(0);
        let end = char_range.end().unwrap_or_else(|| self.len_chars());

        // Bounds check
        assert!(start <= end);
        assert!(
            end <= self.len_chars(),
            "Attempt to slice past end of RopeSlice: slice end {}, RopeSlice length {}",
            end,
            self.len_chars()
        );

        match *self {
            RopeSlice(RSEnum::Full {
                node, start_char, ..
            }) => RopeSlice::new_with_range(
                node,
                start_char as usize + start,
                start_char as usize + end,
            ),
            RopeSlice(RSEnum::Light { text, .. }) => {
                let start_byte = char_to_byte_idx(text, start);
                let end_byte = char_to_byte_idx(text, end);
                let new_text = &text[start_byte..end_byte];
                RopeSlice(RSEnum::Light {
                    text: new_text,
                    char_count: (end - start) as Count,
                    line_break_count: count_line_breaks(new_text) as Count,
                })
            }
        }
    }

    //-----------------------------------------------------------------------
    // Iterator methods

    /// Creates an iterator over the bytes of the `RopeSlice`.
    #[inline]
    pub fn bytes(&self) -> Bytes<'a> {
        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_char,
                end_char,
                ..
            }) => Bytes::new_with_range(node, start_char as usize, end_char as usize),
            RopeSlice(RSEnum::Light { text, .. }) => Bytes::from_str(text),
        }
    }

    /// Creates an iterator over the chars of the `RopeSlice`.
    #[inline]
    pub fn chars(&self) -> Chars<'a> {
        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_char,
                end_char,
                ..
            }) => Chars::new_with_range(node, start_char as usize, end_char as usize),
            RopeSlice(RSEnum::Light { text, .. }) => Chars::from_str(text),
        }
    }

    /// Creates an iterator over the lines of the `RopeSlice`.
    #[inline]
    pub fn lines(&self) -> Lines<'a> {
        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_char,
                end_char,
                ..
            }) => Lines::new_with_range(node, start_char as usize, end_char as usize),
            RopeSlice(RSEnum::Light { text, .. }) => Lines::from_str(text),
        }
    }

    /// Creates an iterator over the chunks of the `RopeSlice`.
    #[inline]
    pub fn chunks(&self) -> Chunks<'a> {
        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_char,
                end_char,
                ..
            }) => Chunks::new_with_range(node, start_char as usize, end_char as usize),
            RopeSlice(RSEnum::Light { text, .. }) => Chunks::from_str(text),
        }
    }

    //-----------------------------------------------------------------------
    // Conversion methods

    /// Returns the entire text of the `RopeSlice` as a newly allocated `String`.
    ///
    /// Runs in O(N) time.
    pub fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len_bytes());
        for chunk in self.chunks() {
            text.push_str(chunk);
        }
        text
    }

    /// Creates a new `Rope` from the contents of the `RopeSlice`.
    ///
    /// Shares data where possible.
    ///
    /// Perhaps unexpectedly, runs in O(log N) time, not O(1) time like
    /// `Rope` cloning.
    pub fn to_rope(&self) -> Rope {
        match *self {
            RopeSlice(RSEnum::Full {
                node,
                start_char,
                end_char,
                ..
            }) => {
                let mut rope = Rope {
                    root: Arc::clone(node),
                };

                // Chop off right end if needed
                if end_char < node.text_info().chars {
                    rope.split_off(end_char as usize);
                }

                // Chop off left end if needed
                if start_char > 0 {
                    rope = rope.split_off(start_char as usize);
                }

                // Return the rope
                rope
            }
            RopeSlice(RSEnum::Light { text, .. }) => Rope::from_str(text),
        }
    }
}

//==============================================================

impl<'a> std::fmt::Debug for RopeSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list().entries(self.chunks()).finish()
    }
}

impl<'a> std::fmt::Display for RopeSlice<'a> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for chunk in self.chunks() {
            write!(f, "{}", chunk)?
        }
        Ok(())
    }
}

impl<'a, 'b> std::cmp::PartialEq<RopeSlice<'b>> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &RopeSlice<'b>) -> bool {
        if self.len_bytes() != other.len_bytes() {
            return false;
        }

        let mut chunk_itr_1 = self.chunks();
        let mut chunk_itr_2 = other.chunks();
        let mut chunk1 = chunk_itr_1.next().unwrap_or("");
        let mut chunk2 = chunk_itr_2.next().unwrap_or("");

        loop {
            if chunk1.len() > chunk2.len() {
                if &chunk1[..chunk2.len()] != chunk2 {
                    return false;
                } else {
                    chunk1 = &chunk1[chunk2.len()..];
                    chunk2 = "";
                }
            } else if &chunk2[..chunk1.len()] != chunk1 {
                return false;
            } else {
                chunk2 = &chunk2[chunk1.len()..];
                chunk1 = "";
            }

            if chunk1.is_empty() {
                if let Some(chunk) = chunk_itr_1.next() {
                    chunk1 = chunk;
                } else {
                    break;
                }
            }

            if chunk2.is_empty() {
                if let Some(chunk) = chunk_itr_2.next() {
                    chunk2 = chunk;
                } else {
                    break;
                }
            }
        }

        return true;
    }
}

impl<'a, 'b> std::cmp::PartialEq<&'b str> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &&'b str) -> bool {
        match *self {
            RopeSlice(RSEnum::Full { .. }) => {
                if self.len_bytes() != other.len() {
                    return false;
                }

                let mut idx = 0;
                for chunk in self.chunks() {
                    if chunk != &other[idx..(idx + chunk.len())] {
                        return false;
                    }
                    idx += chunk.len();
                }

                return true;
            }
            RopeSlice(RSEnum::Light { text, .. }) => {
                return text == *other;
            }
        }
    }
}

impl<'a, 'b> std::cmp::PartialEq<RopeSlice<'a>> for &'b str {
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        other == self
    }
}

impl<'a> std::cmp::PartialEq<str> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        std::cmp::PartialEq::<&str>::eq(self, &other)
    }
}

impl<'a> std::cmp::PartialEq<RopeSlice<'a>> for str {
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        std::cmp::PartialEq::<&str>::eq(other, &self)
    }
}

impl<'a> std::cmp::PartialEq<String> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self == other.as_str()
    }
}

impl<'a> std::cmp::PartialEq<RopeSlice<'a>> for String {
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        self.as_str() == other
    }
}

impl<'a, 'b> std::cmp::PartialEq<std::borrow::Cow<'b, str>> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &std::borrow::Cow<'b, str>) -> bool {
        *self == **other
    }
}

impl<'a, 'b> std::cmp::PartialEq<RopeSlice<'a>> for std::borrow::Cow<'b, str> {
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        **self == *other
    }
}

impl<'a> std::cmp::PartialEq<Rope> for RopeSlice<'a> {
    #[inline]
    fn eq(&self, other: &Rope) -> bool {
        *self == other.slice(..)
    }
}

impl<'a> std::cmp::PartialEq<RopeSlice<'a>> for Rope {
    #[inline]
    fn eq(&self, other: &RopeSlice<'a>) -> bool {
        self.slice(..) == *other
    }
}

//===========================================================

/// Trait to generalize over the various `Range` types for `a..b` syntax when
/// expressing char ranges.
pub trait CharIdxRange {
    fn start(&self) -> Option<usize>;
    fn end(&self) -> Option<usize>;
}

impl CharIdxRange for Range<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }
    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl CharIdxRange for RangeTo<usize> {
    fn start(&self) -> Option<usize> {
        None
    }
    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}

impl CharIdxRange for RangeFrom<usize> {
    fn start(&self) -> Option<usize> {
        Some(self.start)
    }
    fn end(&self) -> Option<usize> {
        None
    }
}

impl CharIdxRange for RangeFull {
    fn start(&self) -> Option<usize> {
        None
    }
    fn end(&self) -> Option<usize> {
        None
    }
}

//===========================================================

#[cfg(test)]
mod tests {
    use str_utils::{byte_to_char_idx, byte_to_line_idx, char_to_byte_idx, char_to_line_idx};
    use Rope;

    // 127 bytes, 103 chars, 1 line
    const TEXT: &str = "Hello there!  How're you doing?  It's \
                        a fine day, isn't it?  Aren't you glad \
                        we're alive?  こんにちは、みんなさん！";
    // 124 bytes, 100 chars, 4 lines
    const TEXT_LINES: &str = "Hello there!  How're you doing?\nIt's \
                              a fine day, isn't it?\nAren't you glad \
                              we're alive?\nこんにちは、みんなさん！";

    #[test]
    fn len_bytes_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(7..98);
        assert_eq!(s.len_bytes(), 105);
    }

    #[test]
    fn len_bytes_02() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(43..43);
        assert_eq!(s.len_bytes(), 0);
    }

    #[test]
    fn len_chars_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(7..98);
        assert_eq!(s.len_chars(), 91);
    }

    #[test]
    fn len_chars_02() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(43..43);
        assert_eq!(s.len_chars(), 0);
    }

    #[test]
    fn len_lines_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..98);
        assert_eq!(s.len_lines(), 3);
    }

    #[test]
    fn len_lines_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(43..43);
        assert_eq!(s.len_lines(), 1);
    }

    #[test]
    fn byte_to_char_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(88..102);

        // ?  こんにちは、みんなさん

        assert_eq!(0, s.byte_to_char(0));
        assert_eq!(1, s.byte_to_char(1));
        assert_eq!(2, s.byte_to_char(2));

        assert_eq!(3, s.byte_to_char(3));
        assert_eq!(3, s.byte_to_char(4));
        assert_eq!(3, s.byte_to_char(5));

        assert_eq!(4, s.byte_to_char(6));
        assert_eq!(4, s.byte_to_char(7));
        assert_eq!(4, s.byte_to_char(8));

        assert_eq!(13, s.byte_to_char(33));
        assert_eq!(13, s.byte_to_char(34));
        assert_eq!(13, s.byte_to_char(35));
        assert_eq!(14, s.byte_to_char(36));
    }

    #[test]
    fn byte_to_line_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        // 's a fine day, isn't it?\nAren't you glad \
        // we're alive?\nこんにちは、みん

        assert_eq!(0, s.byte_to_line(0));
        assert_eq!(0, s.byte_to_line(1));

        assert_eq!(0, s.byte_to_line(24));
        assert_eq!(1, s.byte_to_line(25));
        assert_eq!(1, s.byte_to_line(26));

        assert_eq!(1, s.byte_to_line(53));
        assert_eq!(2, s.byte_to_line(54));
        assert_eq!(2, s.byte_to_line(57));

        assert_eq!(2, s.byte_to_line(78));
    }

    #[test]
    fn byte_to_line_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(50..50);
        assert_eq!(0, s.byte_to_line(0));
    }

    #[test]
    fn byte_to_line_03() {
        let r = Rope::from_str("Hi there\nstranger!");
        let s = r.slice(0..9);
        assert_eq!(0, s.byte_to_line(0));
        assert_eq!(0, s.byte_to_line(8));
        assert_eq!(1, s.byte_to_line(9));
    }

    #[test]
    #[should_panic]
    fn byte_to_line_04() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        s.byte_to_line(79);
    }

    #[test]
    fn char_to_byte_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(88..102);

        // ?  こんにちは、みんなさん

        assert_eq!(0, s.char_to_byte(0));
        assert_eq!(1, s.char_to_byte(1));
        assert_eq!(2, s.char_to_byte(2));

        assert_eq!(3, s.char_to_byte(3));
        assert_eq!(6, s.char_to_byte(4));
        assert_eq!(33, s.char_to_byte(13));
        assert_eq!(36, s.char_to_byte(14));
    }

    #[test]
    fn char_to_line_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        // 's a fine day, isn't it?\nAren't you glad \
        // we're alive?\nこんにちは、みん

        assert_eq!(0, s.char_to_line(0));
        assert_eq!(0, s.char_to_line(1));

        assert_eq!(0, s.char_to_line(24));
        assert_eq!(1, s.char_to_line(25));
        assert_eq!(1, s.char_to_line(26));

        assert_eq!(1, s.char_to_line(53));
        assert_eq!(2, s.char_to_line(54));
        assert_eq!(2, s.char_to_line(55));

        assert_eq!(2, s.char_to_line(62));
    }

    #[test]
    fn char_to_line_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(43..43);

        assert_eq!(0, s.char_to_line(0));
    }

    #[test]
    fn char_to_line_03() {
        let r = Rope::from_str("Hi there\nstranger!");
        let s = r.slice(0..9);
        assert_eq!(0, s.char_to_line(0));
        assert_eq!(0, s.char_to_line(8));
        assert_eq!(1, s.char_to_line(9));
    }

    #[test]
    #[should_panic]
    fn char_to_line_04() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        s.char_to_line(63);
    }

    #[test]
    fn line_to_byte_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        // 's a fine day, isn't it?\nAren't you glad \
        // we're alive?\nこんにちは、みん

        assert_eq!(0, s.line_to_byte(0));
        assert_eq!(25, s.line_to_byte(1));
        assert_eq!(54, s.line_to_byte(2));
        assert_eq!(78, s.line_to_byte(3));
    }

    #[test]
    fn line_to_byte_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(43..43);

        assert_eq!(0, s.line_to_byte(0));
        assert_eq!(0, s.line_to_byte(1));
    }

    #[test]
    #[should_panic]
    fn line_to_byte_03() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        s.line_to_byte(4);
    }

    #[test]
    fn line_to_char_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        assert_eq!(0, s.line_to_char(0));
        assert_eq!(25, s.line_to_char(1));
        assert_eq!(54, s.line_to_char(2));
        assert_eq!(62, s.line_to_char(3));
    }

    #[test]
    fn line_to_char_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(43..43);

        assert_eq!(0, s.line_to_char(0));
        assert_eq!(0, s.line_to_char(1));
    }

    #[test]
    #[should_panic]
    fn line_to_char_03() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);

        s.line_to_char(4);
    }

    #[test]
    fn char_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(34..100);

        // t's \
        // a fine day, isn't it?  Aren't you glad \
        // we're alive?  こんにちは、みんな

        assert_eq!(s.char(0), 't');
        assert_eq!(s.char(10), ' ');
        assert_eq!(s.char(18), 'n');
        assert_eq!(s.char(65), 'な');
    }

    #[test]
    #[should_panic]
    fn char_02() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(34..100);
        s.char(66);
    }

    #[test]
    #[should_panic]
    fn char_03() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(43..43);
        s.char(0);
    }

    #[test]
    fn line_01() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        // "'s a fine day, isn't it?\nAren't you glad \
        //  we're alive?\nこんにちは、みん"

        assert_eq!(s.line(0), "'s a fine day, isn't it?\n");
        assert_eq!(s.line(1), "Aren't you glad we're alive?\n");
        assert_eq!(s.line(2), "こんにちは、みん");
    }

    #[test]
    fn line_02() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..59);
        // "'s a fine day, isn't it?\n"

        assert_eq!(s.line(0), "'s a fine day, isn't it?\n");
        assert_eq!(s.line(1), "");
    }

    #[test]
    fn line_03() {
        let r = Rope::from_str("Hi\nHi\nHi\nHi\nHi\nHi\n");
        let s = r.slice(1..17);

        assert_eq!(s.line(0), "i\n");
        assert_eq!(s.line(1), "Hi\n");
        assert_eq!(s.line(2), "Hi\n");
        assert_eq!(s.line(3), "Hi\n");
        assert_eq!(s.line(4), "Hi\n");
        assert_eq!(s.line(5), "Hi");
    }

    #[test]
    fn line_04() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(43..43);

        assert_eq!(s.line(0), "");
    }

    #[test]
    #[should_panic]
    fn line_05() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        s.line(3);
    }

    #[test]
    fn chunk_at_byte() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        let text = &TEXT_LINES[34..112];
        // "'s a fine day, isn't it?\nAren't you glad \
        //  we're alive?\nこんにちは、みん"

        let mut t = text;
        let mut prev_chunk = "";
        for i in 0..s.len_bytes() {
            let (chunk, b, c, l) = s.chunk_at_byte(i);
            assert_eq!(c, byte_to_char_idx(text, b));
            assert_eq!(l, byte_to_line_idx(text, b));
            if chunk != prev_chunk {
                assert_eq!(chunk, &t[..chunk.len()]);
                t = &t[chunk.len()..];
                prev_chunk = chunk;
            }

            let c1 = {
                let i2 = byte_to_char_idx(text, i);
                text.chars().nth(i2).unwrap()
            };
            let c2 = {
                let i2 = i - b;
                let i3 = byte_to_char_idx(chunk, i2);
                chunk.chars().nth(i3).unwrap()
            };
            assert_eq!(c1, c2);
        }

        assert_eq!(t.len(), 0);
    }

    #[test]
    fn chunk_at_char() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        let text = &TEXT_LINES[34..112];
        // "'s a fine day, isn't it?\nAren't you glad \
        //  we're alive?\nこんにちは、みん"

        let mut t = text;
        let mut prev_chunk = "";
        for i in 0..s.len_chars() {
            let (chunk, b, c, l) = s.chunk_at_char(i);
            assert_eq!(b, char_to_byte_idx(text, c));
            assert_eq!(l, char_to_line_idx(text, c));
            if chunk != prev_chunk {
                assert_eq!(chunk, &t[..chunk.len()]);
                t = &t[chunk.len()..];
                prev_chunk = chunk;
            }

            let c1 = text.chars().nth(i).unwrap();
            let c2 = {
                let i2 = i - c;
                chunk.chars().nth(i2).unwrap()
            };
            assert_eq!(c1, c2);
        }
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn chunk_at_line_break() {
        let r = Rope::from_str(TEXT_LINES);
        let s = r.slice(34..96);
        let text = &TEXT_LINES[34..112];
        // "'s a fine day, isn't it?\nAren't you glad \
        //  we're alive?\nこんにちは、みん"

        // First chunk
        {
            let (chunk, b, c, l) = s.chunk_at_line_break(0);
            assert_eq!(chunk, &text[..chunk.len()]);
            assert_eq!(b, 0);
            assert_eq!(c, 0);
            assert_eq!(l, 0);
        }

        // Middle chunks
        for i in 1..s.len_lines() {
            let (chunk, b, c, l) = s.chunk_at_line_break(i);
            assert_eq!(chunk, &text[b..(b + chunk.len())]);
            assert_eq!(c, byte_to_char_idx(text, b));
            assert_eq!(l, byte_to_line_idx(text, b));
            assert!(l < i);
            assert!(i <= byte_to_line_idx(text, b + chunk.len()));
        }

        // Last chunk
        {
            let (chunk, b, c, l) = s.chunk_at_line_break(s.len_lines());
            assert_eq!(chunk, &text[(text.len() - chunk.len())..]);
            assert_eq!(chunk, &text[b..]);
            assert_eq!(c, byte_to_char_idx(text, b));
            assert_eq!(l, byte_to_line_idx(text, b));
        }
    }

    #[test]
    fn slice_01() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(..);

        let s2 = s1.slice(..);

        assert_eq!(TEXT, s2);
    }

    #[test]
    fn slice_02() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(5..43);

        let s2 = s1.slice(3..25);

        assert_eq!(&TEXT[8..30], s2);
    }

    #[test]
    fn slice_03() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(31..97);

        let s2 = s1.slice(7..64);

        assert_eq!(&TEXT[38..103], s2);
    }

    #[test]
    fn slice_04() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(5..43);

        let s2 = s1.slice(21..21);

        assert_eq!("", s2);
    }

    #[test]
    #[should_panic]
    fn slice_05() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(5..43);

        s.slice(21..20);
    }

    #[test]
    #[should_panic]
    fn slice_06() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(5..43);

        s.slice(37..39);
    }

    #[test]
    fn eq_str_01() {
        let r = Rope::from_str(TEXT);
        let slice = r.slice(..);

        assert_eq!(slice, TEXT);
        assert_eq!(TEXT, slice);
    }

    #[test]
    fn eq_str_02() {
        let r = Rope::from_str(TEXT);
        let slice = r.slice(0..20);

        assert_ne!(slice, TEXT);
        assert_ne!(TEXT, slice);
    }

    #[test]
    fn eq_str_03() {
        let mut r = Rope::from_str(TEXT);
        r.remove(20..21);
        r.insert(20, "z");
        let slice = r.slice(..);

        assert_ne!(slice, TEXT);
        assert_ne!(TEXT, slice);
    }

    #[test]
    fn eq_str_04() {
        let r = Rope::from_str(TEXT);
        let slice = r.slice(..);
        let s: String = TEXT.into();

        assert_eq!(slice, s);
        assert_eq!(s, slice);
    }

    #[test]
    fn eq_rope_slice_01() {
        let r = Rope::from_str(TEXT);
        let s = r.slice(43..43);

        assert_eq!(s, s);
    }

    #[test]
    fn eq_rope_slice_02() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(43..97);
        let s2 = r.slice(43..97);

        assert_eq!(s1, s2);
    }

    #[test]
    fn eq_rope_slice_03() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(43..43);
        let s2 = r.slice(43..45);

        assert_ne!(s1, s2);
    }

    #[test]
    fn eq_rope_slice_04() {
        let r = Rope::from_str(TEXT);
        let s1 = r.slice(43..45);
        let s2 = r.slice(43..43);

        assert_ne!(s1, s2);
    }

    #[test]
    fn eq_rope_slice_05() {
        let r = Rope::from_str("");
        let s = r.slice(0..0);

        assert_eq!(s, s);
    }

    #[test]
    fn to_rope_01() {
        let r1 = Rope::from_str(TEXT);
        let s = r1.slice(..);
        let r2 = s.to_rope();

        assert_eq!(r1, r2);
        assert_eq!(s, r2);
    }

    #[test]
    fn to_rope_02() {
        let r1 = Rope::from_str(TEXT);
        let s = r1.slice(0..24);
        let r2 = s.to_rope();

        assert_eq!(s, r2);
    }

    #[test]
    fn to_rope_03() {
        let r1 = Rope::from_str(TEXT);
        let s = r1.slice(13..89);
        let r2 = s.to_rope();

        assert_eq!(s, r2);
    }

    #[test]
    fn to_rope_04() {
        let r1 = Rope::from_str(TEXT);
        let s = r1.slice(13..41);
        let r2 = s.to_rope();

        assert_eq!(s, r2);
    }

    // Iterator tests are in the iter module
}

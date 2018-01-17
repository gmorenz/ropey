//! Facilities for customizing grapheme cluster segmentation.
//!
//! (**Note:** the functionality in this module is niche.  Unless you
//! have a specific use-case you can safely ignore it and just use
//! the default `Rope` configuration.)
//!
//! Although the default grapheme cluster segmentation in Ropey works
//! for the vast majority of use-cases, there are some niche cases that
//! may require a custom grapheme cluster definition.  This module
//! allows such customization through the
//! [`GraphemeSegmenter`](trait.GraphemeSegmenter.html) trait.
//!
//! For example, if you want a grapheme cluster definition that considers
//! the "ch" pair and _only_ the "ch" pair to be a grapheme cluster, you
//! can implement it like this:
//!
//! ```
//! use ropey::segmentation::GraphemeSegmenter;
//!
//! #[derive(Debug, Copy, Clone)]
//! struct ChSegmenter {}
//!
//! impl GraphemeSegmenter for ChSegmenter {
//!     fn seam_is_break(left: &str, right: &str) -> bool {
//!         let left_ends_in_c
//!             = *left.as_bytes().last().unwrap() == 'c' as u8;
//!         let right_starts_with_h
//!             = right.as_bytes()[0] == 'h' as u8;
//!         !(left_ends_in_c && right_starts_with_h)
//!     }
//! }
//! ```
//!
//! (Note: types implementing `GraphemeSegmenter` are never actually
//! instantiated by Ropey.  They're just a way to statically pass the needed
//! functions to `Rope` and ensure that grapheme-incompatible `Rope`s aren't
//! mixed in incorrect ways.)
//!
//! To create a `Rope` that uses our "ch" grapheme segmenter we can use one
//! of the `*_with_segmenter()` methods on `Rope` or `RopeBuilder`.  For
//! example:
//!
//! ```
//! # use ropey::segmentation::GraphemeSegmenter;
//! #
//! # #[derive(Debug, Copy, Clone)]
//! # struct ChSegmenter {}
//! #
//! # impl GraphemeSegmenter for ChSegmenter {
//! #     fn seam_is_break(left: &str, right: &str) -> bool {
//! #         let left_ends_in_c = *left.as_bytes().last().unwrap() == 'c' as u8;
//! #         let right_starts_with_h = right.as_bytes()[0] == 'h' as u8;
//! #         !(left_ends_in_c && right_starts_with_h)
//! #     }
//! # }
//! #
//! use ropey::Rope;
//!
//! // Create a rope with our custom "ch" segmenter.
//! let mut rope = Rope::<ChSegmenter>::from_str_with_segmenter("Hi chap!");
//!
//! // Verify that it works.
//! let mut itr = rope.graphemes();
//! assert_eq!(itr.next(), Some("H"));
//! assert_eq!(itr.next(), Some("i"));
//! assert_eq!(itr.next(), Some(" "));
//! assert_eq!(itr.next(), Some("ch"));  // <- Grapheme!
//! assert_eq!(itr.next(), Some("a"));
//! assert_eq!(itr.next(), Some("p"));
//! assert_eq!(itr.next(), Some("!"));
//! assert_eq!(itr.next(), None);
//! ```
//!
//! If rather than completely replacing the default grapheme segmenter you
//! instead want to add more graphemes on top of it, you can this (using
//! "ch" as an example again):
//!
//! ```
//! use ropey::segmentation::{GraphemeSegmenter, DefaultSegmenter};
//!
//! #[derive(Debug, Copy, Clone)]
//! struct ChSegmenter {}
//!
//! impl GraphemeSegmenter for ChSegmenter {
//!     fn seam_is_break(left: &str, right: &str) -> bool {
//!         let default
//!             = DefaultSegmenter::seam_is_break(left, right);
//!         let left_ends_in_c
//!             = *left.as_bytes().last().unwrap() == 'c' as u8;
//!         let right_starts_with_h
//!             = right.as_bytes()[0] == 'h' as u8;
//!
//!         default && !(left_ends_in_c && right_starts_with_h)
//!     }
//! }
//! ```

use std::fmt::Debug;
use std::marker::PhantomData;

use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

/// Trait for implementing grapheme segmentation strategies for [`Rope`](../struct.Rope.html).
///
/// See the documentation of the [`segmentation`](index.html) module for details.
pub trait GraphemeSegmenter: Debug + Copy + Clone {
    /// Returns true if the boundary between the two given strings is a
    /// grapheme cluster boundary, and false otherwise.
    ///
    /// Ropey only calls this with valid and non-empty left and right
    /// strings.
    fn seam_is_break(left: &str, right: &str) -> bool;

    /// Returns whether the given byte index in the given string is a
    /// grapheme cluster boundary.
    ///
    /// Ropey only calls this on valid utf8 code point boundaries, and never
    /// on the start or end of the string.
    ///
    /// This function's default implementation simply calls
    /// `seam_is_break()`, passing it the text split at `char_idx`.  If you
    /// have a more efficient approach, go ahead and implement this manually.
    fn is_break(byte_idx: usize, text: &str) -> bool {
        Self::seam_is_break(&text[..byte_idx], &text[byte_idx..])
    }
}

/// Additional functions for GraphemeSegmenters.
pub(crate) trait SegmenterUtils: GraphemeSegmenter {
    /// Makes sure that special cases are handled correctly.
    #[inline]
    fn is_break_checked(byte_idx: usize, text: &str) -> bool {
        if !text.is_char_boundary(byte_idx) {
            false
        } else if byte_idx == 0 || byte_idx == text.len() {
            true
        } else {
            Self::is_break(byte_idx, text)
        }
    }

    /// Makes sure that special cases are handled correctly.
    #[inline]
    fn seam_is_break_checked(left: &str, right: &str) -> bool {
        debug_assert!(!left.is_empty() && !right.is_empty());
        Self::seam_is_break(left, right)
    }

    /// Returns the segment break before (but not including) the given byte
    /// boundary.
    ///
    /// This will return back the passed byte boundary if it is at the start
    /// of the string.
    #[inline]
    fn prev_break(byte_idx: usize, text: &str) -> usize {
        // Bounds check
        debug_assert!(byte_idx <= text.len());

        let mut boundary_idx = byte_idx;
        while boundary_idx > 0 {
            // Find previous codepoint boundary
            boundary_idx -= 1;
            while !text.is_char_boundary(boundary_idx) {
                boundary_idx -= 1;
            }

            // Check if it's a segment break
            if Self::is_break_checked(boundary_idx, text) {
                break;
            }
        }

        boundary_idx
    }

    /// Returns the segment break after (but not including) the given byte
    /// boundary.
    ///
    /// This will return back the passed byte boundary if it is at the end of
    /// the string.
    #[inline]
    fn next_break(byte_idx: usize, text: &str) -> usize {
        // Bounds check
        debug_assert!(byte_idx <= text.len());

        let mut boundary_idx = byte_idx;
        while boundary_idx < text.len() {
            // Find next codepoint boundary
            boundary_idx += 1;
            while !text.is_char_boundary(boundary_idx) {
                boundary_idx += 1;
            }

            // Check if it's a segment break
            if Self::is_break_checked(boundary_idx, text) {
                break;
            }
        }

        boundary_idx
    }

    /// Finds the segment break nearest to the given byte that is not the
    /// left or right edge of the text.
    ///
    /// There is only one circumstance where the left or right edge will be
    /// returned: if the entire text is a single unbroken segment, then the
    /// right edge of the text is returned.
    #[inline]
    fn nearest_internal_break(byte_idx: usize, text: &str) -> usize {
        // Bounds check
        debug_assert!(byte_idx <= text.len());

        // Find codepoint boundary
        let mut boundary_idx = byte_idx;
        while !text.is_char_boundary(boundary_idx) {
            boundary_idx -= 1;
        }

        // Find the two nearest segment boundaries
        let left = if Self::is_break_checked(boundary_idx, text) && boundary_idx != text.len() {
            boundary_idx
        } else {
            Self::prev_break(boundary_idx, text)
        };
        let right = Self::next_break(boundary_idx, text);

        // Otherwise, return the closest of left and right that isn't the
        // start or end of the string
        if left == 0 || (right != text.len() && (byte_idx - left) >= (right - byte_idx)) {
            return right;
        } else {
            return left;
        }
    }

    #[inline]
    fn find_good_split(byte_idx: usize, text: &str, bias_left: bool) -> usize {
        if Self::is_break_checked(byte_idx, text) {
            byte_idx
        } else {
            let prev = Self::prev_break(byte_idx, text);
            let next = Self::next_break(byte_idx, text);
            if bias_left {
                if prev > 0 {
                    prev
                } else {
                    next
                }
            } else {
                if next < text.len() {
                    next
                } else {
                    prev
                }
            }
        }
    }
}

impl<T: GraphemeSegmenter> SegmenterUtils for T {}

//===========================================================================

/// Internal-only segmenter that takes another segmenter and adds on top of
/// its segmentation that CRLF should never be broken.
/// Used by Ropey to ensure that CRLF is never broken regardless of the
/// segmenter passed.
#[derive(Debug, Copy, Clone)]
pub(crate) struct CRLFSegmenter<Seg: GraphemeSegmenter> {
    _seg: PhantomData<Seg>,
}

impl<S: GraphemeSegmenter> GraphemeSegmenter for CRLFSegmenter<S> {
    #[inline]
    fn is_break(byte_idx: usize, text: &str) -> bool {
        debug_assert!(byte_idx <= text.len());

        let bytes = text.as_bytes();
        let crlf_break = (bytes[byte_idx - 1] != 0x0D) | (bytes[byte_idx] != 0x0A);
        crlf_break && S::is_break(byte_idx, text)
    }

    #[inline]
    fn seam_is_break(left: &str, right: &str) -> bool {
        let crlf_break = (left.as_bytes()[left.len() - 1] != 0x0D) | (right.as_bytes()[0] != 0x0A);
        crlf_break && S::seam_is_break(left, right)
    }
}

//===========================================================================

/// Ropey's default grapheme segmenter.
///
/// Uses the extended grapheme cluster rules specified in
/// [Unicode Standard Annex #29](https://www.unicode.org/reports/tr29/)
#[derive(Debug, Copy, Clone)]
pub struct DefaultSegmenter {}

impl GraphemeSegmenter for DefaultSegmenter {
    #[inline]
    fn is_break(byte_idx: usize, text: &str) -> bool {
        GraphemeCursor::new(byte_idx, text.len(), true)
            .is_boundary(text, 0)
            .unwrap()
    }

    #[inline]
    fn seam_is_break(left: &str, right: &str) -> bool {
        let tot_len = left.len() + right.len();
        let mut gc = GraphemeCursor::new(left.len(), tot_len, true);

        gc.next_boundary(right, left.len()).unwrap();
        let prev = {
            match gc.prev_boundary(right, left.len()) {
                Ok(pos) => pos,
                Err(GraphemeIncomplete::PrevChunk) => gc.prev_boundary(left, 0).unwrap(),
                _ => unreachable!(),
            }
        };

        if let Some(a) = prev {
            if a == left.len() {
                return true;
            }
        }

        return false;
    }
}

/// Grapheme segmenter that ignores graphemes completely and treats each
/// code point as an individual segment.
#[derive(Debug, Copy, Clone)]
pub struct NullSegmenter {}

impl GraphemeSegmenter for NullSegmenter {
    #[inline]
    fn is_break(_byte_idx: usize, _text: &str) -> bool {
        true
    }

    #[inline]
    fn seam_is_break(_left: &str, _right: &str) -> bool {
        true
    }
}

//===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    type MSeg = CRLFSegmenter<DefaultSegmenter>;

    #[test]
    fn crlf_segmenter_01() {
        let text = "Hello world!\r\nHow's it going?";

        assert!(CRLFSegmenter::<NullSegmenter>::is_break_checked(0, ""));
        assert!(CRLFSegmenter::<NullSegmenter>::is_break_checked(0, text));
        assert!(CRLFSegmenter::<NullSegmenter>::is_break_checked(12, text));
        assert!(!CRLFSegmenter::<NullSegmenter>::is_break_checked(13, text));
        assert!(CRLFSegmenter::<NullSegmenter>::is_break_checked(14, text));
        assert!(CRLFSegmenter::<NullSegmenter>::is_break_checked(19, text));
    }

    #[test]
    fn crlf_segmenter_02() {
        let l = "Hello world!\r";
        let r = "\nHow's it going?";

        assert!(!CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(l, r));
        assert!(!CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(
            l,
            "\n"
        ));
        assert!(!CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(
            "\r",
            r
        ));
        assert!(!CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(
            "\r",
            "\n"
        ));
        assert!(CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(r, l));
        assert!(CRLFSegmenter::<NullSegmenter>::seam_is_break_checked(
            "\n",
            "\r"
        ));
    }

    #[test]
    fn nearest_internal_break_01() {
        let text = "Hello world!";
        assert_eq!(1, MSeg::nearest_internal_break(0, text));
        assert_eq!(6, MSeg::nearest_internal_break(6, text));
        assert_eq!(11, MSeg::nearest_internal_break(12, text));
    }

    #[test]
    fn nearest_internal_break_02() {
        let text = "Hello\r\n world!";
        assert_eq!(5, MSeg::nearest_internal_break(5, text));
        assert_eq!(7, MSeg::nearest_internal_break(6, text));
        assert_eq!(7, MSeg::nearest_internal_break(7, text));
    }

    #[test]
    fn nearest_internal_break_03() {
        let text = "\r\nHello world!\r\n";
        assert_eq!(2, MSeg::nearest_internal_break(0, text));
        assert_eq!(2, MSeg::nearest_internal_break(1, text));
        assert_eq!(2, MSeg::nearest_internal_break(2, text));
        assert_eq!(14, MSeg::nearest_internal_break(14, text));
        assert_eq!(14, MSeg::nearest_internal_break(15, text));
        assert_eq!(14, MSeg::nearest_internal_break(16, text));
    }

    #[test]
    fn nearest_internal_break_04() {
        let text = "\r\n";
        assert_eq!(2, MSeg::nearest_internal_break(0, text));
        assert_eq!(2, MSeg::nearest_internal_break(1, text));
        assert_eq!(2, MSeg::nearest_internal_break(2, text));
    }

    #[test]
    fn is_break_01() {
        let text = "\n\r\n\r\n\r\n\r\n\r\n\r";

        assert!(MSeg::is_break_checked(0, text));
        assert!(MSeg::is_break_checked(12, text));
        assert!(MSeg::is_break_checked(3, text));
        assert!(!MSeg::is_break_checked(6, text));
    }

    #[test]
    fn seam_is_break_01() {
        let text1 = "\r\n\r\n\r\n";
        let text2 = "\r\n\r\n";

        assert!(MSeg::seam_is_break(text1, text2));
    }

    #[test]
    fn seam_is_break_02() {
        let text1 = "\r\n\r\n\r";
        let text2 = "\n\r\n\r\n";

        assert!(!MSeg::seam_is_break(text1, text2));
    }
}
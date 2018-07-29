var searchIndex = {};
searchIndex["ropey"] = {"doc":"Ropey is a utf8 text rope library, designed to be the backing text buffer for applications such as text editors.  Ropey is fast, Unicode-safe, has low memory overhead, and can handle huge texts and memory-incoherent edits without trouble.","items":[[3,"Rope","ropey","A utf8 text rope.",null,null],[3,"RopeBuilder","","An efficient incremental `Rope` builder.",null,null],[3,"RopeSlice","","An immutable view into part of a `Rope`.",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"rope"}}],[11,"new","","Creates an empty `Rope`.",0,{"inputs":[],"output":{"name":"self"}}],[11,"from_str","","Creates a `Rope` from a string slice.",0,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from_reader","","Creates a `Rope` from the output of a reader.",0,{"inputs":[{"name":"t"}],"output":{"name":"result"}}],[11,"write_to","","Writes the contents of the `Rope` to a writer.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"result"}}],[11,"len_bytes","","Total number of bytes in the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len_chars","","Total number of chars in the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len_lines","","Total number of lines in the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"capacity","","Total size of the `Rope`'s text buffer space, in bytes.",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"shrink_to_fit","","Shrinks the `Rope`'s capacity to the minimum possible.",0,{"inputs":[{"name":"self"}],"output":null}],[11,"insert","","Inserts `text` at char index `char_idx`.",0,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"str"}],"output":null}],[11,"insert_char","","Inserts a single char `ch` at char index `char_idx`.",0,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"char"}],"output":null}],[11,"remove","","Removes the text in the given char index range.",0,{"inputs":[{"name":"self"},{"name":"r"}],"output":null}],[11,"split_off","","Splits the `Rope` at `char_idx`, returning the right part of the split.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"self"}}],[11,"append","","Appends a `Rope` to the end of this one, consuming the other `Rope`.",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":null}],[11,"byte_to_char","","Returns the char index of the given byte.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"byte_to_line","","Returns the line index of the given byte.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char_to_byte","","Returns the byte index of the given char.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char_to_line","","Returns the line index of the given char.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"line_to_byte","","Returns the byte index of the start of the given line.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"line_to_char","","Returns the char index of the start of the given line.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char","","Returns the char at `char_idx`.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"char"}}],[11,"line","","Returns the line at `line_idx`.",0,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"ropeslice"}}],[11,"chunk_at_byte","","Returns the chunk containing the given byte index.",0,null],[11,"chunk_at_char","","Returns the chunk containing the given char index.",0,null],[11,"chunk_at_line_break","","Returns the chunk containing the given line break.",0,null],[11,"slice","","Gets an immutable slice of the `Rope`.",0,{"inputs":[{"name":"self"},{"name":"r"}],"output":{"name":"ropeslice"}}],[11,"bytes","","Creates an iterator over the bytes of the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"bytes"}}],[11,"chars","","Creates an iterator over the chars of the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"chars"}}],[11,"lines","","Creates an iterator over the lines of the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"lines"}}],[11,"chunks","","Creates an iterator over the chunks of the `Rope`.",0,{"inputs":[{"name":"self"}],"output":{"name":"chunks"}}],[11,"to_string","","Returns the entire text of the `Rope` as a newly allocated String.",0,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"self"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"rope"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"cow"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"ropebuilder"}}],[11,"new","","Creates a new RopeBuilder, ready for input.",1,{"inputs":[],"output":{"name":"self"}}],[11,"append","","Appends `chunk` to the end of the in-progress `Rope`.",1,{"inputs":[{"name":"self"},{"name":"str"}],"output":null}],[11,"finish","","Finishes the build, and returns the `Rope`.",1,{"inputs":[{"name":"self"}],"output":{"name":"rope"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"ropeslice"}}],[11,"len_bytes","","Total number of bytes in the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len_chars","","Total number of chars in the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len_lines","","Total number of lines in the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"byte_to_char","","Returns the char index of the given byte.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"byte_to_line","","Returns the line index of the given byte.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char_to_byte","","Returns the byte index of the given char.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char_to_line","","Returns the line index of the given char.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"line_to_byte","","Returns the byte index of the start of the given line.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"line_to_char","","Returns the char index of the start of the given line.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"char","","Returns the char at `char_idx`.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"char"}}],[11,"line","","Returns the line at `line_idx`.",2,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"ropeslice"}}],[11,"chunk_at_byte","","Returns the chunk containing the given byte index.",2,null],[11,"chunk_at_char","","Returns the chunk containing the given char index.",2,null],[11,"chunk_at_line_break","","Returns the chunk containing the given line break.",2,null],[11,"as_str","","Returns the entire contents of the `RopeSlice` as a `&str` if possible.",2,{"inputs":[{"name":"self"}],"output":{"generics":["str"],"name":"option"}}],[11,"from_str","","Creates a `RopeSlice` directly from a string slice.",2,{"inputs":[{"name":"str"}],"output":{"name":"ropeslice"}}],[11,"slice","","Returns a sub-slice of the `RopeSlice` in the given char index range.",2,{"inputs":[{"name":"self"},{"name":"r"}],"output":{"name":"self"}}],[11,"bytes","","Creates an iterator over the bytes of the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"bytes"}}],[11,"chars","","Creates an iterator over the chars of the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"chars"}}],[11,"lines","","Creates an iterator over the lines of the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"lines"}}],[11,"chunks","","Creates an iterator over the chunks of the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"chunks"}}],[11,"to_string","","Returns the entire text of the `RopeSlice` as a newly allocated `String`.",2,{"inputs":[{"name":"self"}],"output":{"name":"string"}}],[11,"to_rope","","Creates a new `Rope` from the contents of the `RopeSlice`.",2,{"inputs":[{"name":"self"}],"output":{"name":"rope"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"ropeslice"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"cow"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"rope"}],"output":{"name":"bool"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"ropeslice"}],"output":{"name":"bool"}}],[0,"iter","","Iterators over a `Rope`'s data.",null,null],[3,"Bytes","ropey::iter","An iterator over a `Rope`'s bytes.",null,null],[3,"Chars","","An iterator over a `Rope`'s chars.",null,null],[3,"Lines","","An iterator over a `Rope`'s lines.",null,null],[3,"Chunks","","An iterator over a `Rope`'s contiguous `str` chunks.",null,null],[11,"next","","",3,{"inputs":[{"name":"self"}],"output":{"generics":["u8"],"name":"option"}}],[11,"next","","",4,{"inputs":[{"name":"self"}],"output":{"generics":["char"],"name":"option"}}],[11,"next","","",5,{"inputs":[{"name":"self"}],"output":{"generics":["ropeslice"],"name":"option"}}],[11,"next","","",6,{"inputs":[{"name":"self"}],"output":{"generics":["str"],"name":"option"}}],[0,"str_utils","ropey","Utility functions for utf8 string slices.",null,null],[5,"byte_to_char_idx","ropey::str_utils","Converts from byte-index to char-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"byte_to_line_idx","","Converts from byte-index to line-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"char_to_byte_idx","","Converts from char-index to byte-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"char_to_line_idx","","Converts from char-index to line-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"line_to_byte_idx","","Converts from line-index to byte-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"line_to_char_idx","","Converts from line-index to char-index in a string slice.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}]],"paths":[[3,"Rope"],[3,"RopeBuilder"],[3,"RopeSlice"],[3,"Bytes"],[3,"Chars"],[3,"Lines"],[3,"Chunks"]]};
searchIndex["smallvec"] = {"doc":"Small vectors in various sizes. These store a certain number of elements inline, and fall back to the heap for larger allocations.  This can be a useful optimization for improving cache locality and reducing allocator traffic for workloads that fit within the inline buffer.","items":[[3,"Drain","smallvec","An iterator that removes the items from a `SmallVec` and yields them by value.",null,null],[3,"SmallVec","","A `Vec`-like container that can store a small number of elements inline.",null,null],[3,"IntoIter","","An iterator that consumes a `SmallVec` and yields its items by value.",null,null],[8,"VecLike","","Common operations implemented by both `Vec` and `SmallVec`.",null,null],[10,"push","","Append an element to the vector.",0,{"inputs":[{"name":"self"},{"name":"t"}],"output":null}],[8,"ExtendFromSlice","","Trait to be implemented by a collection that can be extended from a slice",null,null],[10,"extend_from_slice","","Extends a collection from a slice of its element type",1,null],[8,"Array","","Types that can be used as the backing store for a SmallVec",null,null],[16,"Item","","The type of the array's elements.",2,null],[10,"size","","Returns the number of items the array can hold.",2,{"inputs":[],"output":{"name":"usize"}}],[10,"ptr","","Returns a pointer to the first element of the array.",2,null],[10,"ptr_mut","","Returns a mutable pointer to the first element of the array.",2,null],[11,"next","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",3,null],[11,"next_back","","",3,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"drop","","",3,{"inputs":[{"name":"self"}],"output":null}],[11,"new","","Construct an empty vector",4,{"inputs":[],"output":{"name":"smallvec"}}],[11,"with_capacity","","Construct an empty vector with enough capacity pre-allocated to store at least `n` elements.",4,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"from_vec","","Construct a new `SmallVec` from a `Vec<A::Item>`.",4,{"inputs":[{"name":"vec"}],"output":{"name":"smallvec"}}],[11,"from_buf","","Constructs a new `SmallVec` on the stack from an `A` without copying elements.",4,{"inputs":[{"name":"a"}],"output":{"name":"smallvec"}}],[11,"set_len","","Sets the length of a vector.",4,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"inline_size","","The maximum number of elements this vector can hold inline",4,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"len","","The number of elements stored in the vector",4,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"is_empty","","Returns `true` if the vector is empty",4,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"capacity","","The number of items the vector can hold without reallocating",4,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"spilled","","Returns `true` if the data has spilled into a separate heap-allocated buffer.",4,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"drain","","Empty the vector and return an iterator over its former contents.",4,{"inputs":[{"name":"self"}],"output":{"name":"drain"}}],[11,"push","","Append an item to the vector.",4,null],[11,"pop","","Remove an item from the end of the vector and return it, or None if empty.",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"grow","","Re-allocate to set the capacity to `max(new_cap, inline_size())`.",4,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"reserve","","Reserve capacity for `additional` more elements to be inserted.",4,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"reserve_exact","","Reserve the minumum capacity for `additional` more elements to be inserted.",4,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"shrink_to_fit","","Shrink the capacity of the vector as much as possible.",4,{"inputs":[{"name":"self"}],"output":null}],[11,"truncate","","Shorten the vector, keeping the first `len` elements and dropping the rest.",4,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"as_slice","","Extracts a slice containing the entire vector.",4,null],[11,"as_mut_slice","","Extracts a mutable slice of the entire vector.",4,null],[11,"swap_remove","","Remove the element at position `index`, replacing it with the last element.",4,null],[11,"clear","","Remove all elements from the vector.",4,{"inputs":[{"name":"self"}],"output":null}],[11,"remove","","Remove and return the element at position `index`, shifting all elements after it to the left.",4,null],[11,"insert","","Insert an element at position `index`, shifting all elements after it to the right.",4,null],[11,"insert_many","","Insert multiple elements at position `index`, shifting all following elements toward the back.",4,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"i"}],"output":null}],[11,"into_vec","","Convert a SmallVec to a Vec, without reallocating if the SmallVec has already spilled onto the heap.",4,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"retain","","Retains only the elements specified by the predicate.",4,{"inputs":[{"name":"self"},{"name":"f"}],"output":null}],[11,"dedup","","Removes consecutive duplicate elements.",4,{"inputs":[{"name":"self"}],"output":null}],[11,"dedup_by","","Removes consecutive duplicate elements using the given equality relation.",4,{"inputs":[{"name":"self"},{"name":"f"}],"output":null}],[11,"dedup_by_key","","Removes consecutive elements that map to the same key.",4,{"inputs":[{"name":"self"},{"name":"f"}],"output":null}],[11,"from_slice","","Copy the elements from a slice into a new `SmallVec`.",4,null],[11,"insert_from_slice","","Copy elements from a slice into the vector at position `index`, shifting any following elements toward the back.",4,null],[11,"extend_from_slice","","Copy elements from a slice and append them to the vector.",4,null],[11,"resize","","Resizes the vector so that its length is equal to `len`.",4,null],[11,"from_elem","","Creates a `SmallVec` with `n` copies of `elem`. ``` use smallvec::SmallVec;",4,null],[11,"deref","","",4,null],[11,"deref_mut","","",4,null],[11,"as_ref","","",4,null],[11,"as_mut","","",4,null],[11,"borrow","","",4,null],[11,"borrow_mut","","",4,null],[11,"write","","",4,null],[11,"write_all","","",4,null],[11,"flush","","",4,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"from","","",4,null],[11,"from","","",4,{"inputs":[{"name":"vec"}],"output":{"name":"smallvec"}}],[11,"from","","",4,{"inputs":[{"name":"a"}],"output":{"name":"smallvec"}}],[11,"index","","",4,null],[11,"index_mut","","",4,null],[11,"index","","",4,null],[11,"index_mut","","",4,null],[11,"index","","",4,null],[11,"index_mut","","",4,null],[11,"index","","",4,null],[11,"index_mut","","",4,null],[11,"index","","",4,null],[11,"index_mut","","",4,null],[11,"extend_from_slice","","",4,null],[11,"push","","",4,null],[11,"from_iter","","",4,{"inputs":[{"name":"i"}],"output":{"name":"smallvec"}}],[11,"extend","","",4,{"inputs":[{"name":"self"},{"name":"i"}],"output":null}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","",4,{"inputs":[],"output":{"name":"smallvec"}}],[11,"drop","","",4,{"inputs":[{"name":"self"}],"output":null}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"smallvec"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"smallvec"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"smallvec"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",4,{"inputs":[{"name":"self"},{"name":"smallvec"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"cmp","","",4,{"inputs":[{"name":"self"},{"name":"smallvec"}],"output":{"name":"ordering"}}],[11,"hash","","",4,{"inputs":[{"name":"self"},{"name":"h"}],"output":null}],[11,"drop","","",5,{"inputs":[{"name":"self"}],"output":null}],[11,"next","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"size_hint","","",5,null],[11,"next_back","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"into_iter","","",4,null],[14,"smallvec","","Creates a [`SmallVec`] containing the arguments.",null,null]],"paths":[[8,"VecLike"],[8,"ExtendFromSlice"],[8,"Array"],[3,"Drain"],[3,"SmallVec"],[3,"IntoIter"]]};
searchIndex["unreachable"] = {"doc":"unreachable","items":[[5,"unreachable","unreachable","Hint to the optimizer that any code path which calls this function is statically unreachable and can be removed.",null,null],[8,"UncheckedOptionExt","","An extension trait for `Option<T>` providing unchecked unwrapping methods.",null,null],[10,"unchecked_unwrap","","Get the value out of this Option without checking for None.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[10,"unchecked_unwrap_none","","Assert that this Option is a None to the optimizer.",0,{"inputs":[{"name":"self"}],"output":null}],[8,"UncheckedResultExt","","An extension trait for `Result<T, E>` providing unchecked unwrapping methods.",null,null],[10,"unchecked_unwrap_ok","","Get the value out of this Result without checking for Err.",1,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[10,"unchecked_unwrap_err","","Get the error out of this Result without checking for Ok.",1,{"inputs":[{"name":"self"}],"output":{"name":"e"}}]],"paths":[[8,"UncheckedOptionExt"],[8,"UncheckedResultExt"]]};
searchIndex["void"] = {"doc":"Void","items":[[4,"Void","void","The empty type for cases which can't occur.",null,null],[5,"unreachable","","A safe version of `intrinsincs::unreachable`.",null,null],[8,"ResultVoidExt","","Extensions to `Result<T, Void>`",null,null],[10,"void_unwrap","","Get the value out of a wrapper.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[8,"ResultVoidErrExt","","Extensions to `Result<Void, E>`",null,null],[10,"void_unwrap_err","","Get the error out of a wrapper.",1,{"inputs":[{"name":"self"}],"output":{"name":"e"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"void"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["ordering"],"name":"option"}}]],"paths":[[8,"ResultVoidExt"],[8,"ResultVoidErrExt"],[4,"Void"]]};
initSearch(searchIndex);

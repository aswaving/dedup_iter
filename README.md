This crate provides a couple of iterator adapters for deduplication from elements of a source iterator, inspired by the dedup methods in `Vec`.

# `dedup`
The `DedupIteratorAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator.
The `dedup` trait method of `DedupIteratorAdapter` returns a `DedupIterator`.
## Example
 ```rust
use dedup_iter::DedupIteratorAdapter;

assert_eq!("abcdefe", "aabbccdddeeeeffffeee".chars().dedup().collect::<String>());
 ```

# `dedup_by`
The `DedupByAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator
using a function to determine equality.
The `dedup_by` trait method returns a `DedupBy` iterator struct.
## Examples
```rust
use std::ascii::AsciiExt;
use dedup_iter::DedupByAdapter;

assert_eq!(
    "This string had way too much redundant whitespace!",
    "This  string   had      way too     much redundant \n whitespace!".chars()
     .dedup_by(|a, b| a.is_whitespace() && b.is_whitespace() )
     .collect::<String>()
);
```

# `dedup_by_key`
The `DedupByKeyAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator
using a key to determine equality.
The `dedup_by_key` trait method returns a `DedupByKey` iterator struct.
## Examples
```rust
use dedup_iter::DedupByKeyAdapter;

assert_eq!(
    "abcdefe",
    "aabbccdddeeeeffffeee".chars()
     .dedup_by_key(|a| *a as usize)
     .collect::<String>()
);
```

//! This crate provides a couple of iterator adapters for deduplication of elements from a source iterator, inspired by the dedup methods
//! in `Vec`.
//!
//! # `dedup`
//! The `DedupAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator.
//! The `dedup` trait method returns a `Dedup` iterator struct.
//! ## Example
//! ```rust
//! use dedup_iter::DedupAdapter;
//!
//! assert_eq!("abcdefe", "aabbccdddeeeeffffeee".chars().dedup().collect::<String>());
//! ```
//!
//! # `dedup_by`
//! The `DedupByAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator
//! using a function to determine equality.
//! The `dedup_by` trait method returns a `DedupBy` iterator struct.
//! ## Examples
//! ```rust
//! use std::ascii::AsciiExt;
//! use dedup_iter::DedupByAdapter;
//!
//! assert_eq!(
//!        "This string had way too much redundant whitespace!",
//!        "This  string   had      way too     much redundant \n whitespace!".chars()
//!         .dedup_by(|a, b| a.is_whitespace() && b.is_whitespace() )
//!         .collect::<String>()
//!        );
//! ```

//! # `dedup_by_key`
//! The `DedupByKeyAdapter` is an iterator adapter that removes consecutive repeated elements from the source iterator
//! using a key to determine equality.
//! The `dedup_by_key` trait method returns a `DedupByKey` iterator struct.
//! ## Examples
//! ```rust
//! use dedup_iter::DedupByKeyAdapter;
//!
//! assert_eq!(
//!        "abcdefe",
//!        "aabbccdddeeeeffffeee".chars()
//!         .dedup_by_key(|a| *a as usize)
//!         .collect::<String>()
//!        );
//! ```

/// An iterator that removes elements that are the same as previous one.
///
/// This struct is created by the `dedup` method of trait `DedupAdapter`, implemented on Iterator.
/// To use the `dedup` method, `use dedup_iter::DedupAdapter`.
pub struct Dedup<I, T> {
    iter: I,
    current_item: Option<T>,
}

/// An iterator that removes elements that are the same as previous one, according the provided function.
///
/// This struct is created by the `dedup_by` method of trait `DedupByAdapter`, implemented on Iterator.
/// To use the `dedup_by` method, `use dedup_iter::DedupByAdapter`.
pub struct DedupBy<I, F, T> {
    iter: I,
    current_item: Option<T>,
    same_bucket: F,
}

/// An iterator that removes elements that have a key that is the same as the key of previous element.
/// The client provided function computes the key.
///
/// This struct is created by the `dedup_by_key` method of trait `DedupByKeyAdapter`, implemented on Iterator.
/// To use the `dedup_by_key` method, `use dedup_iter::DedupByKeyAdapter`.
pub struct DedupByKey<I, F, K> {
    iter: I,
    current_key: Option<K>,
    key: F,
}

impl<I, T> Iterator for Dedup<I, T>
where
    I: Iterator<Item = T>,
    T: PartialEq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        for x in self.iter.by_ref() {
            let item = Some(x.clone());
            if self.current_item != item {
                self.current_item = item;
                return Some(x);
            }
        }
        None
    }
}

impl<I, F, T> Iterator for DedupBy<I, F, T>
where
    I: Iterator<Item = T>,
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        for x in self.iter.by_ref() {
            let item = Some(x.clone());
            let different = match self.current_item {
                None => true,
                Some(ref current_item) => !(self.same_bucket)(current_item, &x), 
            };
            if different {
                self.current_item = item;
                return Some(x);
            }
        }
        None
    }
}

impl<I, F, K> Iterator for DedupByKey<I, F, K>
where
    I: Iterator,
    F: Fn(&I::Item) -> K,
    K: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        for x in self.iter.by_ref() {
            let key = (self.key)(&x);
            let different = match self.current_key {
                None => true,
                Some(ref current_key) => key != *current_key,
            };
            if different {
                self.current_key = Some(key);
                return Some(x);
            }
        }
        None
    }
}

/// Provides the `dedup` method on `Iterator`s.
pub trait DedupAdapter: Iterator {
    fn dedup(self) -> Dedup<Self, Self::Item>
    where
        Self: Sized,
    {
        Dedup {
            iter: self,
            current_item: None,
        }
    }
}

/// Provides the `dedup_by` method on `Iterator`s.
pub trait DedupByAdapter<F>: Iterator {
    fn dedup_by(self, same_bucket: F) -> DedupBy<Self, F, Self::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item, &Self::Item) -> bool,
    {
        DedupBy {
            iter: self,
            current_item: None,
            same_bucket: same_bucket,
        }

    }
}

/// Provides the `dedup_by_key` method on `Iterator`s.
pub trait DedupByKeyAdapter<F, K>: Iterator {
    fn dedup_by_key(self, key: F) -> DedupByKey<Self, F, K>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
    {
        DedupByKey {
            iter: self,
            current_key: None,
            key: key,
        }

    }
}

impl<I> DedupAdapter for I
where
    I: Iterator,
{
}

impl<I, F> DedupByAdapter<F> for I
where
    I: Iterator,
{
}

impl<I, F, K> DedupByKeyAdapter<F, K> for I
where
    I: Iterator,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicate_chars() {
        let t = "Thiss sisssssling ssstring";
        let v = t.chars().dedup().collect::<String>();
        assert_eq!(&v, "This sisling string");
    }

    #[test]
    fn remove_duplicate_whitespace() {
        let t = "Hierr zzitt ikk    well      goedd!";
        let v = t.chars()
            .dedup_by(|a, b| a.is_whitespace() && b.is_whitespace())
            .collect::<String>();
        assert_eq!(&v, "Hierr zzitt ikk well goedd!");
    }

    #[test]
    fn almost_acronym() {
        let t = "First In, Last Out";
        let v = t.chars()
            .dedup_by_key(|a| a.is_whitespace())
            .collect::<String>();
        assert_eq!(&v, "F I L O");
    }

    #[test]
    fn dedup_iter_empty() {
        let t = Vec::<u8>::new();
        let c = t.iter().dedup().count();
        assert_eq!(c, 0);
    }

    #[test]
    fn dedup_iter_numbers() {
        let t = vec![10, 20, 20, 21, 30, 20];
        let v = t.iter().dedup().collect::<Vec<_>>();
        assert_eq!(v, vec![&10, &20, &21, &30, &20]);
    }

    #[test]
    fn dedup_by_eq() {
        let t = "Hierr zzitt ikk    well      goedd!";
        let v = t.chars().dedup_by(|a, b| *a == *b).collect::<String>();
        assert_eq!(&v, "Hier zit ik wel goed!");
    }

    #[test]
    fn dedup_by_key() {
        #[derive(Debug, PartialEq)]
        struct Person<'a> {
            id: u64,
            name: &'a str,
        };
        let t = vec![
            Person {
                id: 0,
                name: "Bilbo",
            },
            Person {
                id: 0,
                name: "Bilbo",
            },
        ];
        let v = t.iter()
            .dedup_by_key(|person| person.id)
            .collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                &Person {
                    id: 0,
                    name: "Bilbo",
                },
            ]
        );
    }

    #[test]
    fn dedup_by_always_same() {
        let t = "abdefghijklmopqrstuvwxyz";
        let v = t.chars().dedup_by(|_, _| true).collect::<String>();
        assert_eq!(&v, "a");
    }

    #[test]
    fn dedup_by_key_always_same() {
        let t = "abdefghijklmopqrstuvwxyz";
        let v = t.chars().dedup_by_key(|_| 0).collect::<String>();
        assert_eq!(&v, "a");
    }
}

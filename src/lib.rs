//! This struct maintains a fixed length circular Vec.
//! You provide the items for the circular Vec at initialization
//! time and can then never change the amount of items held within.
//! It provides a `next` function that, when it hits the end of the struct,
//! just loops back to the start.
//!
//! The CircularVec allows for in place changes to the items held
//! within using the `next_mut` function, though for most cases
//! just using `next` is fine. To call either of these functions
//! you must have a mutable reference to the CircularVec so that
//! it can increment its internal counter.
//!
//! Notably, CircularVec does not implement `IntoIterator` because it
//! would produce an iterator that never ends, which is not the
//! intended use of `IntoIterator`. Accordingly, the `next` function
//! here does not return the item (`T`), but a reference to it (`&T`), and
//! returns `&T` instead of `Option<&T>` because there will always
//! be an item it can return.
//!
//! Example usage:
//!
//!     let mut cv: CircularVec<String> = ["hello".to_string(), "world".to_string()]
//!         .to_vec()
//!         .into_iter()
//!         .collect();
//!             
//!     assert_eq!(cv.next(), "hello");
//!     assert_eq!(cv.next(), "world");
//!     assert_eq!(cv.next(), "hello");
//!     assert_eq!(cv.next(), "world");

use std::iter::FromIterator;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

/// See crate level documentation.
pub struct CircularVec<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> CircularVec<T> {
    fn new(items: Vec<T>) -> Self {
        CircularVec { items, index: 0 }
    }

    /// Get an immutable reference to the next item in the CircularVec.
    pub fn next(&mut self) -> &T {
        let original_index = self.index;
        self.increment_index();
        &self.items[original_index]
    }

    pub fn skip(&mut self, n: usize) {
        let mut n = n;
        while n > 0 {
            self.increment_index();
            n -= 1;
        }
    }
    /// Get a mutable reference to the next item in the CircularVec.
    pub fn next_mut(&mut self) -> &mut T {
        let original_index = self.index;
        self.increment_index();
        IndexMut::index_mut(&mut *self.items, original_index)
    }

    fn increment_index(&mut self) {
        self.index = (self.index + 1) % self.items.len();
    }
}

impl<T> FromIterator<T> for CircularVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut items = Vec::new();

        for i in iter {
            items.push(i);
        }

        Self::new(items)
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for CircularVec<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&*self.items, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_through() {
        let mut cv: CircularVec<u64> = [50, 60, 70, 80].to_vec().into_iter().collect();
        assert_eq!(cv.next(), &50);
        assert_eq!(cv.next(), &60);
        assert_eq!(cv.next(), &70);
        assert_eq!(cv.next(), &80);
        assert_eq!(cv.next(), &50);

        assert_eq!(cv[0], 50);
    }

    #[test]
    fn loop_through_string() {
        let mut cv: CircularVec<String> = ["hello".to_string(), "world".to_string()]
            .to_vec()
            .into_iter()
            .collect();

        assert_eq!(cv.next(), "hello");
        assert_eq!(cv.next(), "world");
        assert_eq!(cv.next(), "hello");
        assert_eq!(cv.next(), "world");
        assert_eq!(cv.next(), "hello");
        assert_eq!(cv.next(), "world");

        assert_eq!(cv[0], "hello");
    }
}

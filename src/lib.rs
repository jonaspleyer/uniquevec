#![deny(missing_docs)]
#![no_std]
//! This crate offers a [Vec]-like datastructure which only contains unique
//! entries.
//! It is `no_std` by default but necessarily requires an allocator.
//! To access elements inside the vector, we implement the [Deref](core::ops::Deref) trait such that
//! it can be used like a regular [Vec].
//! To modify entries or create new instances, we implement methods which are listed below.
//!
//! ## Create and Modify
//! | Method | Description |
//! |:--- | --- |
//! | [`UniqueVec::new()`](UniqueVec::new) | Creates a new empty [UniqueVec]. |
//! | [`UniqueVec::from_iter(iterator)`](UniqueVec::from_iter) | Creates a new [UniqueVec] from an iterator. |
//! | [`UniqueVec::push(item)`](UniqueVec::push) | Pushes a new entry to the back or returns it if already present. |
//! | [`UniqueVec::clear()`](UniqueVec::clear) | Clears all entries. |
//! | [`UniqueVec::pop()`](UniqueVec::pop) | Removes and returns the last entry. |
//! | [`UniqueVec::extend_from_iter(iterator)`](UniqueVec::extend_from_iter) | Extends elements by the given iterator. Returns duplicates in order. |
//!
//! ## Implemented Traits
//!
//! | Trait | Implemented | Comment |
//! | --- | --- | --- |
//! | [Deref](core::ops::Deref) | ✅ | |
//! | [DerefMut](core::ops::DerefMut) | ❌ | See the ["Create and Modify"](#create-and-modify) table above. |
//! | [Extend] | ✅ |
//! | [From] for [Vec] | ✅ |
//! | [IntoIterator] | ✅ |
//!
//! ## Features
//!
//! - The [serde](https://serde.rs/) feature offers serialization support.

extern crate alloc;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A unique vector containing only non-recurring values but in the correct order.
///
/// ```
/// # use uniquevec::UniqueVec;
/// let mut unique_vec = UniqueVec::new();
/// unique_vec.push(1_usize);
/// unique_vec.push(2_usize);
/// let res = unique_vec.push(1_usize);
/// assert!(res.is_some());
/// assert_eq!(*unique_vec, vec![1, 2]);
/// ```
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug)]
pub struct UniqueVec<T>(Vec<T>);

impl<T> UniqueVec<T> {
    /// Creates an new empty [UniqueVec].
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Construct a new [UniqueVec] from a given vector.
    /// This function will also return the rest which was not inserted into the [UniqueVec].
    ///
    /// ```
    /// # use uniquevec::UniqueVec;
    /// let input = vec![1, 33, 2, 0, 33, 4, 56, 2];
    /// let (unique_vec, rest) = UniqueVec::from_iter(input);
    /// assert_eq!(*unique_vec, vec![1, 33, 2, 0, 4, 56]);
    /// assert_eq!(rest, vec![33, 2]);
    /// ```
    pub fn from_iter(iter: impl IntoIterator<Item = T>) -> (Self, Vec<T>)
    where
        T: PartialEq,
    {
        let mut new_inner = Vec::new();
        let rest = iter
            .into_iter()
            .filter_map(|element| {
                if new_inner.contains(&element) {
                    Some(element)
                } else {
                    new_inner.push(element);
                    None
                }
            })
            .collect();
        (Self(new_inner), rest)
    }

    /// Add an element to the [UniqueVec] if not already present.
    ///
    /// ```
    /// # use uniquevec::UniqueVec;
    /// let mut unique_vec = UniqueVec::new();
    /// assert!(unique_vec.push(1_f64).is_none());
    /// assert!(unique_vec.push(2_f64).is_none());
    /// assert!(unique_vec.push(1_f64).is_some());
    /// assert_eq!(*unique_vec, vec![1_f64, 2_f64]);
    /// ```
    pub fn push(&mut self, element: T) -> Option<T>
    where
        T: PartialEq,
    {
        if self.0.contains(&element) {
            Some(element)
        } else {
            self.0.push(element);
            None
        }
    }

    /// Empties the [UniqueVec] returning all values
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Remove last element from [UniqueVec]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// ```
    /// # use uniquevec::UniqueVec;
    /// let mut unique_vec = UniqueVec::from_iter(vec![3, 1, 19]).0;
    /// let other = unique_vec.extend_from_iter([73, 1843, 19, 3]);
    /// assert_eq!(other, vec![19, 3]);
    /// ```
    pub fn extend_from_iter<S: IntoIterator<Item = T>>(&mut self, iter: S) -> Vec<T>
    where
        T: PartialEq,
    {
        let iter = iter.into_iter();
        let (duplicates, new_entries) =
            core::iter::Iterator::partition::<Vec<T>, _>(iter, |elem| self.contains(elem));
        self.0.extend(new_entries);
        duplicates
    }
}

impl<T> core::iter::Extend<T> for UniqueVec<T>
where
    T: PartialEq,
{
    fn extend<S: IntoIterator<Item = T>>(&mut self, iter: S) {
        self.extend_from_iter(iter);
    }
}

impl<T> core::ops::Deref for UniqueVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Vec<T>> for UniqueVec<T>
where
    T: PartialEq,
{
    fn from(value: Vec<T>) -> Self {
        Self::from_iter(value).0
    }
}

impl<T> IntoIterator for UniqueVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

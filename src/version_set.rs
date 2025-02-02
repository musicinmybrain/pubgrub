// SPDX-License-Identifier: MPL-2.0

//! As its name suggests, the [VersionSet] trait describes sets of versions.
//!
//! One needs to define
//! - the associate type for versions,
//! - two constructors for the empty set and a singleton set,
//! - the complement and intersection set operations,
//! - and a function to evaluate membership of versions.
//!
//! Two functions are automatically derived, thanks to the mathematical properties of sets.
//! You can overwrite those implementations, but we highly recommend that you don't,
//! except if you are confident in a correct implementation that brings much performance gains.
//!
//! It is also extremely important that the `Eq` trait is correctly implemented.
//! In particular, you can only use `#[derive(Eq, PartialEq)]` if `Eq` is strictly equivalent to the
//! structural equality, i.e. if version sets have canonical representations.
//! Such problems may arise if your implementations of `complement()` and `intersection()` do not
//! return canonical representations so be careful there.

use std::fmt::{Debug, Display};

use crate::Ranges;

/// Trait describing sets of versions.
pub trait VersionSet: Debug + Display + Clone + Eq {
    /// Version type associated with the sets manipulated.
    type V: Debug + Display + Clone + Ord;

    // Constructors
    /// Constructor for an empty set containing no version.
    fn empty() -> Self;
    /// Constructor for a set containing exactly one version.
    fn singleton(v: Self::V) -> Self;

    // Operations
    /// Compute the complement of this set.
    fn complement(&self) -> Self;
    /// Compute the intersection with another set.
    fn intersection(&self, other: &Self) -> Self;

    // Membership
    /// Evaluate membership of a version in this set.
    fn contains(&self, v: &Self::V) -> bool;

    // Automatically implemented functions ###########################

    /// Constructor for the set containing all versions.
    /// Automatically implemented as `Self::empty().complement()`.
    fn full() -> Self {
        Self::empty().complement()
    }

    /// Compute the union with another set.
    /// Thanks to set properties, this is automatically implemented as:
    /// `self.complement().intersection(&other.complement()).complement()`
    fn union(&self, other: &Self) -> Self {
        self.complement()
            .intersection(&other.complement())
            .complement()
    }

    /// Whether the range have no overlapping segments.
    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other) == Self::empty()
    }

    /// Whether all range of `self` are contained in `other`.
    fn subset_of(&self, other: &Self) -> bool {
        self == &self.intersection(other)
    }
}

impl<T: Debug + Display + Clone + Eq + Ord> VersionSet for Ranges<T> {
    type V = T;

    fn empty() -> Self {
        Ranges::empty()
    }

    fn singleton(v: Self::V) -> Self {
        Ranges::singleton(v)
    }

    fn complement(&self) -> Self {
        Ranges::complement(self)
    }

    fn intersection(&self, other: &Self) -> Self {
        Ranges::intersection(self, other)
    }

    fn contains(&self, v: &Self::V) -> bool {
        Ranges::contains(self, v)
    }

    fn full() -> Self {
        Ranges::full()
    }

    fn union(&self, other: &Self) -> Self {
        Ranges::union(self, other)
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        Ranges::is_disjoint(self, other)
    }

    fn subset_of(&self, other: &Self) -> bool {
        Ranges::subset_of(self, other)
    }
}

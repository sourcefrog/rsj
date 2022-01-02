// Copyright 2021 Martin Pool

//! Array objects.

use std::fmt;

use ndarray::prelude::*;

use crate::atom::Atom;

/// Arrays potentially have n dimensions, although only 1-dimensional arrays are
/// supported for now.
///
/// Arrays are backed by an ndarray array.
///
/// Arrays contain atoms.
// TODO: Require that arrays contain homogenous atoms.
#[derive(Debug, Clone, PartialEq)]
pub struct Array(Array1<Atom>);

impl Array {
    /// Construct an array by taking ownership of a Vec of Atoms.
    pub fn from_vec(v: Vec<Atom>) -> Array {
        Array(v.into())
    }

    /// Iterate by-reference the atoms in the array.
    pub fn iter_atoms<'a>(&'a self) -> impl Iterator<Item = &Atom> + 'a {
        self.into_iter()
    }

    /// Return the number of _items_ in the array: the cells whose rank is one lower than the rank of the
    /// array.
    ///
    /// Since only 1d arrays are supported at the moment this is just the atoms.
    pub fn number_items(&self) -> usize {
        self.0.len()
    }

    /// Return the shape of the array, as another array.
    pub fn shape(&self) -> Array {
        self.0.shape().iter().map(|&s| Atom::from(s)).collect()
    }
}

/// Iterate by-reference the elements of the array.
impl<'a> IntoIterator for &'a Array {
    type Item = &'a Atom;
    type IntoIter = ndarray::iter::Iter<'a, Atom, Ix1>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Construct an array by collecting things that convert into Atoms.
impl<T> std::iter::FromIterator<T> for Array
where
    Atom: From<T>,
{
    fn from_iter<I>(iter: I) -> Array
    where
        I: IntoIterator<Item = T>,
    {
        Array(iter.into_iter().map(Atom::from).collect())
    }
}

/// Construct an array from an iterator of things that convert into Atoms.
impl<I, T> From<I> for Array
where
    I: IntoIterator<Item = T>,
    Atom: From<T>,
{
    fn from(v: I) -> Array {
        v.into_iter().collect()
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, atom) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", atom)?;
        }
        Ok(())
    }
}

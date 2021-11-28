// Copyright 2021 Martin Pool

//! Array objects.

use std::fmt;

use crate::atom::Atom;

/// Arrays potentially have n dimensions, although only 1-dimensional arrays are implemented now.
///
/// Arrays contain atoms.
// TODO: Require that arrays contain homogenous atoms.
#[derive(Debug, Clone, PartialEq)]
pub struct Array(Vec<Atom>);

impl Array {
    /// Construct an array by taking ownership of a Vec of Atoms.
    pub fn from_vec(v: Vec<Atom>) -> Array {
        Array(v)
    }

    /// Iterate by-reference the atoms in the array.
    pub fn iter_atoms<'a>(&'a self) -> impl Iterator<Item = &Atom> + 'a {
        self.into_iter()
    }
}

/// Iterate by-reference the elements of the array.
impl<'a> IntoIterator for &'a Array {
    type Item = &'a Atom;
    type IntoIter = std::slice::Iter<'a, Atom>;

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

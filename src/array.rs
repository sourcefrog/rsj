// Copyright 2021 Martin Pool

//! Array objects.

use std::fmt;

use num_complex::Complex64;

use crate::atom::{display_complex, Atom};

/// Arrays potentially have n dimensions, although only 1-dimensional arrays are implemented now.
#[derive(Debug, Clone, PartialEq)]
pub struct Array(pub Vec<Complex64>);

impl Array {
    pub fn from_vec(vec: Vec<Complex64>) -> Self {
        Array(vec)
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &n) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            display_complex(n, f)?;
        }
        Ok(())
    }
}

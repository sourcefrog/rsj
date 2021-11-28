// Copyright 2021 Martin Pool

//! Nouns (J objects.)

use std::fmt;

use num_complex::Complex64;

use crate::array::Array;
use crate::atom::Atom;

#[derive(Debug, Clone, PartialEq)]
pub enum Noun {
    Atom(Atom),
    Array(Array),
}

impl Noun {
    pub fn matrix_from_vec(vec: Vec<Complex64>) -> Noun {
        Noun::Array(Array::from_vec(vec))
    }
}

impl From<f64> for Noun {
    fn from(v: f64) -> Noun {
        Noun::Atom(v.into())
    }
}

impl From<Complex64> for Noun {
    fn from(v: Complex64) -> Noun {
        Noun::Atom(v.into())
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Atom(a) => write!(f, "{}", a),
            Noun::Array(m) => write!(f, "{}", m),
        }
    }
}

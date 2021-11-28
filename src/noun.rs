// Copyright 2021 Martin Pool

//! Nouns (J objects.)

use std::fmt;

use num_complex::Complex64;

use crate::atom::{display_complex, Atom};

#[derive(Debug, Clone, PartialEq)]
pub enum Noun {
    Atom(Atom),
    Matrix(Matrix),
}

impl Noun {
    pub fn matrix_from_vec(vec: Vec<Complex64>) -> Noun {
        Noun::Matrix(Matrix::from_vec(vec))
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
            Noun::Matrix(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Complex64>);

impl Matrix {
    pub fn from_vec(vec: Vec<Complex64>) -> Self {
        Matrix(vec)
    }
}

impl fmt::Display for Matrix {
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

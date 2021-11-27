// Copyright 2021 Martin Pool

//! Nouns (J objects.)

use std::fmt::{self, Write};

use num_complex::Complex64;

#[derive(Debug, Clone, PartialEq)]
pub enum Noun {
    // TODO: Maybe a Number should be just an 0-dimensional array, or a 1d array with one element?
    Number(Complex64),
    Matrix(Matrix),
    // TODO: char, ...
}

impl Noun {
    pub fn matrix_from_vec(vec: Vec<Complex64>) -> Noun {
        Noun::Matrix(Matrix::from_vec(vec))
    }
}

impl From<f64> for Noun {
    fn from(v: f64) -> Noun {
        Noun::Number(v.into())
    }
}

impl From<Complex64> for Noun {
    fn from(v: Complex64) -> Noun {
        Noun::Number(v)
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Number(v) => display_number(*v, f),
            Noun::Matrix(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Number(Complex64),
    // TODO: char, ...
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(Vec<Complex64>);

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
            display_number(n, f)?;
        }
        Ok(())
    }
}

fn display_number(n: Complex64, f: &mut fmt::Formatter) -> fmt::Result {
    // TODO: Move to display of the atom?
    let mut s = String::new();
    if n.im == 0.0 {
        display_f64(n.re, f)?;
    } else {
        display_f64(n.re, f)?;
        write!(s, "j")?;
        display_f64(n.im, f)?;
    }
    Ok(())
}

fn display_f64(n: f64, f: &mut fmt::Formatter) -> fmt::Result {
    if n == f64::INFINITY {
        write!(f, "_")
    } else if n == f64::NEG_INFINITY {
        write!(f, "__")
    } else {
        let mut s = format!("{}", n);
        s = s.replace('-', "_");
        write!(f, "{}", s)
    }
}

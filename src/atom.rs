// Copyright 2021 Martin Pool

//! Nouns (J objects.)

use std::fmt::{self, Write};

use num_complex::Complex64;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Complex(Complex64),
    // TODO: char, ...
}

impl From<f64> for Atom {
    fn from(v: f64) -> Self {
        Atom::Complex(v.into())
    }
}

impl From<&Complex64> for Atom {
    fn from(v: &Complex64) -> Self {
        Atom::Complex(*v)
    }
}

impl From<Complex64> for Atom {
    fn from(v: Complex64) -> Self {
        Atom::Complex(v)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Complex(v) => display_complex(*v, f),
        }
    }
}

pub(crate) fn display_complex(n: Complex64, f: &mut fmt::Formatter) -> fmt::Result {
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

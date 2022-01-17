// Copyright 2021, 2022 Martin Pool

//! Nouns (J objects.)

use std::convert::TryInto;
use std::fmt::{self, Write};

use num_complex::Complex64;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Complex(Complex64),
    // TODO: char, ...
}

impl Atom {
    pub fn zero() -> Atom {
        Atom::Complex(0.0.into())
    }

    pub fn to_complex(&self) -> Complex64 {
        match self {
            Atom::Complex(a) => *a,
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Atom::Complex(Complex64 { re, im }) => *re == 0.0 && *im == 0.0,
        }
    }

    /// Return an f64 if this is representable as such.
    pub fn try_to_f64(&self) -> Option<f64> {
        match self {
            Atom::Complex(Complex64 { re, im }) => {
                if *im == 0.0 {
                    Some(*re)
                } else {
                    None
                }
            }
        }
    }
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

impl From<usize> for Atom {
    fn from(v: usize) -> Self {
        let a: u32 = v.try_into().unwrap();
        Atom::Complex(Complex64::new(a.into(), 0.0))
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
        f.write_char('_')
    } else if n == f64::NEG_INFINITY {
        f.write_str("__")
    } else {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(n);
        let s = s.replace('-', "_");
        // Ryu formats 1 as `1.0`, but we don't want that here, especially because
        // we commonly store integer values in f64.
        let s = s.strip_suffix(".0").unwrap_or(&s);
        f.write_str(s)
    }
}

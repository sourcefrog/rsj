// Copyright 2021, 2022 Martin Pool

//! J primitive (built-in) verbs.

// See https://code.jsoftware.com/wiki/Vocabulary/Words#Primitives

use std::borrow::Cow;
use std::fmt;

use bstr::BStr;
use fmt::Formatter;

use crate::array::Array;
use crate::atom::Atom;
use crate::error::{Error, Result};
use crate::noun::Noun;
use crate::verb::Verb;

/// A builtin primitive verb, such as `-` or `<.`.
pub struct Primitive(&'static [u8], Monad, Dyad);

// All implemented primitives.
pub const DOLLAR: Primitive = Primitive(b"$", Monad::Infinite(shape_of), Dyad::Unimplemented);
pub const MINUS: Primitive = Primitive(b"-", Monad::Zero(negate), Dyad::Zero(minus));
pub const MINUS_DOT: Primitive = Primitive(b"-.", Monad::Zero(not), Dyad::Unimplemented);
pub const NUMBER: Primitive = Primitive(b"#", Monad::Infinite(tally), Dyad::Unimplemented);
pub const PLUS: Primitive = Primitive(b"+", Monad::Unimplemented, Dyad::Zero(plus));

pub const PRIMITIVES: &[Primitive] = &[
    DOLLAR,
    MINUS,
    MINUS_DOT,
    NUMBER,
    Primitive(b"%", Monad::Zero(reciprocal), Dyad::Zero(divide)),
    Primitive(b"*", Monad::Zero(signum), Dyad::Zero(times)),
    PLUS,
    Primitive(b"i.", Monad::Infinite(integers), Dyad::Unimplemented),
];

impl Primitive {
    pub fn name(&self) -> &'static BStr {
        self.0.into()
    }

    pub fn by_name<S>(s: &S) -> Result<&'static Primitive>
    where
        S: AsRef<[u8]>,
    {
        let s = s.as_ref();
        for prim in PRIMITIVES {
            if s == prim.name() {
                return Ok(prim);
            }
        }
        Err(Error::Unimplemented("primitive".into()))
    }
}

impl Verb for Primitive {
    fn display(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self.name()))
    }

    fn monad(&self, y: &Noun) -> Result<Noun> {
        self.1.apply(y)
    }

    fn dyad(&self, x: &Noun, y: &Noun) -> Result<Noun> {
        self.2.apply(x, y)
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for Primitive {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Primitive").field("name", &self.0).finish()
    }
}

impl std::cmp::PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// A primitive monad implementation which is applicable at one of several possible ranks.
enum Monad {
    /// A monad that applies per-atom.
    Zero(fn(&Atom) -> Result<Atom>),
    Infinite(fn(&Noun) -> Result<Noun>),
    Unimplemented,
    // TODO: One, Two, ...
}

impl Monad {
    /// Apply this monad to the y noun, at the appropriate rank.
    fn apply(&self, y: &Noun) -> Result<Noun> {
        match self {
            Monad::Zero(f) => match y {
                Noun::Atom(a) => f(a).map(Noun::Atom),
                Noun::Array(array) => Ok(Noun::Array(Array::from_vec(
                    array.iter_atoms().map(f).collect::<Result<Vec<Atom>>>()?,
                ))),
            },
            Monad::Infinite(f) => f(y),
            Monad::Unimplemented => Err(Error::Unimplemented("Monad::Unimplemented".into())),
        }
    }
}

/// A primitive verb applicable at various ranks.
enum Dyad {
    // TODO: Other ranks, and in particular asymmetric ranks. It might need
    // a different representation.
    /// Per atom on both sides (0, 0).
    Zero(fn(&Atom, &Atom) -> Result<Atom>),
    Unimplemented,
}

impl Dyad {
    fn apply(&self, x: &Noun, y: &Noun) -> Result<Noun> {
        // TODO: This code for working out how to apply element-at-a-time etc
        // probably should be generic to all verbs, not only primitives.
        match self {
            Dyad::Zero(f) => match (x, y) {
                (Noun::Atom(ax), Noun::Atom(ay)) => f(ax, ay).map(Noun::from),
                (Noun::Array(ax), Noun::Array(ay)) => {
                    // element-wise
                    // TODO: This is actually too specific: it's OK for the arrays to be
                    // different shapes as long as they "agree":
                    // https://code.jsoftware.com/wiki/Vocabulary/Agreement
                    if ax.shape() == ay.shape() {
                        Ok(Noun::Array(Array::from_vec(
                            ax.iter_atoms()
                                .zip(ay.iter_atoms())
                                .map(|(ix, iy)| f(ix, iy))
                                .collect::<Result<Vec<Atom>>>()?,
                        )))
                    } else {
                        Err(Error::Length)
                    }
                }
                (Noun::Atom(ax), Noun::Array(ay)) => Ok(Noun::Array(Array::from_vec(
                    ay.iter_atoms()
                        .map(|iy| f(ax, iy))
                        .collect::<Result<Vec<Atom>>>()?,
                ))),
                (Noun::Array(ax), Noun::Atom(ay)) => Ok(Noun::Array(Array::from_vec(
                    ax.iter_atoms()
                        .map(|ix| f(ix, ay))
                        .collect::<Result<Vec<Atom>>>()?,
                ))),
            },
            &Dyad::Unimplemented => Err(Error::Unimplemented("Dyad::Unimplemented".into())),
        }
    }
}

fn negate(y: &Atom) -> Result<Atom> {
    match y {
        Atom::Complex(a) => Ok(Atom::Complex(-a)),
    }
}

fn signum(y: &Atom) -> Result<Atom> {
    if let Some(y) = y.try_to_f64() {
        // J signum is 0 for 0, while Rust signum is 1
        if y == 0.0 {
            Ok(Atom::zero())
        } else {
            Ok(y.signum().into())
        }
    } else {
        // Should be a point on the unit circle on the line from the origin to y.
        Err(Error::Unimplemented("signum of complex".into()))
    }
}

fn minus(x: &Atom, y: &Atom) -> Result<Atom> {
    Ok(Atom::Complex(x.to_complex() - y.to_complex()))
}

/// Add atoms.
fn plus(x: &Atom, y: &Atom) -> Result<Atom> {
    Ok(Atom::Complex(x.to_complex() + y.to_complex()))
}

/// `x % y` divide
fn divide(x: &Atom, y: &Atom) -> Result<Atom> {
    let x = x.to_complex();
    let y = y.to_complex();
    // As a special case in J, `0 % 0 = 0`.
    if x.re == 0.0 && x.im == 0.0 {
        // TODO: Maybe `_0 % __` should be 0?
        return Ok(0f64.copysign(y.re).into());
    }
    if y.im == 0f64 {
        // TODO: This might still be wrong for complex numbers.
        if y.re == 0f64 {
            // num_complex 0.recip() gives NaN, but J wants signed infinity.
            // <https://code.jsoftware.com/wiki/Vocabulary/percent>
            return Ok(Atom::Complex(f64::INFINITY.copysign(y.re).into()));
        } else if y.re.is_infinite() {
            return Ok(Atom::Complex(0f64.copysign(y.re).into()));
        }
    }
    Ok(Atom::Complex(x / y))
}

fn times(x: &Atom, y: &Atom) -> Result<Atom> {
    if x.is_zero() || y.is_zero() {
        // Multiplying even infinity by 0 is 0.
        // https://code.jsoftware.com/wiki/Vocabulary/star
        Ok(Atom::zero())
    } else {
        Ok(Atom::Complex(x.to_complex() * y.to_complex()))
    }
}

fn not(y: &Atom) -> Result<Atom> {
    let y = y.try_to_f64().ok_or(Error::Domain)?;
    if y == 0.0 {
        Ok(1.0.into())
    } else if y == 1.0 {
        Ok(0.0.into())
    } else if y > 0.0 && y < 1.0 {
        Ok(Atom::from(1.0 - y))
    } else {
        Err(Error::Domain)
    }
}

fn reciprocal(y: &Atom) -> Result<Atom> {
    divide(&1f64.into(), y)
}

/// Count the items in y.
fn tally(y: &Noun) -> Result<Noun> {
    match y {
        Noun::Atom(_) => Ok(Noun::Atom(1.into())),
        Noun::Array(a) => Ok(Noun::Atom(a.number_items().into())),
    }
}

/// Return a list describing the shape of y.
fn shape_of(y: &Noun) -> Result<Noun> {
    match y {
        Noun::Atom(_) => Ok(Noun::Array(Array::empty())),
        Noun::Array(a) => Ok(Noun::Array(a.shape())),
    }
}

fn integers(y: &Noun) -> Result<Noun> {
    match y {
        Noun::Atom(y) => {
            if let Some(y) = y.try_to_f64() {
                if y < 0.0 {
                    // TODO: Negative numbers should return an array in reverse order.
                    return Err(Error::Unimplemented("i. negative".into()));
                }
                // TODO: Exclude fractions?
                let y = y as usize;
                if y > crate::ARRAY_SIZE_LIMIT {
                    return Err(Error::OutOfMemory);
                }
                Ok(Noun::Array(Array::from((0..y).into_iter().map(Atom::from))))
            } else {
                Err(Error::Domain)
            }
        }
        // TODO: Return a multi-dimensional array.
        _ => Err(Error::Unimplemented("integers from list".into())),
    }
}

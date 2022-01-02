// Copyright 2021 Martin Pool

//! J primitive (built-in) verbs.

// See https://code.jsoftware.com/wiki/Vocabulary/Words#Primitives

use std::borrow::Cow;
use std::fmt;

use fmt::Formatter;

use crate::array::Array;
use crate::atom::Atom;
use crate::error::{Error, Result};
use crate::noun::Noun;
use crate::verb::Verb;

/// A builtin primitive verb, such as `-` or `<.`.
pub struct Primitive(&'static str, Monad, Dyad);

// All implemented primitives.
pub const MINUS: Primitive = Primitive("-", Monad::Zero(negate), Dyad::Zero(minus));
pub const MINUS_DOT: Primitive = Primitive("-.", Monad::Zero(not), Dyad::Unimplemented);
pub const NUMBER: Primitive = Primitive("#", Monad::Infinite(tally), Dyad::Unimplemented);

impl Verb for Primitive {
    fn display(&self) -> Cow<str> {
        Cow::Borrowed(self.0)
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
        write!(f, "{}", self.0)
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
            &Dyad::Unimplemented => Err(Error::Unimplemented("Dyad::Unimplemented")),
        }
    }
}

fn negate(y: &Atom) -> Result<Atom> {
    match y {
        Atom::Complex(a) => Ok(Atom::Complex(-a)),
    }
}

fn minus(x: &Atom, y: &Atom) -> Result<Atom> {
    let Atom::Complex(x) = x;
    let Atom::Complex(y) = y;
    Ok(Atom::Complex(x - y))
}

fn not(y: &Atom) -> Result<Atom> {
    let Atom::Complex(a) = y;
    if a.im != 0.0 {
        Err(Error::Domain)
    } else if a.re == 0.0 {
        Ok(1.0.into())
    } else if a.re == 1.0 {
        Ok(0.0.into())
    } else if a.re > 0.0 && a.re < 1.0 {
        Ok(Atom::from(1.0 - a))
    } else {
        Err(Error::Domain)
    }
}

/// Count the items in y.
fn tally(y: &Noun) -> Result<Noun> {
    match y {
        Noun::Atom(_) => Ok(Noun::Atom(1.into())),
        Noun::Array(a) => Ok(Noun::Atom(a.number_items().into())),
    }
}

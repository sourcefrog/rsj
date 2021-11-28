// Copyright 2021 Martin Pool

//! J primitive (built-in) verbs.

// See https://code.jsoftware.com/wiki/Vocabulary/Words#Primitives

use std::borrow::Cow;
use std::fmt;

use fmt::Formatter;

use crate::array::Array;
use crate::atom::Atom;
use crate::error::Result;
use crate::noun::Noun;
use crate::verb::Verb;

/// A builtin primitive verb, such as `-` or `<.`.
#[derive(Clone)]
pub struct Primitive(
    &'static str,
    fn(&Noun) -> Result<Noun>,
    // dyad: fn(&Noun, &Noun) -> Result<Noun>,
);

impl Verb for Primitive {
    fn display(&self) -> Cow<str> {
        Cow::Borrowed(self.0)
    }

    fn monad(&self, y: &Noun) -> Result<Noun> {
        // TODO: Apply with the correct rank, etc.
        self.1(y)
    }

    fn dyad(&self, _x: &Noun, _y: &Noun) -> Result<Noun> {
        //     self.dyad(x, y)
        todo!();
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

pub const MINUS: Primitive = Primitive("-", negate);

/// If the noun is an atom, apply the function to it. Otherwise, apply it to every element of the
/// array to produce a new array of equal shape.
// TODO: Run this automatically depending on the rank of the verb?
fn monad_per_atom(y: &Noun, f: fn(&Atom) -> Result<Atom>) -> Result<Noun> {
    match y {
        Noun::Atom(a) => f(a).map(Noun::Atom),
        Noun::Array(array) => Ok(Noun::Array(Array::from_vec(
            array.iter().map(f).collect::<Result<Vec<Atom>>>()?,
        ))),
    }
}

fn negate(y: &Noun) -> Result<Noun> {
    monad_per_atom(y, negate_atom)
}

fn negate_atom(y: &Atom) -> Result<Atom> {
    match y {
        Atom::Complex(a) => Ok(Atom::Complex(-a)),
    }
}

pub const MINUS_DOT: Primitive = Primitive("-.", not);

fn not(y: &Noun) -> Result<Noun> {
    monad_per_atom(y, not_atom)
}

fn not_atom(y: &Atom) -> Result<Atom> {
    match y {
        Atom::Complex(a) => {
            if a.im != 0.0 {
                todo!()
            } else if a.re == 0.0 {
                Ok(1.0.into())
            } else if a.re == 1.0 {
                Ok(0.0.into())
            } else {
                todo!()
            }
        }
    }
}

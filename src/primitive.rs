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

impl Primitive {
    /// Lookup a single-character primitive verb.
    pub fn lookup_single(name: char) -> Option<&'static Primitive> {
        Primitive::lookup(&name.to_string())
    }

    /// Lookup an primitive verb by name.
    pub fn lookup(name: &str) -> Option<&'static Primitive> {
        PRIMITIVES.iter().find(|i| i.0 == name)
    }
}

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

fn negate(y: &Noun) -> Result<Noun> {
    match y {
        // TODO: Abstract element-at-a-time etc. This shouldn't need special cases for one
        // number vs many.
        Noun::Atom(Atom::Complex(a)) => Ok(Noun::Atom(Atom::Complex(-a))),
        Noun::Array(Array(v)) => Ok(Noun::Array(Array(
            v.iter().map(|Atom::Complex(c)| Atom::Complex(-c)).collect(),
        ))),
    }
}

pub const MINUS_DOT: Primitive = Primitive("-.", not);

fn not(y: &Noun) -> Result<Noun> {
    match y {
        // TODO: Abstract element-at-a-time etc. This shouldn't need special cases for one
        // number vs many.
        Noun::Atom(Atom::Complex(a)) => {
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
        Noun::Array(_) => todo!(),
    }
}

/// All primitive verbs.
const PRIMITIVES: &[Primitive] = &[MINUS];

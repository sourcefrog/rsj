// Copyright 2021 Martin Pool

//! J verbs (like functions or operators).
//!
//! J verbs have the interesting behavior that they can be applied either as a "monad"
//! with a single right argument (like a prefix operator), or as a "dyad" with left and right
//! arguments (like an infix operator).

use std::borrow::Cow;
use std::fmt;

use fmt::Formatter;

use crate::error::Result;
use crate::noun::Noun;

/// A verb, whether inherent (built-in) or derived.
pub trait Verb: fmt::Debug {
    // TODO: ranks...

    fn display(&self) -> Cow<str>;

    /// Evaluate this verb as a monad.
    fn monad(&self, y: &Noun) -> Result<Noun>;

    /// Evaluate this verb as a dyad.
    fn dyad(&self, x: &Noun, y: &Noun) -> Result<Noun>;
}

/// A builtin verb.
#[derive(PartialEq, Clone)]
pub struct Inherent(
    &'static str,
    // monad: fn(&Noun) -> Result<Noun>,
    // dyad: fn(&Noun, &Noun) -> Result<Noun>,
);

impl Inherent {
    /// Lookup an inherent verb by name.
    pub fn lookup(name: &str) -> Option<&'static Inherent> {
        INHERENTS.iter().find(|i| i.0 == name)
    }
}

impl Verb for Inherent {
    fn display(&self) -> Cow<str> {
        Cow::Borrowed(self.0)
    }

    fn monad(&self, _y: &Noun) -> Result<Noun> {
        //     self.monad(y)
        todo!();
    }

    fn dyad(&self, _x: &Noun, _y: &Noun) -> Result<Noun> {
        //     self.dyad(x, y)
        todo!();
    }
}

impl fmt::Display for Inherent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Inherent {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Inherent").field("name", &self.0).finish()
    }
}

/// All inherent verbs.
const INHERENTS: &[Inherent] = &[Inherent("+")];

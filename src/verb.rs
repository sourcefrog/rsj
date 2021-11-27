// Copyright 2021 Martin Pool

//! J verbs (like functions or operators).
//!
//! J verbs have the interesting behavior that they can be applied either as a "monad"
//! with a single right argument (like a prefix operator), or as a "dyad" with left and right
//! arguments (like an infix operator).

use std::borrow::Cow;
use std::fmt;

use crate::error::Result;
use crate::noun::Noun;

/// A verb, whether primitive or derived.
pub trait Verb: fmt::Debug {
    // TODO: ranks...

    fn display(&self) -> Cow<str>;

    /// Evaluate this verb as a monad.
    fn monad(&self, y: &Noun) -> Result<Noun>;

    /// Evaluate this verb as a dyad.
    fn dyad(&self, x: &Noun, y: &Noun) -> Result<Noun>;
}

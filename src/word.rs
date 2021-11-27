// Copyright 2021 Martin Pool

//! J words and sentences.
//!
//! The most interesting thing about J's grammar is that a space separated list of numbers is a
//! single "word": this accounts for how `* 1 2 3` "knows" to multiply all the numbers: they're
//! effectively a single word which constitues the argument.

use std::fmt;

use crate::noun::Noun;
use crate::primitive::Primitive;

/// A sentence (like a statement) of J code, on a single line.
// TODO: Maybe this isn't adding enough value over just a Vec<Word>?
#[derive(Debug, Clone, PartialEq)]
pub struct Sentence(Vec<Word>);

impl Sentence {
    /// Return a slice of the words in this sentence.
    pub fn words(&self) -> &[Word] {
        &self.0
    }

    /// Return a J-formatted representation of the sentence.
    pub fn display(&self) -> String {
        format!("{}", self)
    }

    /// Wrap a vec of words as a Sentence.
    pub fn from_vec(vec: Vec<Word>) -> Sentence {
        Sentence(vec)
    }

    pub fn empty() -> Sentence {
        Sentence(Vec::new())
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, w) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", w)?;
        }
        Ok(())
    }
}

/// A single J word.
///
/// Note that a list of numbers counts as a single word, even though it contains spaces.
/// So, in J, `1 2 3 + 4 5 6` is three words.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    Noun(Noun),
    Verb(&'static Primitive),
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Noun(noun) => write!(f, "{}", noun),
            Word::Verb(v) => write!(f, "{}", v),
        }
    }
}

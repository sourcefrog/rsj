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
pub type Sentence = Vec<Word>;

/// A single J word.
///
/// Note that a list of numbers counts as a single word, even though it contains spaces.
/// So, in J, `1 2 3 + 4 5 6` is three words.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    Noun(Noun),
    Verb(&'static Primitive),
    OpenParen,
    CloseParen,
}

impl From<Noun> for Word {
    fn from(n: Noun) -> Word {
        Word::Noun(n)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Noun(noun) => noun.fmt(f),
            Word::Verb(verb) => verb.fmt(f),
            Word::OpenParen => f.write_str("("),
            Word::CloseParen => f.write_str(")"),
        }
    }
}

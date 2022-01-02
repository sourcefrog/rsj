// Copyright 2021 Martin Pool

//! Scan J source into words.
//!
//! Scanning proceeds left to right and produces a list of "words" which might be constant nouns,
//! verbs, etc. Comments are dropped at this stage.
//!
//! The most interesting thing about J's grammar is that a space separated list of numbers is a
//! single "word": this accounts for how `* 1 2 3` "knows" to multiply all the numbers: they're
//! effectively a single word which constitues the argument.

use std::str::FromStr;

use num_complex::Complex64;

use crate::atom::Atom;
use crate::error::{Error, Result};
use crate::lex::Lex;
use crate::noun::Noun;
use crate::primitive::Primitive;
use crate::word::{Sentence, Word};

/// Parse J source into a sentence of words.
pub fn parse(s: &str) -> Result<Sentence> {
    Sentence::scan(&mut Lex::new(s)).map(Option::unwrap)
}

/// Scan from characters into objects.
trait Scan {
    /// Attempt to scan an instance of Self from `lex`.
    ///
    /// There are three possible outcomes:
    /// 1. `Ok(Some(x))` -- successfully read one.
    /// 2. `Ok(None)` -- there is no instance of this object here.
    /// 3. `Err(..)` -- part of this object is here but it's invalid.
    fn scan(lex: &mut Lex) -> Result<Option<Self>>
    where
        Self: Sized;
}

impl Scan for Sentence {
    fn scan(lex: &mut Lex) -> Result<Option<Sentence>> {
        let mut words = Vec::new();
        while let Some(word) = Word::scan(lex)? {
            words.push(word);
        }
        Ok(Some(Sentence::from_vec(words)))
    }
}

impl Scan for Word {
    fn scan(lex: &mut Lex) -> Result<Option<Word>> {
        loop {
            lex.drop_whitespace();
            if lex.is_end() {
                return Ok(None);
            }
            if lex.starts_with("NB.") {
                lex.drop_line();
            } else {
                break;
            }
        }
        if let Some(sym) = lex.take_any("#$%&*+-/<=>?@") {
            let mut s = String::from(sym);
            if let Some(dots) = lex.take_any(".:") {
                s.push(dots);
            }
            return Ok(Some(Word::Verb(Primitive::by_name(&s)?)));
        } else if lex.peek().is_ascii_alphabetic() {
            if let Some(dots) = lex.lookahead(1) {
                if dots == '.' || dots == ':' {
                    let mut s = String::from(lex.take());
                    s.push(lex.take());
                    return Ok(Some(Word::Verb(Primitive::by_name(&s)?)));
                }
            }
        }
        // Take as many contiguous numbers as we can as one list-of-numbers "word".
        let mut numbers: Vec<Atom> = Vec::new();
        while let Some(number) = Complex64::scan(lex)? {
            numbers.push(number.into());
            lex.drop_whitespace();
        }
        if numbers.len() == 1 {
            Ok(Some(Word::Noun(Noun::Atom(numbers.remove(0)))))
        } else if !numbers.is_empty() {
            Ok(Some(Word::Noun(Noun::from(numbers))))
        } else if lex.is_end() {
            Ok(None)
        } else {
            Err(Error::Unexpected(lex.peek()))
        }
    }
}

/// Take one number, if there is one.
impl Scan for Complex64 {
    fn scan(lex: &mut Lex) -> Result<Option<Complex64>> {
        if lex.is_end() {
            return Ok(None);
        }
        if lex.peek().is_ascii_digit() || lex.peek() == '_' {
            // TODO: Parse complex numbers with j
            // TODO: `e` exponents.
            // TODO: `x` and `p` for polar coordinates?
            // TODO: More forms from https://www.jsoftware.com/help/dictionary/dcons.htm.
            let mut num_str = String::new();
            while let Some(c) = lex.try_peek() {
                match c {
                    '_' | '.' | '0'..='9' => {
                        // Note: This will accept '123.13.12313' but the later float parser will fail
                        // on it.
                        lex.take();
                        num_str.push(c);
                    }
                    c if c.is_ascii_alphabetic() => return Err(Error::Unexpected(c)),
                    _ => break,
                }
            }
            let number = if num_str == "_" {
                Complex64::new(f64::INFINITY, 0.0)
            } else if num_str == "__" {
                Complex64::new(f64::NEG_INFINITY, 0.0)
            } else {
                if num_str.starts_with('_') {
                    num_str.replace_range(0..=0, "-");
                }
                Complex64::from_str(&num_str).map_err(Error::ParseNumber)?
            };
            Ok(Some(number))
        } else {
            Ok(None) // Doesn't look like a number
        }
    }
}

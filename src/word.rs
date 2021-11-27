// Copyright 2021 Martin Pool

//! Scan J source into words.
//!
//! Scanning proceeds left to right and produces a list of "words" which might be constant nouns,
//! verbs, etc. Comments are dropped at this stage.
//!
//! The most interesting thing about J's grammar is that a space separated list of numbers is a
//! single "word": this accounts for how `* 1 2 3` "knows" to multiply all the numbers: they're
//! effectively a single word which constitues the argument.

use std::fmt;
use std::str::FromStr;

use num_complex::Complex64;

use crate::error::{Error, Result};
use crate::lex::Lex;
use crate::noun::{self, Noun};

/// A sentence (like a statement) of J code, on a single line.
#[derive(Debug, PartialEq)]
pub struct Sentence(Vec<Word>);

/// Parse J source into a sentence of words.
pub fn tokenize(s: &str) -> Result<Sentence> {
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

impl Sentence {
    /// Return a slice of the words in this sentence.
    pub fn words(&self) -> &[Word] {
        &self.0
    }

    /// Return a J-formatted representation of the sentence.
    pub fn display(&self) -> String {
        format!("{}", self)
    }
}

impl Scan for Sentence {
    fn scan(lex: &mut Lex) -> Result<Option<Sentence>> {
        let mut words = Vec::new();
        while let Some(word) = Word::scan(lex)? {
            words.push(word);
        }
        Ok(Some(Sentence(words)))
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
    Constant(noun::Noun),
    Verb(String),
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
        match lex.peek() {
            '+' | '-' | '*' | '%' => return Ok(Some(Word::Verb(format!("{}", lex.take())))),
            _ => (),
        }
        // Take as many contiguous numbers as we can as one list-of-numbers "word".
        let mut numbers = Vec::new();
        loop {
            lex.drop_whitespace();
            if let Some(number) = Complex64::scan(lex)? {
                numbers.push(number)
            } else {
                break;
            }
        }
        if numbers.len() == 1 {
            Ok(Some(Word::Constant(Noun::Number(numbers[0]))))
        } else if !numbers.is_empty() {
            Ok(Some(Word::Constant(Noun::matrix_from_vec(numbers))))
        } else if lex.is_end() {
            Ok(None)
        } else {
            Err(Error::Unexpected(lex.peek()))
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Constant(noun) => write!(f, "{}", noun),
            Word::Verb(v) => write!(f, "{}", v),
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
                    ' ' | '\n' | '\r' | '\t' => break,
                    c => return Err(Error::Unexpected(c as char)),
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

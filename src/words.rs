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

use crate::noun::{self, Noun};

/// An error parsing J source.
#[derive(Debug, PartialEq)]
pub enum Error {
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
}

type Result<T> = std::result::Result<T, Error>;

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
                continue;
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
                return Ok(Some(Word::Constant(Noun::Number(numbers[0]))));
            } else if !numbers.is_empty() {
                return Ok(Some(Word::Constant(Noun::matrix_from_vec(numbers))));
            } else if lex.is_end() {
                return Ok(None);
            } else {
                return Err(Error::Unexpected(lex.peek()));
            }
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

/// A stream of characters from a string being parsed, with lookahead.
struct Lex {
    chars: Vec<char>,
    /// Position of the cursor within [chars].
    pos: usize,
}

impl Lex {
    fn new(s: &str) -> Lex {
        Lex {
            chars: s.chars().collect(),
            pos: 0,
        }
    }

    /// True if at the end of the input.
    pub fn is_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Look at the next character without consuming it.
    ///
    /// Panics at end of input.
    pub fn peek(&self) -> char {
        self.chars[self.pos]
    }

    /// Peek the next character if there is one, or None at the end.
    pub fn try_peek(&self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            Some(self.peek())
        }
    }

    /// Take and return the next character.
    ///
    /// Panics at end of input.
    pub fn take(&mut self) -> char {
        let c = self.chars[self.pos];
        self.pos += 1;
        c
    }

    /// Take the next character, or None at end of input.
    #[allow(unused)]
    pub fn try_take(&mut self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            Some(self.take())
        }
    }

    /// Drop any leading whitespace.
    pub fn drop_whitespace(&mut self) {
        while !self.is_end() {
            if self.peek().is_ascii_whitespace() {
                self.take();
            } else {
                break;
            }
        }
    }

    /// Drop the rest of this line.
    pub fn drop_line(&mut self) {
        while !self.is_end() {
            if self.take() == '\n' {
                break;
            }
        }
    }

    fn remaining(&self) -> usize {
        assert!(self.pos <= self.chars.len());
        self.chars.len() - self.pos
    }

    /// Test if the next few characters match `s`.
    pub fn starts_with(&self, s: &str) -> bool {
        if self.remaining() < s.len() {
            return false;
        }
        let mut p = self.pos;
        for c in s.chars() {
            if c != self.chars[p] {
                return false;
            }
            p += 1;
        }
        true
    }
}

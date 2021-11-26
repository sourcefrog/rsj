// Copyright 2021 Martin Pool

//! Scan J source into words.

use std::str::FromStr;

use num_complex::Complex64;

/// An error parsing J source.
#[derive(Debug, PartialEq)]
pub enum Error {
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
}

type Result<T> = std::result::Result<T, Error>;

/// A sentence (like a statement) of J code, on a single line.
type Sentence = Vec<Word>;

/// Parse J source into a sentence of words.
pub fn tokenize(s: &str) -> Result<Sentence> {
    parse_sentence(&mut Lex::new(s))
}

fn parse_sentence(lex: &mut Lex) -> Result<Sentence> {
    let mut words = Vec::new();
    while let Some(word) = Word::parse(lex)? {
        words.push(word);
    }
    Ok(words)
}

/// A single J word.
///
/// Note that a list of numbers counts as a single word, even though it contains spaces.
/// So, in J, `1 2 3 + 4 5 6` is three words.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    /// A list of one or more numbers.
    Numbers(Vec<Complex64>),
}

impl Word {
    fn parse(lex: &mut Lex) -> Result<Option<Word>> {
        // Take as many contiguous numbers as we can as one list-of-numbers "word".
        let mut numbers = Vec::new();
        loop {
            lex.drop_whitespace();
            if let Some(number) = parse_number(lex)? {
                numbers.push(number)
            } else {
                break;
            }
        }
        if !numbers.is_empty() {
            Ok(Some(Word::Numbers(numbers)))
        } else if lex.is_end() {
            Ok(None)
        } else {
            Err(Error::Unexpected(lex.peek()))
        }
    }
}

/// Take one number, if there is one.
fn parse_number(lex: &mut Lex) -> Result<Option<Complex64>> {
    if lex.is_end() {
        return Ok(None);
    }
    if lex.peek().is_ascii_digit() || lex.peek() == '_' {
        // TODO: Parse complex numbers with j
        // TODO: `e` exponents.
        // TODO: `x` and `p` for polar coordinates?
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

    /// Drop any leading whitespace
    pub fn drop_whitespace(&mut self) {
        while !self.is_end() {
            if self.peek().is_ascii_whitespace() {
                self.take();
            } else {
                break;
            }
        }
    }
}

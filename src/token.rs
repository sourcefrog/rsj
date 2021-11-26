// Copyright 2021 Martin Pool

//! Scan J source into tokens.

use std::str::FromStr;

use num_complex::Complex64;

/// An error parsing J source.
#[derive(Debug, PartialEq)]
pub enum Error {
    Eof,
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
}

type Result<T> = std::result::Result<T, Error>;

/// A sentence (like a statement) of J code, on a single line.
type Sentence = Vec<Word>;

/// A single J word.
///
/// Note that a list of numbers counts as a single word, even though it contains spaces.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    /// A list of one or more numbers.
    Numbers(Vec<Complex64>),
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
    pub fn try_take(&mut self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            Some(self.take())
        }
    }
}

impl Word {
    fn parse(lex: &mut Lex) -> Result<Self> {
        loop {
            // Eat any leading whitespace.
            if lex.is_end() {
                return Err(Error::Eof);
            } else if lex.peek().is_ascii_whitespace() {
                lex.take();
            } else {
                break;
            }
        }
        if lex.peek().is_ascii_digit() || lex.peek() == '_' {
            // TODO: Parse complex numbers with j
            // TODO: `e` exponents.
            // TODO: `x` and `p` for polar coordinates?
            let mut num_str = String::new();
            let mut num_underscores = 0;
            while let Some(c) = lex.try_peek() {
                if c == '_' {
                    lex.take();
                    if !num_str.is_empty() {
                        return Err(Error::Unexpected(c)); // No _ in the middle of a number
                    } else {
                        num_underscores += 1;
                    }
                } else if c.is_ascii_digit() || c == '.' {
                    // Note: This will accept '123.13.12313' but the later float parser will fail
                    // on it.
                    lex.take();
                    num_str.push(c);
                } else if c.is_ascii_whitespace() {
                    break;
                } else {
                    return Err(Error::Unexpected(c as char));
                }
            }
            let number = if num_str == "" {
                if num_underscores == 1 {
                    Complex64::new(f64::INFINITY, 0.0)
                } else if num_underscores == 2 {
                    Complex64::new(f64::NEG_INFINITY, 0.0)
                } else {
                    return Err(Error::Unexpected('_'));
                }
            } else {
                if num_underscores == 1 {
                    num_str.insert(0, '-');
                } else if num_underscores > 1 {
                    return Err(Error::Unexpected('_'));
                }
                Complex64::from_str(&num_str).map_err(Error::ParseNumber)?
            };
            // TODO: Continue on to read multiple numbers
            Ok(Word::Numbers(vec![number]))
        } else {
            return Err(Error::Unexpected(lex.peek()));
        }
    }
}

/// Parse J source into a stream of tokens.
pub fn tokenize(s: &str) -> Result<Sentence> {
    let mut sentence: Sentence = Vec::new();
    let mut lex = Lex::new(s);
    loop {
        match Word::parse(&mut lex) {
            Ok(word) => sentence.push(word),
            Err(Error::Eof) => break,
            Err(e) => return Err(e),
        }
    }
    Ok(sentence)
}

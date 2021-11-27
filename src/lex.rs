// Copyright 2021 Martin Pool

//! A helper for scanning: a character buffer supporting lookahead, skipping whitespace, and other
//! utilities.

/// A stream of characters from a string being parsed, with lookahead.
pub(crate) struct Lex {
    chars: Vec<char>,
    /// Position of the cursor within [chars].
    pos: usize,
}

impl Lex {
    pub fn new(s: &str) -> Lex {
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

    /// Take `n` characters as a String.
    pub fn take_string(&mut self, n: usize) -> String {
        (0..n).map(|_| self.take()).collect()
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

    /// Test if the next few characters match `s`.
    pub fn starts_with(&self, s: &str) -> bool {
        let mut p = self.pos;
        for c in s.chars() {
            if self.chars.get(p) != Some(&c) {
                return false;
            }
            p += 1;
        }
        true
    }
}

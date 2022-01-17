// Copyright 2021, 2022 Martin Pool

//! A helper for scanning: a character buffer supporting lookahead, skipping whitespace, and other
//! utilities.

/// A stream of characters from a string being parsed, with lookahead.
pub(crate) struct Lex<'buf> {
    buf: &'buf [u8],
    /// Position of the cursor within `buf`.
    pos: usize,
}

impl<'buf> Lex<'buf> {
    pub fn new(buf: &'buf [u8]) -> Lex<'buf> {
        Lex { buf, pos: 0 }
    }

    /// True if at the end of the input.
    pub fn is_end(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    /// Look at the next byte without consuming it.
    ///
    /// Panics at end of input.
    #[must_use]
    pub fn peek(&self) -> u8 {
        self.buf[self.pos]
    }

    /// Peek the next byte if there is one, or None at the end.
    #[must_use]
    pub fn try_peek(&self) -> Option<u8> {
        if self.is_end() {
            None
        } else {
            Some(self.peek())
        }
    }

    /// If the next byte is any byte from `s`, consume and return it.
    #[must_use]
    pub fn take_any(&mut self, s: &[u8]) -> Option<u8> {
        if let Some(ch) = self.try_peek() {
            if s.contains(&ch) {
                self.advance();
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Take and return the next byte.
    ///
    /// Panics at end of input.
    #[must_use]
    pub fn take(&mut self) -> u8 {
        let c = self.buf[self.pos];
        self.pos += 1;
        c
    }

    /// Discard the current character.
    pub fn drop(&mut self) {
        self.advance();
    }

    /// Consume the next byte and return true if it's equal to `b`.
    #[must_use]
    pub fn take_if(&mut self, b: u8) -> bool {
        if self.try_peek() == Some(b) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    /// Return the byte that's `n` positions ahead of the cursor, if
    /// there is one.
    pub fn lookahead(&self, n: usize) -> Option<u8> {
        self.buf.get(self.pos + n).cloned()
    }

    /// Drop any leading whitespace.
    pub fn drop_whitespace(&mut self) {
        while !self.is_end() {
            if self.peek().is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Drop the rest of this line.
    pub fn drop_line(&mut self) {
        while !self.is_end() {
            if self.take() == b'\n' {
                break;
            }
        }
    }

    /// Test if the next few characters match `s`.
    #[must_use]
    pub fn starts_with(&self, s: &[u8]) -> bool {
        let mut p = self.pos;
        for c in s {
            if self.buf.get(p) != Some(c) {
                return false;
            }
            p += 1;
        }
        true
    }
}

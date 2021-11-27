// Copyright 2021 Martin Pool

//! Evaluate sentences.

use crate::error::Result;
use crate::verb::Verb;
use crate::word::{tokenize, Sentence, Word};

/// A J interpreter session.
#[derive(Debug, Default)]
pub struct Session {}

impl Session {
    pub fn new() -> Session {
        Session {}
    }

    /// Evaluate one line (as text) and return the result (as text).
    pub fn eval_text(&self, line: &str) -> String {
        match tokenize(line).and_then(|s| self.eval_sentence(&s)) {
            Ok(word) => format!("{}", word),
            Err(err) => format!("error: {:?}", err),
        }
    }

    /// Evaluate a parsed sentence and return the result.
    pub fn eval_sentence(&self, sentence: &Sentence) -> Result<Sentence> {
        // Evaluation proceeds from right to left by pushing words onto a stack, and then reducing
        // the stack if it matches any of several patterns.
        let mut stack: Vec<Word> = Vec::new();
        for w in sentence.words().iter().rev() {
            stack.push(w.clone());
            // dbg!(&stack);
            if stack.len() >= 2 {
                if let Word::Verb(v) = &stack[1] {
                    if let Word::Noun(y) = &stack[0] {
                        let a = v.monad(y)?;
                        stack.pop();
                        stack.pop();
                        stack.push(Word::Noun(a));
                    }
                }
            }
        }
        // TODO: Can this ever return a partly-undigested stack of more than 1 word?
        assert!(stack.len() <= 1);
        Ok(Sentence::from_vec(stack))
    }
}

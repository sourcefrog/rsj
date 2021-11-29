// Copyright 2021 Martin Pool

//! Evaluate sentences.

use crate::error::{Result, Error};
use crate::parse::parse;
use crate::verb::Verb;
use crate::word::{Sentence, Word};

/// A J interpreter session.
#[derive(Debug, Default)]
pub struct Session {}

impl Session {
    pub fn new() -> Session {
        Session {}
    }

    /// Evaluate one line (as text) and return the result (as text).
    pub fn eval_text(&self, line: &str) -> String {
        match parse(line).and_then(|s| self.eval_sentence(&s)) {
            Ok(word) => format!("{}", word),
            Err(err) => format!("error: {:?}", err),
        }
    }

    /// Evaluate a parsed sentence and return the result.
    pub fn eval_sentence(&self, sentence: &Sentence) -> Result<Sentence> {
        // Evaluation proceeds from right to left, looking for patterns that can be evaluated
        // and reduced.
        let mut stack: Vec<Word> = sentence.words().to_vec();
        // We're currently trying to evaluate stack[cursor..(cursor+4)].
        for cursor in (0..(stack.len())).rev() {
            // dbg!(&stack);
            if stack.len() - cursor >= 2 {
                // TODO: Just seeing `VERB NOUN` is not enough to evaluate the verb monadically,
                // because there might be more nouns to the left. We should wait, unless this is
                // the left-hand end of the input...
                if let Word::Verb(v) = &stack[cursor] {
                    if let Word::Noun(y) = &stack[cursor + 1] {
                        stack[cursor] = Word::Noun(v.monad(y)?);
                        stack.remove(cursor + 1);
                    }
                }
            }
        }
        // If the stack wasn't reduced to a single word that's probably 
        // because it contains some grammar we don't support yet...?
        if stack.len() > 1 {
            Err(Error::Unimplemented)
        } else {
            Ok(Sentence::from_vec(stack))
        }
    }
}

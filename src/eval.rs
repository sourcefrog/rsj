// Copyright 2021 Martin Pool

//! Evaluate sentences.

use crate::error::{Error, Result};
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
    pub fn eval_text(&mut self, line: &str) -> String {
        match parse(line).and_then(|s| self.eval_sentence(&s)) {
            Ok(sentence) => format!("{}", sentence),
            Err(err) => format!("error: {:?}", err),
        }
    }

    /// Evaluate a parsed sentence and return the result.
    pub fn eval_sentence(&mut self, sentence: &Sentence) -> Result<Sentence> {
        // Evaluation proceeds from right to left, looking for patterns that can be evaluated
        // and reduced.
        //
        // See https://www.jsoftware.com/help/dictionary/dicte.htm.
        let mut stack: Vec<Word> = sentence.words().to_vec();
        // We're currently trying to evaluate stack[cursor..(cursor+4)].
        for cursor in (0..(stack.len())).rev() {
            // dbg!(&stack);
            if stack.len() - cursor < 2 {
                continue; // not enough to make progress
            }
            if cursor == 0 || matches!(stack[cursor - 1], Word::Verb(..)) {
                // TODO: Parens and assignment should also match here.
                if let [Word::Verb(v), Word::Noun(y), ..] = &stack[cursor..] {
                    stack[cursor] = Word::Noun(v.monad(y)?);
                    stack.remove(cursor + 1);
                    // TODO: Maybe try again at the same cursor position?
                }
            }
            if let [Word::Noun(x), Word::Verb(v), Word::Noun(y), ..] = &stack[cursor..] {
                stack[cursor] = Word::Noun(v.dyad(x, y)?);
                stack.remove(cursor + 1);
                stack.remove(cursor + 1);
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

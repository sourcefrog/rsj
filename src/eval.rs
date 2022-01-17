// Copyright 2021, 2022 Martin Pool

//! Evaluate sentences.

use crate::error::{Error, Result};
use crate::scan::scan_sentence;
use crate::verb::Verb;
use crate::word::{Sentence, Word};

/// A J interpreter session.
#[derive(Debug, Default)]
pub struct Session {}

// TODO: Make this a configurable instance variable in the Session.
const OUTPUT_WIDTH: usize = 80;

impl Session {
    pub fn new() -> Session {
        Session {}
    }

    /// Evaluate one line (as text) and return the result (as text).
    pub fn eval_text(&mut self, line: &str) -> String {
        match scan_sentence(line).and_then(|s| self.eval_sentence(&s)) {
            Ok(Some(word)) => format!("{:.*}", OUTPUT_WIDTH, word),
            Ok(None) => String::new(),
            Err(err) => format!("error: {:?}", err),
        }
    }

    /// Evaluate a parsed sentence and return the result.
    pub fn eval_sentence(&mut self, sentence: &Sentence) -> Result<Option<Word>> {
        // Evaluation proceeds from right to left, looking for patterns that can be evaluated
        // and reduced.
        //
        // See https://www.jsoftware.com/help/dictionary/dicte.htm.
        let mut stack: Vec<Word> = sentence.clone();
        // We're currently trying to evaluate stack[cursor..(cursor+4)].
        let mut cursor = stack.len();
        loop {
            // dbg!(&cursor, &stack);
            // match START ^ VERB:v NOUN:y ...
            // or VERB ^ VERB:v NOUN:y
            // or OPENPAREN ^ VERB:v NOUN:y
            // into applying v to y
            if cursor == 0 || matches!(stack[cursor - 1], Word::Verb(..) | Word::OpenParen) {
                // TODO: Assignment should also match here.
                if let [Word::Verb(v), Word::Noun(y), ..] = &stack[cursor..] {
                    stack[cursor] = Word::Noun(v.monad(y)?);
                    stack.remove(cursor + 1);
                }
            }
            if let [Word::Noun(x), Word::Verb(v), Word::Noun(y), ..] = &stack[cursor..] {
                // ... NOUN:x VERB:v NOUN:y ...
                stack[cursor] = Word::Noun(v.dyad(x, y)?);
                stack.remove(cursor + 1);
                stack.remove(cursor + 1);
            } else if let [Word::OpenParen, Word::Verb(_) | Word::Noun(_), Word::CloseParen, ..] =
                &stack[cursor..]
            {
                // ... OPEN w CLOSE => w
                stack.remove(cursor);
                stack.remove(cursor + 1);
                cursor += 1;
            }
            if cursor == 0 {
                break;
            } else {
                cursor -= 1
            };
        }
        match stack.len() {
            0 => Ok(None),
            1 => {
                let w = stack.pop().unwrap();
                // TODO: This feels kludgey and indicates perhaps the word type
                // should not contain both syntax like parens and also nouns and
                // verbs?
                if matches!(w, Word::Noun(_) | Word::Verb(_)) {
                    Ok(Some(w))
                } else {
                    Err(Error::SyntaxError)
                }
            }
            _ => {
                // If the stack wasn't reduced to a single word that's probably
                // because it contains some grammar that's either invalid, or at least not
                // implemented yet.
                Err(Error::SyntaxError)
            }
        }
    }
}

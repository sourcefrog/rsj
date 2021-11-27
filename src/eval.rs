// Copyright 2021 Martin Pool

//! Evaluate sentences.

use crate::word::tokenize;

/// A J interpreter session.
#[derive(Debug, Default)]
pub struct Session {}

impl Session {
    pub fn new() -> Session {
        Session {}
    }

    /// Evaluate one line (as text) and return the result (as text).
    pub fn eval_text(&self, line: &str) -> String {
        match tokenize(line) {
            Ok(sentence) => format!("{}", sentence),
            Err(err) => format!("error: {:?}", err),
        }
    }

    // /// Evaluate a parsed line and return the object result.
    // pub fn eval_sentence(&self, sentence: &Sentence) -> Result<Sentence> {
    //     Ok(sentence.to_owned())
    // }
}

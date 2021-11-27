// Copyright 2021 Martin Pool

//! Read-eval-print UI.

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::words::tokenize;

const PROMPT: &str = "   ";

/// An interpreter error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {}

// type Result<T> = std::result::Result<T, Error>;

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

/// Read and evaluate input from stdin until stopped by ^c or ^d.
pub fn repl() {
    let mut rl = Editor::<()>::new();
    let session = Session::new();
    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let output = session.eval_text(&line);
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

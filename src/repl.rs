// Copyright 2021 Martin Pool

//! Read-eval-print UI.

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::token::tokenize;

const PROMPT: &str = "   ";

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
}

/// Read and evaluate input from stdin until stopped by ^c or ^d.
pub fn repl() {
    let mut rl = Editor::<()>::new();
    let session = Session::new();
    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", session.eval_text(&line));
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

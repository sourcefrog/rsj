// Copyright 2021 Martin Pool

//! Read-eval-print UI.

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::eval::Session;

const PROMPT: &str = "   ";

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

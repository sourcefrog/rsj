// Copyright 2021 Martin Pool

//! Read-eval-print UI.

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::token::tokenize;

const PROMPT: &str = "   ";

/// Read and evaluate input from stdin until stopped by ^c or ^d.
pub fn repl() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(PROMPT) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                match tokenize(&line) {
                    Ok(sentence) => println!("{:?}", sentence),
                    Err(err) => println!("error: {:?}", err),
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

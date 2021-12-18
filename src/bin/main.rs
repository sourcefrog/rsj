// Copyright 2021 Martin Pool

//! Toy J interpreter: main program.

use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "J language interpreter")]
struct Args {
    #[argh(
        option,
        short = 'D',
        description = "read and update J examples in Markdown file"
    )]
    diff_markdown: Option<PathBuf>,

    #[argh(
        option,
        description = "extract and print the J transcript from a Markdown file"
    )]
    extract_transcript: Option<PathBuf>,
}

fn main() -> rsj::error::Result<()> {
    let args: Args = argh::from_env();
    if let Some(markdown_path) = args.diff_markdown {
        rsj::markdown::update_file(&markdown_path)?;
    } else if let Some(markdown_path) = args.extract_transcript {
        print!("{}", rsj::markdown::extract_transcript(&markdown_path)?);
    } else {
        rsj::repl::repl();
    }
    Ok(())
}

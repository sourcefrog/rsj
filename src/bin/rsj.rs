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
        description = "read and execute J fragments in Markdown file and show diff"
    )]
    diff_markdown: Option<PathBuf>,

    #[argh(
        option,
        short = 'M',
        description = "update a Markdown file containing J fragments"
    )]
    update_markdown: Option<PathBuf>,

    #[argh(
        option,
        description = "extract and print the J transcript from a Markdown file"
    )]
    extract_transcript: Option<PathBuf>,
}

fn main() -> rsj::error::Result<()> {
    let args: Args = argh::from_env();
    if let Some(markdown_path) = args.diff_markdown {
        let diff = rsj::markdown::diff_file(&markdown_path)?;
        print!("{}", diff);
        if !diff.is_empty() {
            std::process::exit(1);
        }
    } else if let Some(mdpath) = args.update_markdown {
        rsj::markdown::update_file(&mdpath)?;
    } else if let Some(markdown_path) = args.extract_transcript {
        print!("{}", rsj::markdown::extract_transcript(&markdown_path)?);
    } else {
        rsj::repl::repl();
    }
    Ok(())
}

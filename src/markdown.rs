// Copyright 2021 Martin Pool

//! Execute J code within code blocks in Markdown documents.

use std::path::Path;

use pulldown_cmark::{Event, Tag};

use crate::error::{Error, Result};

/// Extract J input and output from Markdown; run the commands; update the file to reflect their
/// output.
pub fn update_file(markdown_path: &Path) -> Result<()> {
    let markdown = std::fs::read_to_string(&markdown_path).map_err(|e| Error::IoError(e))?;
    let parser = pulldown_cmark::Parser::new(&markdown);
    for (event, range) in parser.into_offset_iter() {
        // println!("event {:?} at {:?}", event, range);
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                println!("found code block {:?} at {:?}", kind, range);
                println!("{}", &markdown[range]);
            }
            _ => (),
        }
    }
    // TODO: Actually run the examples; collect output; write out.
    Ok(())
}

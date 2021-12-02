// Copyright 2021 Martin Pool

//! Execute J code within code blocks in Markdown documents.

use std::path::Path;

use pulldown_cmark::{CodeBlockKind, Event, Tag};

use crate::error::{Error, Result};

/// Extract J input and output from Markdown; run the commands; update the file to reflect their
/// output.
pub fn update_file(markdown_path: &Path) -> Result<()> {
    let markdown = std::fs::read_to_string(&markdown_path).map_err(|e| Error::IoError(e))?;
    examples_from_markdown(&markdown)?;
    // TODO: Actually run the examples; collect output; write out.
    Ok(())
}

/// A section of a markdown file.
enum Chunk<'markdown> {
    /// A chunk of J input and output lines, left-aligned.
    JExample(String, CodeBlockKind<'markdown>),
    /// Any other markdown text.
    Other(&'markdown str),
}

/// Parse markdown text into a series of chunks that are either J examples, or
/// any other text.
///
/// The resulting Chunks, when concatenated, should exactly reproduce the markdown input.
fn examples_from_markdown(markdown: &str) -> Result<Vec<Chunk>> {
    // The parser events don't account for 100% of input bytes, but we do want to exactly
    // reproduce the input, assuming the J output already has the right values.
    // Therefore, rather than concatenating all the tags, we specifically mark
    // out hunks for J text, and everything in between them counts as Other.
    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut chunks = Vec::new();
    let mut in_j_block = false;
    // Everything in markdown[..prev] has already been moved into chunks...
    let mut prev: usize = 0;
    for (event, range) in parser.into_offset_iter() {
        println!("{:?} at {:?}", event, range);
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                // TODO: Look at kind and fenced-block tags
                assert!(!in_j_block, "nested code blocks?");
                in_j_block = true;
                if range.start > prev {
                    chunks.push(Chunk::Other(&markdown[prev..range.start]));
                }
                // TODO: Strip out any extra indents and the fences.
                let text = &markdown[range.clone()];
                let text: String = match kind {
                    CodeBlockKind::Indented => strip_indents(text),
                    _ => unimplemented!(),
                };
                chunks.push(Chunk::JExample(text, kind.clone()));
                prev = range.end;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_j_block = false;
            }
            _ => (),
        }
    }
    if prev < markdown.len() {
        chunks.push(Chunk::Other(&markdown[prev..]));
    }
    Ok(chunks)
}

/// pulldown_cmark returns block-indented code with the indent stripped from the first line, but
/// not the following lines. Strip it off all of them.
fn strip_indents(text: &str) -> String {
    // TODO: Don't assume it's 4 spaces?
    let mut s = String::new();
    for (i, l) in text.lines().enumerate() {
        if i > 0 {
            s.push_str(l.strip_prefix("    ").unwrap());
        } else {
            s.push_str(&l);
        }
        s.push('\n');
    }
    s
}

fn reinsert_indents(ijs: &str) -> String {
    let mut s = String::new();
    for (i, l) in ijs.lines().enumerate() {
        if i > 0 {
            s.push_str("    ");
        }
        s.push_str(l);
        s.push('\n');
    }
    s
}

fn recombine_chunks(chunks: &[Chunk]) -> String {
    let mut s = String::new();
    for c in chunks {
        match c {
            Chunk::Other(text) => s.push_str(&text),
            Chunk::JExample(text, kind) => {
                // TODO: Re-insert fences or indents.
                match kind {
                    CodeBlockKind::Indented => {
                        s.push_str(&reinsert_indents(text));
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
    s
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn extract_and_recombine_examples() {
        let md = "Introductory text.

       3 + 4
    7

And closing text.
";
        let chunks = examples_from_markdown(&md).unwrap();
        assert_eq!(&recombine_chunks(&chunks), md);
        let examples: Vec<&Chunk> = chunks
            .iter()
            .filter(|i| matches!(i, Chunk::JExample(_, _)))
            .collect();
        assert_eq!(examples.len(), 1);
        match &examples[0] {
            &Chunk::JExample(text, kind) => {
                assert_eq!(*kind, CodeBlockKind::Indented);
                assert_eq!(text, &"   3 + 4\n7\n");
            }
            _ => panic!(),
        }
    }
}

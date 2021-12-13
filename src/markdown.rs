// Copyright 2021 Martin Pool

//! Execute J code within code blocks in Markdown documents.

use std::path::Path;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};

use crate::error::{Error, Result};

/// Extract J input and output from Markdown; run the commands; update the file to reflect their
/// output.
pub fn update_file(markdown_path: &Path) -> Result<()> {
    let markdown = std::fs::read_to_string(&markdown_path).map_err(Error::IoError)?;
    let literate = Literate::from_str(&markdown)?;
    for chunk in literate.chunks {
        if let Chunk::JExample(example, _) = chunk {
            print!("{}", example);
        }
    }
    // TODO: Actually run the examples; collect output; write out.
    Ok(())
}

/// A parsed Markdown file containing J examples.
///
/// The lifetime is bounded by a markdown source string held externally.
pub struct Literate<'markdown> {
    chunks: Vec<Chunk<'markdown>>,
}

/// A section of a markdown file.
enum Chunk<'markdown> {
    /// A chunk of J input and output lines, left-aligned.
    JExample(String, CodeBlockKind<'markdown>),
    /// Any other markdown text.
    Other(&'markdown str),
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

impl<'markdown> Literate<'markdown> {
    /// Parse markdown text into a series of chunks that are either J examples, or
    /// any other text.
    ///
    /// The resulting Chunks, when concatenated, should exactly reproduce the markdown input.
    ///
    /// This is not exactly std::str::FromStr because it keeps pointers into the input.
    fn from_str(markdown: &'markdown str) -> Result<Literate<'markdown>> {
        // The parser events don't account for 100% of input bytes, but we do want to exactly
        // reproduce the input, assuming the J output already has the right values.
        // Therefore, rather than concatenating all the tags, we specifically mark
        // out hunks for J text, and everything in between them counts as Other.
        let parser = pulldown_cmark::Parser::new(markdown);
        let mut chunks = Vec::new();
        let mut in_j_block = None;
        // Everything in markdown[..prev] has already been moved into chunks...
        let mut prev: usize = 0;
        // All the text in the currently incomplete J code block.
        let mut current_code: Vec<CowStr> = Vec::new();
        for (event, range) in parser.into_offset_iter() {
            // println!("{:?} at {:?}", event, range);
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    // TODO: Look at kind and fenced-block tags
                    assert!(in_j_block.is_none(), "nested code blocks?");
                    in_j_block = Some(kind);
                    if range.start > prev {
                        chunks.push(Chunk::Other(&markdown[prev..range.start]));
                    }
                }
                Event::End(Tag::CodeBlock(_)) => {
                    chunks.push(Chunk::JExample(
                        current_code.concat(),
                        in_j_block.take().unwrap(),
                    ));
                    current_code.clear();
                    prev = range.end;
                }
                Event::Text(t) if in_j_block.is_some() => {
                    current_code.push(t);
                }
                _ => (),
            }
        }
        assert!(in_j_block.is_none());
        assert!(current_code.is_empty());
        if prev < markdown.len() {
            chunks.push(Chunk::Other(&markdown[prev..]));
        }
        Ok(Literate { chunks })
    }

    /// Reassemble text and examples into a Markdown doc.
    ///
    /// If all the examples are up-to-date this should recreate the input exactly.
    fn reassemble(&self) -> String {
        let mut s = String::new();
        for c in &self.chunks {
            match c {
                Chunk::Other(text) => s.push_str(text),
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
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    const MD: &str = "# Literate J in Markdown

Introductory text.

       3 + 4
    7

And closing text.
";

    #[test]
    fn extract_and_recombine_examples() {
        let literate = Literate::from_str(MD).unwrap();
        assert_eq!(
            &literate.reassemble(),
            MD,
            "chunks recombine to exacply match the input"
        );
        let examples: Vec<&Chunk> = literate
            .chunks
            .iter()
            .filter(|i| matches!(i, Chunk::JExample(_, _)))
            .collect();
        assert_eq!(examples.len(), 1);
        match &examples[0] {
            &Chunk::JExample(text, kind) => {
                assert_eq!(*kind, CodeBlockKind::Indented);
                assert_eq!(
                    text, &"   3 + 4\n7\n",
                    "first line of input is indented with J prompt; others are flush"
                );
            }
            _ => panic!(),
        }
    }
}

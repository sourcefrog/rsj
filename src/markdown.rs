// Copyright 2021 Martin Pool

//! Execute J code within code blocks in Markdown documents.

use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
use similar::TextDiff;

use crate::error::Result;
use crate::eval::Session;
use crate::transcript;

/// Extract J input and output from Markdown; run the commands; return a diff
/// reflecting differences in output.
///
/// If there are no differences the result is an empty string.
pub fn diff_file(markdown_path: &Path) -> Result<String> {
    let markdown = std::fs::read_to_string(&markdown_path)?;
    let output = Literate::parse(&markdown)?.run(&mut Session::new())?;
    let text_diff = TextDiff::from_lines(&markdown, &output);
    let old_name = format!("{}", markdown_path.display());
    let new_name = format!("{}.new", markdown_path.display());
    Ok(text_diff
        .unified_diff()
        .context_radius(8)
        .header(&old_name, &new_name)
        .to_string())
}

/// Run the J source embeddet in a Markdown file and update the file with the
/// results of executing the J sentences.
pub fn update_file(markdown_path: &Path) -> Result<()> {
    let markdown = std::fs::read_to_string(&markdown_path)?;
    let output = Literate::parse(&markdown)?.run(&mut Session::new())?;
    if output != markdown {
        let backup_path = PathBuf::from(format!("{}.old", markdown_path.display()));
        fs::rename(markdown_path, backup_path)?;
        fs::write(markdown_path, output.as_bytes())?;
    }
    Ok(())
}

pub fn extract_transcript(markdown_path: &Path) -> Result<String> {
    let markdown = std::fs::read_to_string(&markdown_path)?;
    Literate::parse(&markdown)?.extract_transcript()
}

/// A section of a markdown file.
enum Chunk<'markdown> {
    /// A chunk of J input and output lines, left-aligned.
    J(String, CodeBlockKind<'markdown>),
    /// Any other markdown text.
    Other(&'markdown str),
}

/// A parsed Markdown file containing J examples.
///
/// The lifetime is bounded by a markdown source string held externally.
struct Literate<'markdown> {
    chunks: Vec<Chunk<'markdown>>,
}

impl<'markdown> Literate<'markdown> {
    /// Parse markdown text into a series of chunks that are either J examples, or
    /// any other text.
    ///
    /// The resulting Chunks, when concatenated, should exactly reproduce the markdown input.
    ///
    /// This is not exactly std::str::FromStr because it keeps pointers into the input.
    fn parse(md: &'markdown str) -> Result<Literate<'markdown>> {
        // The parser events don't account for 100% of input bytes, but we do want to exactly
        // reproduce the input, assuming the J output already has the right values.
        // Therefore, rather than concatenating all the tags, we specifically mark
        // out hunks for J text, and everything in between them counts as Other.
        let parser = pulldown_cmark::Parser::new(md);
        let mut in_j_block = None;
        // Everything in markdown[..prev] has already been moved into chunks...
        let mut prev: usize = 0;
        // All the text in the currently incomplete J code block.
        let mut current_code: Vec<CowStr> = Vec::new();
        let mut chunks = Vec::new();
        for (event, range) in parser.into_offset_iter() {
            // println!("{:?} at {:?}", event, range);
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    // TODO: Look at kind and fenced-block tags
                    assert!(in_j_block.is_none(), "nested code blocks?");
                    in_j_block = Some(kind);
                    if range.start > prev {
                        chunks.push(Chunk::Other(&md[prev..range.start]));
                    }
                }
                Event::End(Tag::CodeBlock(_)) => {
                    chunks.push(Chunk::J(current_code.concat(), in_j_block.take().unwrap()));
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
        if prev < md.len() {
            chunks.push(Chunk::Other(&md[prev..]));
        }
        Ok(Literate { chunks })
    }

    /// Return the J transcript of all the examples.
    fn extract_transcript(&self) -> Result<String> {
        let mut s = String::new();
        for chunk in &self.chunks {
            if let Chunk::J(example, _) = chunk {
                s.push_str(example)
            }
        }
        Ok(s)
    }

    /// Run all the examples and return the resulting reassembled document.
    pub fn run(&self, session: &mut Session) -> Result<String> {
        let mut output = String::new();
        for chunk in &self.chunks {
            match chunk {
                Chunk::J(j, kind) => {
                    let chunk_out = transcript::rerun(session, j)?;
                    match kind {
                        CodeBlockKind::Indented => {
                            // TODO: This might be wrong if it's indented more than one level.
                            output.push_str(&reinsert_indents(&chunk_out))
                        }
                        CodeBlockKind::Fenced(tags) => {
                            writeln!(output, "```{}", tags).unwrap();
                            output.push_str(&chunk_out);
                            output.push_str("```");
                        }
                    }
                }
                Chunk::Other(md) => output.push_str(md),
            }
        }
        Ok(output)
    }

    /// Reassemble text and examples into a Markdown doc.
    ///
    /// If all the examples are up-to-date this should recreate the input exactly.
    #[allow(unused)]
    fn reassemble(&self) -> String {
        let mut s = String::new();
        for c in &self.chunks {
            match c {
                Chunk::Other(text) => s.push_str(text),
                Chunk::J(text, kind) => {
                    // TODO: Re-insert fences or indents.
                    match kind {
                        // TODO: This might be wrong if it's indented more than one level.
                        CodeBlockKind::Indented => {
                            s.push_str(&reinsert_indents(text));
                        }
                        CodeBlockKind::Fenced(tags) => {
                            writeln!(s, "```{}", tags).unwrap();
                            s.push_str(text);
                            s.push_str("```");
                        }
                    }
                }
            }
        }
        s
    }
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
        let literate = Literate::parse(MD).unwrap();
        assert_eq!(
            &literate.reassemble(),
            MD,
            "chunks recombine to exacply match the input"
        );
        let examples: Vec<&Chunk> = literate
            .chunks
            .iter()
            .filter(|i| matches!(i, Chunk::J(_, _)))
            .collect();
        assert_eq!(examples.len(), 1);
        match &examples[0] {
            &Chunk::J(text, kind) => {
                assert_eq!(*kind, CodeBlockKind::Indented);
                assert_eq!(
                    text, &"   3 + 4\n7\n",
                    "first line of input is indented with J prompt; others are flush"
                );
            }
            _ => panic!(),
        }
        assert_eq!(
            literate.extract_transcript().unwrap(),
            "   3 + 4
7
"
        );
    }
}

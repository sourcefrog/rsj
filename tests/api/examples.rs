// Copyright 2021 Martin Pool

//! Test example session files.

use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

use pretty_assertions::assert_eq;

use rsj::eval::Session;

const PROMPT: &str = "   ";

fn glob_in_dir<'a, P>(dir: &P, extension: &'a str) -> impl Iterator<Item = PathBuf> + 'a
where
    P: AsRef<Path> + ?Sized,
{
    read_dir(dir.as_ref())
        .expect("read_dir")
        .map(Result::unwrap)
        .map(|de| de.path())
        .filter(move |path| path.extension().and_then(|e| e.to_str()) == Some(extension))
}

#[test]
fn examples_j() {
    for path in glob_in_dir("t", "ijs") {
        println!("** {:?}", path);
        run_j_example(&path);
    }
}

fn run_j_example(path: &Path) {
    let mut session = Session::new();
    let body = fs::read_to_string(path).unwrap();
    let mut lines = body.lines();
    while let Some(input) = lines.next() {
        println!("{}", input);
        if input.is_empty() {
            continue;
        }
        let input = input.strip_prefix(PROMPT).expect("prompt on input line");
        let output = session.eval_text(input);

        if !output.is_empty() {
            let expected = lines.next().unwrap();
            assert!(!expected.starts_with(PROMPT));
            assert_eq!(output, expected);
        }
    }
}

#[test]
fn examples_md() {
    for md_path in glob_in_dir("t", "md") {
        println!("** {:?}", &md_path);
        let diff = rsj::markdown::diff_file(&md_path).unwrap();
        println!("{}", diff);
        assert!(diff.is_empty());
    }
}

// Copyright 2021 Martin Pool

//! Test example session files.

use std::fs;
use std::path::Path;

use pretty_assertions::assert_eq;

use rsj::repl::Session;

const PROMPT: &str = "   ";
const EXAMPLE_DIR: &str = "t";

#[test]
fn examples() {
    let dir = Path::new(EXAMPLE_DIR);
    for entry in fs::read_dir(dir).unwrap().map(Result::unwrap) {
        let example_path = dir.join(entry.file_name());
        println!("** {:?}", example_path);
        test_one_example(&example_path);
    }
}

fn test_one_example(path: &Path) {
    let session = Session::new();
    let body = fs::read_to_string(path).unwrap();
    let mut lines = body.lines();
    while let Some(input) = lines.next() {
        println!("{}", input);
        let input = input.strip_prefix(PROMPT).expect("prompt on input line");
        let expected = lines.next().unwrap();
        assert!(!expected.starts_with(PROMPT));

        let output = session.eval_text(input);
        assert_eq!(output, expected);
    }
}

// Copyright 2021 Martin Pool

//! Test example session files.

use std::fs;
use std::path::Path;

use pretty_assertions::assert_eq;

use rsj::eval::Session;

const PROMPT: &str = "   ";
const EXAMPLE_DIR: &str = "t";

#[test]
fn examples_j() {
    let dir = Path::new(EXAMPLE_DIR);
    for path in fs::read_dir(dir)
        .unwrap()
        .map(Result::unwrap)
        .map(|de| de.path())
    {
        if path.extension().and_then(|e| e.to_str()) == Some("ijs") {
            println!("** {:?}", path);
            test_one_example(&path);
        }
    }
}

fn test_one_example(path: &Path) {
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

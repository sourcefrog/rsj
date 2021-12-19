// Copyright 2021 Martin Pool

//! Test the Markdown-handling options.

use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use assert_cmd::Command;

fn needs_update_md_files() -> impl Iterator<Item = PathBuf> {
    read_dir("t/needs_update")
        .expect("read t/needs_update")
        .map(Result::unwrap)
        .map(|de| de.path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("md"))
}

#[test]
fn diff_needs_update() {
    for path in needs_update_md_files() {
        let diff_file = format!("{}.diff", path.display());
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-D")
            .arg(path)
            .assert()
            .stderr("")
            .stdout(read_to_string(&diff_file).expect("read diff file"))
            .code(1);
    }
}

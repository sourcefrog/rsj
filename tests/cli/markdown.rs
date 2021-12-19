// Copyright 2021 Martin Pool

//! Test the Markdown-handling options.

use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;

fn md_files_in_dir<P>(p: &P) -> impl Iterator<Item = PathBuf>
where
    P: AsRef<Path>,
{
    read_dir(p.as_ref())
        .expect("read_dir")
        .map(Result::unwrap)
        .map(|de| de.path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("md"))
}

#[test]
fn diff_needs_update() {
    for path in md_files_in_dir(&"t/needs_update") {
        println!("** {}", path.display());
        let diff_file = format!("{}.diff", path.display());
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-D")
            .arg(path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(read_to_string(&diff_file).expect("read diff file"))
            .code(1);
    }
}

#[test]
fn blog_files_are_up_to_date() {
    for path in md_files_in_dir(&"blog") {
        println!("** {}", path.display());
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-D")
            .arg(path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::is_empty())
            .code(0);
    }
}

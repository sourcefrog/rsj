// Copyright 2021 Martin Pool

//! Test the Markdown-handling options.

use std::fs::{self, read_dir, read_to_string};
use std::path::{Path, PathBuf};

use assert_cmd::prelude::OutputOkExt;
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
fn update_files_needing_update() {
    let tmpdir = tempfile::tempdir().unwrap();
    for path in md_files_in_dir(&"t/needs_update") {
        println!("** {}", path.display());
        let tmp_path = tmpdir.path().join(path.file_name().unwrap());
        fs::copy(path, &tmp_path).unwrap();
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-M")
            .arg(tmp_path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::is_empty())
            .code(0);
        // TODO: Check the expected content.
        // TODO: Check the backup file was created and is identical to
        // the original content.
        // TODO: Check that neither the mtime or content of the text
        // input file was mutated - which should be impossible since
        // we made a copy, but let's make sure.
        // TODO: Check that running it again does nothing.
    }
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

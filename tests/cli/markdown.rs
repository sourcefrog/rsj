// Copyright 2021 Martin Pool

//! Test the Markdown-handling options.

use std::fs::{self, read_dir, read_to_string};
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

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
    let testdata_dir = Path::new("t/needs_update");
    let snapshot_dir = Path::new("../../").join(testdata_dir);
    for path in md_files_in_dir(&testdata_dir) {
        println!("** {}", path.display());
        let tmp_path = tmpdir.path().join(path.file_name().unwrap());
        fs::copy(&path, &tmp_path).unwrap();
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-M")
            .arg(&tmp_path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::is_empty())
            .code(0);
        insta::with_settings!({snapshot_path => &snapshot_dir}, {
            insta::assert_snapshot!(
                path.display().to_string(),
                read_to_string(&tmp_path).unwrap()
            );
        });
        let backup_path = PathBuf::from(format!("{}.old", tmp_path.display()));
        assert_eq!(
            read_to_string(backup_path).unwrap(),
            read_to_string(&path).unwrap(),
            "backup file doesn't match the original file"
        );
    }
}

#[test]
fn diff_shows_expected_update() {
    for path in md_files_in_dir(&"t/needs_update") {
        println!("** {}", path.display());
        let diff_file = format!("{}.diff", path.display());
        let a = Command::cargo_bin("rsj")
            .unwrap()
            .arg("-D")
            .arg(path)
            .assert()
            .stderr(predicate::str::is_empty())
            .code(1);
        let expected_diff = read_to_string(&diff_file)
            .expect("read diff file")
            .replace("\r\n", "\n");
        let output_str = String::from_utf8_lossy(&a.get_output().stdout).replace("\r\n", "\n");
        assert_eq!(output_str, expected_diff);
    }
}

#[test]
fn blog_files_are_up_to_date() {
    let tmpdir = tempfile::tempdir().unwrap();
    for path in md_files_in_dir(&"blog") {
        println!("** {}", path.display());
        let tmp_path = tmpdir.path().join(path.file_name().unwrap());
        fs::copy(&path, &tmp_path).unwrap();
        let orig_tmp_mtime = fs::metadata(&tmp_path).unwrap().modified().unwrap();

        // -D produces no output and succeeds, because it's up to date.
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-D")
            .arg(&tmp_path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::is_empty())
            .code(0);

        // -M does not modify the file or produce a backup.
        Command::cargo_bin("rsj")
            .unwrap()
            .arg("-M")
            .arg(&tmp_path)
            .assert()
            .stderr(predicate::str::is_empty())
            .stdout(predicate::str::is_empty())
            .code(0);
        let backup_path = PathBuf::from(format!("{}.old", tmp_path.display()));
        assert!(!backup_path.exists(), "{:?} exists", &backup_path);
        assert_eq!(
            fs::metadata(&tmp_path).unwrap().modified().unwrap(),
            orig_tmp_mtime
        );
    }
}

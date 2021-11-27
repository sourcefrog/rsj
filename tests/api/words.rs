// Copyright 2021 Martin Pool

//! Tests for scanning text into sentences of words.
//!
//! The scanner is primarily tested by parsing example files from the `t/` directory, but these
//! tests are more fundamental and also exercise aspects of the API that are less visible in text,
//! including the fact that a word can contain multiple numbers.

use num_complex::Complex64;
use pretty_assertions::assert_eq;

use rsj::noun::Noun;
use rsj::words::{self, tokenize, Word};

#[test]
fn number_with_whitespace() {
    let sentence = tokenize("  123.45  ").unwrap();
    assert_eq!(sentence.words(), &[Word::Constant(Noun::from(123.45))]);
    assert_eq!(sentence.display(), "123.45");
}

#[test]
fn simple_integer() {
    assert_eq!(
        tokenize("123").unwrap().words(),
        &[Word::Constant(Noun::from(123.0))]
    );
    assert_eq!(tokenize("123").unwrap().display(), "123");
}

#[test]
fn simple_floating_point() {
    let sentence = tokenize("123.456").unwrap();
    assert_eq!(sentence.display(), "123.456");
    assert_eq!(sentence.words(), &[Word::Constant(123.456.into())]);
}

#[test]
fn fraction() {
    let s = tokenize("0.456789").unwrap();
    assert_eq!(s.display(), "0.456789");
    assert_eq!(s.words(), &[Word::Constant(0.456789.into())]);
}

#[test]
fn negative() {
    let s = tokenize("_1").unwrap();
    assert_eq!(s.display(), "_1");
    assert_eq!(s.words(), &[Word::Constant(Noun::from(-1.0))]);
}

#[test]
fn infinities() {
    assert_eq!(
        tokenize("_").unwrap().words(),
        &[Word::Constant(Noun::from(f64::INFINITY))]
    );
    assert_eq!(tokenize("_").unwrap().display(), "_");

    assert_eq!(
        tokenize("__").unwrap().words(),
        &[Word::Constant(Noun::from(f64::NEG_INFINITY))]
    );
    assert_eq!(tokenize("__").unwrap().display(), "__");
}

#[test]
fn no_underscore_inside_numbers() {
    assert!(matches!(
        tokenize("1_000"),
        Err(words::Error::ParseNumber(_))
    ));
}

#[test]
fn several_numbers_in_one_word() {
    assert_eq!(
        tokenize("  1 2 3 _4.56 _99 __").unwrap().words(),
        &[Word::Constant(Noun::matrix_from_vec(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(3.0, 0.0),
            Complex64::new(-4.56, 0.0),
            Complex64::new(-99.0, 0.0),
            Complex64::new(f64::NEG_INFINITY, 0.0),
        ]))]
    );
    assert_eq!(
        tokenize("  1 2 3 _4.56 _99 __").unwrap().display(),
        "1 2 3 _4.56 _99 __"
    );
}

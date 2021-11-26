// Copyright 2021 Martin Pool

//! Test tokenizing sentences.

use num_complex::Complex64;
use pretty_assertions::assert_eq;

use rsj::token::{self, tokenize, Word};

#[test]
fn number_with_whitespace() {
    let sentence = tokenize("  123.45  ").unwrap();
    assert_eq!(
        sentence.words(),
        &[Word::Numbers(vec![Complex64::new(123.45, 0.0)])]
    );
    assert_eq!(sentence.display(), "123.45");
}

#[test]
fn simple_integer() {
    assert_eq!(
        tokenize("123").unwrap().words(),
        &[Word::Numbers(vec![Complex64::new(123.0, 0.0)])]
    );
    assert_eq!(tokenize("123").unwrap().display(), "123");
}

#[test]
fn simple_floating_point() {
    let sentence = tokenize("123.456").unwrap();
    assert_eq!(sentence.display(), "123.456");
    assert_eq!(
        sentence.words(),
        &[Word::Numbers(vec![Complex64::new(123.456, 0.0)])]
    );
}

#[test]
fn fraction() {
    let s = tokenize("0.456789").unwrap();
    assert_eq!(s.display(), "0.456789");
    assert_eq!(
        s.words(),
        &[Word::Numbers(vec![Complex64::new(0.456789, 0.0)])]
    );
}

#[test]
fn negative() {
    let s = tokenize("_1").unwrap();
    assert_eq!(s.display(), "_1");
    assert_eq!(s.words(), &[Word::Numbers(vec![Complex64::new(-1.0, 0.0)])]);
}

#[test]
fn infinities() {
    assert_eq!(
        tokenize("_").unwrap().words(),
        &[Word::Numbers(vec![Complex64::new(f64::INFINITY, 0.0)])]
    );
    assert_eq!(tokenize("_").unwrap().display(), "_");

    assert_eq!(
        tokenize("__").unwrap().words(),
        &[Word::Numbers(vec![Complex64::new(f64::NEG_INFINITY, 0.0)])]
    );
    assert_eq!(tokenize("__").unwrap().display(), "__");
}

#[test]
fn no_underscore_inside_numbers() {
    assert!(matches!(
        tokenize("1_000"),
        Err(token::Error::ParseNumber(_))
    ));
}

#[test]
fn several_numbers_in_one_word() {
    assert_eq!(
        tokenize("  1 2 3 _4.56 _99 __").unwrap().words(),
        &[Word::Numbers(vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(3.0, 0.0),
            Complex64::new(-4.56, 0.0),
            Complex64::new(-99.0, 0.0),
            Complex64::new(f64::NEG_INFINITY, 0.0),
        ])]
    );
    assert_eq!(
        tokenize("  1 2 3 _4.56 _99 __").unwrap().display(),
        "1 2 3 _4.56 _99 __"
    );
}

// Copyright 2021 Martin Pool

//! Test tokenizing sentences.

use num_complex::Complex64;
use pretty_assertions::assert_eq;

use rsj::token::{self, tokenize, Word};

#[test]
fn number_with_whitespace() {
    assert_eq!(
        tokenize("  123.45  ").unwrap(),
        &[Word::Numbers(vec![Complex64::new(123.45, 0.0)])]
    );
}

#[test]
fn simple_integer() {
    assert_eq!(
        tokenize("123").unwrap(),
        &[Word::Numbers(vec![Complex64::new(123.0, 0.0)])]
    );
}

#[test]
fn simple_floating_point() {
    assert_eq!(
        tokenize("123.456").unwrap(),
        &[Word::Numbers(vec![Complex64::new(123.456, 0.0)])]
    );
    assert_eq!(
        tokenize("0.456789").unwrap(),
        &[Word::Numbers(vec![Complex64::new(0.456789, 0.0)])]
    );
}

#[test]
fn negative() {
    assert_eq!(
        tokenize("_1").unwrap(),
        &[Word::Numbers(vec![Complex64::new(-1.0, 0.0)])]
    );
}

#[test]
fn infinities() {
    assert_eq!(
        tokenize("_").unwrap(),
        &[Word::Numbers(vec![Complex64::new(f64::INFINITY, 0.0)])]
    );
    assert_eq!(
        tokenize("__").unwrap(),
        &[Word::Numbers(vec![Complex64::new(f64::NEG_INFINITY, 0.0)])]
    );
}

#[test]
fn no_underscore_inside_numbers() {
    assert_eq!(
        tokenize("1_000").unwrap_err(),
        token::Error::Unexpected('_')
    );
}

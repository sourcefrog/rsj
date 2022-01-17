// Copyright 2021 Martin Pool

//! Tests for scanning text into sentences of words.
//!
//! The scanner is primarily tested by parsing example files from the `t/` directory, but these
//! tests are more fundamental and also exercise aspects of the API that are less visible in text,
//! including the fact that a word can contain multiple numbers.

use num_complex::Complex64;
use pretty_assertions::assert_eq;

use rsj::array::Array;
use rsj::error::Error;
use rsj::noun::Noun;
use rsj::primitive;
use rsj::scan::scan_sentence;
use rsj::word::Word;

#[test]
fn number_with_whitespace() {
    let sentence = scan_sentence("  123.45  ").unwrap();
    assert_eq!(sentence, &[Word::Noun(Noun::from(123.45))]);
}

#[test]
fn simple_integer() {
    assert_eq!(
        scan_sentence("123").unwrap(),
        &[Word::Noun(Noun::from(123.0))]
    );
}

#[test]
fn simple_floating_point() {
    let sentence = scan_sentence("123.456").unwrap();
    assert_eq!(sentence, &[Word::Noun(123.456.into())]);
}

#[test]
fn fraction() {
    let s = scan_sentence("0.456789").unwrap();
    assert_eq!(s, &[Word::Noun(0.456789.into())]);
}

#[test]
fn negative() {
    let s = scan_sentence("_1").unwrap();
    assert_eq!(s, &[Word::Noun(Noun::from(-1.0))]);
}

#[test]
fn infinities() {
    assert_eq!(
        scan_sentence("_").unwrap(),
        &[Word::Noun(Noun::from(f64::INFINITY))]
    );

    assert_eq!(
        scan_sentence("__").unwrap(),
        &[Word::Noun(Noun::from(f64::NEG_INFINITY))]
    );
}

#[test]
fn primitive() {
    let minus = &primitive::MINUS;
    assert_eq!(
        scan_sentence(" - -").unwrap(),
        &[Word::Verb(minus), Word::Verb(minus),]
    );
}

#[test]
fn no_underscore_inside_numbers() {
    assert!(matches!(scan_sentence("1_000"), Err(Error::ParseNumber(_))));
}

#[test]
fn several_numbers_in_one_word() {
    assert_eq!(
        scan_sentence("  1 2 3 _4.56 _99 __").unwrap(),
        &[Word::Noun(Noun::Array(Array::from([
            Complex64::new(1.0, 0.0),
            Complex64::new(2.0, 0.0),
            Complex64::new(3.0, 0.0),
            Complex64::new(-4.56, 0.0),
            Complex64::new(-99.0, 0.0),
            Complex64::new(f64::NEG_INFINITY, 0.0),
        ])))]
    );
}

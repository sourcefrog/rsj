// Copyright 2021 Martin Pool

//! Error type.

/// An error from the interpreter.
#[derive(Debug, PartialEq)]
pub enum Error {
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
    Domain,
}

pub type Result<T> = std::result::Result<T, Error>;

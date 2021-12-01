// Copyright 2021 Martin Pool

//! Error type.

/// An error from the interpreter.
#[derive(Debug)]
pub enum Error {
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
    Domain,
    /// J language feature that's not supported yet.
    Unimplemented,
    IoError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

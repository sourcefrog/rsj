// Copyright 2021 Martin Pool

//! Error type.

/// An error from the interpreter.
#[derive(Debug)]
pub enum Error {
    Unexpected(char),
    ParseNumber(num_complex::ParseComplexError<std::num::ParseFloatError>),
    Domain,
    /// J language feature that's not supported yet.
    Unimplemented(&'static str),
    IoError(std::io::Error),
    /// The arrays are not the same shape or length.
    Length,
    /// The operation would use too much memory.
    ///
    /// (Because of memory overcommit on Linux etc, we're not exactly
    /// "out", but it would be imprudent to continue.)
    OutOfMemory,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

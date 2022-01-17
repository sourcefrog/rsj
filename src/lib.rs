// Copyright 2021 Martin Pool

//! Toy implementation of J in Rust.

pub mod array;
pub mod atom;
pub mod error;
pub mod eval;
pub mod lex;
pub mod markdown;
pub mod noun;
pub mod primitive;
pub mod repl;
pub mod scan;
pub mod transcript;
pub mod verb;
pub mod word;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Error rather than allocating an array larger than this, so
/// that the machine's memory is not exhausted.
pub const ARRAY_SIZE_LIMIT: usize = 100_000_000;

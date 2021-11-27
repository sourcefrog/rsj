// Copyright 2021 Martin Pool

//! Toy implementation of J in Rust.

pub mod error;
pub mod eval;
pub mod lex;
pub mod noun;
pub mod repl;
pub mod verb;
pub mod word;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

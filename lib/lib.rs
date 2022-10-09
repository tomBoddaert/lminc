//! Assembly and simulation for the Little Minion Computer
//!  and Little Man Computer
//!
//! This crate includes a binary executable and a library
//!
//! Examples for the executable can be found in README.md.
//! Examples for the library can be found in the examples directory.

/// Assemble assembly and number lists
pub mod assembler;

/// Save and load assembled code
pub mod loader;

/// Run assembled code
pub mod runner;

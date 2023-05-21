//! Assembler and simulator for the Little Minion Computer
//!  and Little Man Computer

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Assemble assembly to memory
pub mod assembler;
/// Definitions for the assembly
pub mod assembly;
/// Run assembled code
pub mod computer;
/// Generic additions to errors
pub mod errors;
/// Save and load memory
pub mod file;
#[doc(hidden)]
pub mod helper;
/// Three digit numbers
pub mod num3;
/// Assemble numbers to memory
pub mod number_assembler;
/// Parse text to assembly
pub mod parser;
/// Run the computer and deal with input and output
pub mod runner;

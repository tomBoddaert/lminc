use std::{
    error,
    fmt::{self, Display},
    io,
};

use lminc::{
    assembler,
    errors::LineNumber,
    file::FromFileError,
    number_assembler, parser,
    runner::{stdio, tester::CSVErrorWithLineNumber},
};

#[derive(Debug)]
pub enum Error {
    Usage(String),
    FileError(io::Error),
    ParseError(parser::ErrorWithLocation<LineNumber>),
    AssemblerError(assembler::ErrorWithInstructionNumber),
    NumberAssemblerError(number_assembler::ErrorWithLineNumber),
    LoadError(FromFileError),
    RunnerError(stdio::Error),
    FromCSVError(CSVErrorWithLineNumber),
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(usage) => write!(f, "Usage: '{usage}'"),
            Self::FileError(error) => write!(f, "File error: {error}"),
            Self::ParseError(error) => write!(f, "Error parsing file: {error}"),
            Self::AssemblerError(error) => write!(f, "Error assembling file: {error}"),
            Self::NumberAssemblerError(error) => write!(f, "Error assembling number file: {error}"),
            Self::LoadError(error) => write!(f, "Error loading binary file: {error}"),
            Self::RunnerError(error) => fmt::Display::fmt(error, f),
            Self::FromCSVError(error) => write!(f, "Error reading CSV: {error}"),
            Self::Custom(message) => fmt::Display::fmt(message, f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::FileError(error) => Some(error),
            Self::ParseError(error) => Some(error),
            Self::AssemblerError(error) => Some(error),
            Self::NumberAssemblerError(error) => Some(error),
            Self::LoadError(error) => Some(error),
            Self::RunnerError(error) => Some(error),
            Self::FromCSVError(error) => Some(error),
            _ => None,
        }
    }
}

macro_rules! from_impl {
    ( $error:path, $variant:path ) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                $variant(value)
            }
        }
    };
}

from_impl!(io::Error, Self::FileError);
from_impl!(parser::ErrorWithLocation<LineNumber>, Self::ParseError);
from_impl!(assembler::ErrorWithInstructionNumber, Self::AssemblerError);
from_impl!(
    number_assembler::ErrorWithLineNumber,
    Self::NumberAssemblerError
);
from_impl!(FromFileError, Self::LoadError);
from_impl!(stdio::Error, Self::RunnerError);
from_impl!(CSVErrorWithLineNumber, Self::FromCSVError);
from_impl!(String, Self::Custom);

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_owned())
    }
}

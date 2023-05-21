use core::fmt;

use crate::{assembly, errors};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Parsing errors
pub enum Error {
    /// Too many words on one line (before any comments)
    TooManyWords,
    /// Too many instructions (maximum of 100)
    TooManyInstructions,
    /// Multiple instructions on one line (before any comments)
    MultipleInstructions,
    /// A number was found before an instruction (they cannot be used as labels)
    UnexpectedNumber,
    /// A line did not have an instruction but did have other non-space contents
    NoInstruction,
    /// See [assembly::Error]
    DataPresence(assembly::Error),
    /// The label was not found in the parsed assembly
    UnknownLabel,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyWords => write!(f, "Too many words!"),
            Self::TooManyInstructions => write!(f, "Too many instructions!"),
            Self::MultipleInstructions => write!(f, "Multiple instructions on one line!"),
            Self::UnexpectedNumber => write!(f, "Expected a label not a number!"),
            Self::NoInstruction => write!(f, "Missing instruction!"),
            Self::DataPresence(error) => write!(f, "{error}"),
            Self::UnknownLabel => write!(f, "Unknown label!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::DataPresence;

        match self {
            DataPresence(error) => Some(error),
            _ => None,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub type ErrorWithLocation<Location> = errors::ErrorWithLocation<Error, Location>;

impl From<assembly::Error> for Error {
    fn from(value: assembly::Error) -> Self {
        Self::DataPresence(value)
    }
}

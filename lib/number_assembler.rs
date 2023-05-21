use core::{fmt, num::ParseIntError};

use crate::{
    computer::Memory,
    errors::{ErrorWithLocation, LineNumber},
    num3::{ThreeDigitNumber, TryFromError},
};

/// Assemble from numbers
pub struct NumberAssembler {
    memory: Memory,
    index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Errors for assembling from numbers
pub enum FromNumbersError {
    TooManyNumbers,
    InvalidNumber(ParseIntError),
    TooLarge(TryFromError),
}

impl fmt::Display for FromNumbersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyNumbers => write!(f, "Too many numbers (> 99)!"),
            Self::InvalidNumber(error) => fmt::Display::fmt(error, f),
            Self::TooLarge(error) => fmt::Display::fmt(error, f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromNumbersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidNumber(error) => Some(error),
            Self::TooLarge(error) => Some(error),
            Self::TooManyNumbers => None,
        }
    }
}

pub type ErrorWithLineNumber = ErrorWithLocation<FromNumbersError, LineNumber>;

impl From<ParseIntError> for FromNumbersError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidNumber(value)
    }
}

impl From<TryFromError> for FromNumbersError {
    fn from(value: TryFromError) -> Self {
        Self::TooLarge(value)
    }
}

impl NumberAssembler {
    #[must_use]
    /// Create a new [`NumberAssembler`]
    pub const fn new() -> Self {
        Self {
            memory: [ThreeDigitNumber::ZERO; 100],
            index: 0,
        }
    }

    /// Assembles one line with up to one number, with comments
    ///
    /// # Errors
    /// See [`FromNumbersError`]
    pub fn assemble_line(&mut self, line: &str) -> Result<(), FromNumbersError> {
        // Make sure there is space for a number
        if self.index == 100 {
            return Err(FromNumbersError::TooManyNumbers);
        }

        // Get the part of the line before any comments
        let Some(code) = line.split(&['#', ';'][..]).next()
            .filter(|code| !code.is_empty()) else { return Ok(()) };

        // Try to parse as a u16 then try to convert to a three digit number
        let number: u16 = code.trim().parse()?;
        let number = ThreeDigitNumber::try_from(number)?;

        self.memory[self.index] = number;
        self.index += 1;

        Ok(())
    }

    /// Assembles from a list of numbers, with comments
    ///
    /// # Errors
    /// Returns a [`FromNumbersError`] with a [`LineNumber`].
    /// See [`FromNumbersError`] for possible errors
    pub fn assemble_from_text(text: &str) -> Result<Memory, ErrorWithLineNumber> {
        let mut assembler = Self::new();

        // Assemble each line
        text.lines()
            .enumerate()
            .try_for_each(|(line_number, line)| {
                assembler
                    .assemble_line(line)
                    // Add the line number as the error location
                    .map_err(|error| ErrorWithLocation(LineNumber(line_number + 1), error))
            })?;

        Ok(assembler.memory)
    }
}

#[cfg(test)]
mod test {
    use crate::number_assembler::NumberAssembler;

    #[test]
    fn empty_numbers() {
        let numbers = "";
        let memory = NumberAssembler::assemble_from_text(numbers).expect("failed to assemble");

        assert!(
            memory.iter().all(|num| u16::from(*num) == 0),
            "Could not assemble empty numbers!"
        );
    }

    #[test]
    fn full_numbers() {
        let numbers = "902\n".repeat(100);
        let memory = NumberAssembler::assemble_from_text(&numbers).expect("failed to assemble");

        assert!(
            memory.iter().all(|num| u16::from(*num) == 902),
            "Could not assemble full numbers!"
        );
    }

    #[test]
    fn fibonacci_numbers() {
        let numbers = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fib_num.txt"));
        let memory = NumberAssembler::assemble_from_text(numbers).expect("failed to assemble");

        let expected_memory: [u16; 100] = [
            605, 0, 1, 0, 100, 501, 102, 902, 303, 502, 301, 503, 302, 204, 816, 605, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert!(
            memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| u16::from(*number) == expected),
            "Could not assemble Fibonacci (numbers)!"
        );
    }
}

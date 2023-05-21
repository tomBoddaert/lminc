extern crate alloc;
use core::{fmt, num::ParseIntError};

use alloc::collections::linked_list::{IntoIter, LinkedList};

use crate::{
    errors::{self, LineNumber},
    num3::ThreeDigitNumber,
};

use super::Test;

#[cfg(feature = "extended")]
/// A test for programs using [`LinkedList`]s for the inputs and outputs
pub type StdTest<'a> = Test<
    'a,
    IntoIter<ThreeDigitNumber>,
    IntoIter<ThreeDigitNumber>,
    IntoIter<ThreeDigitNumber>,
    IntoIter<ThreeDigitNumber>,
>;

#[cfg(not(feature = "extended"))]
/// A test for programs using [`LinkedList`]s for the inputs and outputs
pub type StdTest<'a> = Test<'a, IntoIter<ThreeDigitNumber>, IntoIter<ThreeDigitNumber>>;

#[derive(Clone, Debug, PartialEq, Eq)]
/// CSV parsing errors
pub enum CSVError {
    /// A line did not have exactly 4 sections (or 6 with extended mode)
    NumberOfSections(usize),
    /// The max_cycles entry was not a valid number
    InvalidMaxCycles(ParseIntError),
    /// An input number was not a valid number
    InvalidInputNumber(ParseIntError),
    /// An input number was too large
    InputTooLarge(u16),
    /// An output number was not a valid number
    InvalidOutputNumber(ParseIntError),
    /// An output number was too large
    OutputTooLarge(u16),
    #[cfg(feature = "extended")]
    /// An input character was not a valid input character
    InvalidCharInput(char),
    #[cfg(feature = "extended")]
    /// An output character was not a valid output character
    InvalidCharOutput(char),
}

impl fmt::Display for CSVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(not(feature = "extended"))]
            Self::NumberOfSections(sections) => {
                write!(f, "Wrong number of sections ({sections}, should be 4)!")
            }
            #[cfg(feature = "extended")]
            Self::NumberOfSections(sections) => write!(
                f,
                "Wrong number of sections ({sections}, should be 4 or 6)!"
            ),
            Self::InvalidMaxCycles(_) => write!(f, "Invalid maximum number of cycles!"),
            Self::InvalidInputNumber(_) => write!(f, "Invalid input number!"),
            Self::InputTooLarge(number) => {
                write!(f, "Input number too large ({number} should be < 1000)!")
            }
            Self::InvalidOutputNumber(_) => write!(f, "Invalid output number!"),
            Self::OutputTooLarge(number) => {
                write!(f, "Output number too large ({number} should be < 1000)!")
            }
            #[cfg(feature = "extended")]
            Self::InvalidCharInput(character) => {
                write!(f, "Invalid input character ({character:?})!")
            }
            #[cfg(feature = "extended")]
            Self::InvalidCharOutput(character) => {
                write!(f, "Invalid output character ({character:?})!")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CSVError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidMaxCycles(error)
            | Self::InvalidInputNumber(error)
            | Self::InvalidOutputNumber(error) => Some(error),
            _ => None,
        }
    }
}

pub type CSVErrorWithLineNumber = errors::ErrorWithLocation<CSVError, LineNumber>;

impl<'a> StdTest<'a> {
    #[cfg_attr(
        not(feature = "extended"),
        doc = "Creates a new test from a line of csv in the format \n `name;comma separated inputs;comma separated outputs;maximum cycles`"
    )]
    #[cfg_attr(
        feature = "extended",
        doc = "Creates a new test from a line of csv in the format \n `name;comma separated inputs;comma separated outputs;[non-separated char inputs; non-separated char outputs;]maximum cycles`, where the contents of the `[..]` is optional"
    )]
    pub fn from_csv_line(text: &'a str) -> Result<Self, CSVError> {
        let mut sections = text.split(';');

        let number_of_sections = sections.clone().count();

        #[cfg(not(feature = "extended"))]
        if number_of_sections != 4 {
            return Err(CSVError::NumberOfSections(number_of_sections));
        }

        #[cfg(feature = "extended")]
        let char_io = match number_of_sections {
            4 => false,
            6 => true,
            _ => return Err(CSVError::NumberOfSections(number_of_sections)),
        };

        let name = sections
            .next()
            .ok_or(CSVError::NumberOfSections(number_of_sections))?;

        let inputs_str = sections
            .next()
            .ok_or(CSVError::NumberOfSections(number_of_sections))?;

        let outputs_str = sections
            .next()
            .ok_or(CSVError::NumberOfSections(number_of_sections))?;

        #[cfg(feature = "extended")]
        let (char_inputs_str, char_outputs_str) = if char_io {
            (
                Some(
                    sections
                        .next()
                        .ok_or(CSVError::NumberOfSections(number_of_sections))?,
                ),
                Some(
                    sections
                        .next()
                        .ok_or(CSVError::NumberOfSections(number_of_sections))?,
                ),
            )
        } else {
            (None, None)
        };

        let max_cycles = sections
            .next()
            .ok_or(CSVError::NumberOfSections(number_of_sections))?;

        let mut inputs = LinkedList::new();
        let mut outputs = LinkedList::new();
        #[cfg(feature = "extended")]
        let mut char_inputs = LinkedList::new();
        #[cfg(feature = "extended")]
        let mut char_outputs = LinkedList::new();

        for input in inputs_str.split(',').filter(|number| !number.is_empty()) {
            let number = input.parse::<u16>().map_err(CSVError::InvalidInputNumber)?;
            inputs.push_back(
                ThreeDigitNumber::try_from(number).map_err(|_| CSVError::InputTooLarge(number))?,
            );
        }

        for output in outputs_str.split(',').filter(|number| !number.is_empty()) {
            let number = output
                .parse::<u16>()
                .map_err(CSVError::InvalidOutputNumber)?;
            outputs.push_back(
                ThreeDigitNumber::try_from(number).map_err(|_| CSVError::OutputTooLarge(number))?,
            );
        }

        #[cfg(feature = "extended")]
        {
            if let Some(char_inputs_str) = char_inputs_str {
                for char_input in char_inputs_str.chars() {
                    char_inputs.push_back(
                        ThreeDigitNumber::try_from(
                            u16::try_from(char_input as u32)
                                .map_err(|_| CSVError::InvalidCharInput(char_input))?,
                        )
                        .map_err(|_| CSVError::InvalidCharInput(char_input))?,
                    );
                }
            }

            if let Some(char_outputs_str) = char_outputs_str {
                for char_output in char_outputs_str.chars() {
                    char_outputs.push_back(
                        ThreeDigitNumber::try_from(
                            u16::try_from(char_output as u32)
                                .map_err(|_| CSVError::InvalidCharInput(char_output))?,
                        )
                        .map_err(|_| CSVError::InvalidCharInput(char_output))?,
                    );
                }
            }
        }

        Ok(Self {
            name: if name.is_empty() { None } else { Some(name) },
            max_cycles: max_cycles.parse().map_err(CSVError::InvalidMaxCycles)?,
            inputs: inputs.into_iter(),
            outputs: outputs.into_iter(),
            #[cfg(feature = "extended")]
            char_inputs: char_inputs.into_iter(),
            #[cfg(feature = "extended")]
            char_outputs: char_outputs.into_iter(),
        })
    }

    /// Creates an iterator over tests from CSV text.
    /// See `from_csv_line` for format and errors
    ///
    /// # Errors
    /// Iterator can return a [`CSVError`] with a [`LineNumber`]
    pub fn from_csv(
        text: &'a str,
    ) -> impl Iterator<Item = Result<StdTest, CSVErrorWithLineNumber>> {
        text.lines().enumerate().map(|(line_number, line)| {
            Self::from_csv_line(line)
                .map_err(|error| errors::ErrorWithLocation(LineNumber(line_number + 1), error))
        })
    }
}

#[cfg(test)]
mod test {
    use core::assert_eq;

    use crate::{assembler::assemble_from_text, computer::Computer, num3::ThreeDigitNumber};

    use super::StdTest;

    #[test]
    fn csv_line_empty() {
        let line = ";;;1";

        let test = StdTest::from_csv_line(line).expect("failed to parse csv line");

        assert_eq!(test.name, None, "Got a name from CSV line!");

        assert_eq!(test.inputs.len(), 0, "Got too many inputs from CSV line!");

        assert_eq!(test.outputs.len(), 0, "Got too many outputs from CSV line!");

        #[cfg(feature = "extended")]
        assert_eq!(
            test.char_inputs.len(),
            0,
            "Got too many char inputs from CSV line!",
        );

        #[cfg(feature = "extended")]
        assert_eq!(
            test.char_outputs.len(),
            0,
            "Got too many char outputs from CSV line!",
        );

        assert_eq!(
            test.max_cycles, 1,
            "Failed to get the max cycles from CSV line!",
        );
    }

    #[test]
    fn csv_line() {
        let line = "name;1,2;3,4;5";

        let test = StdTest::from_csv_line(line).expect("failed to parse csv line");

        assert_eq!(test.name, Some("name"), "Failed to get name from CSV line!");

        let mut inputs = test.inputs;

        assert_eq!(
            inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(1) }),
            "Failed to get first input from CSV line!",
        );

        assert_eq!(
            inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(2) }),
            "Failed to get second input from CSV line!",
        );

        assert_eq!(inputs.next(), None, "Got too many inputs from CSV line!");

        let mut outputs = test.outputs;

        assert_eq!(
            outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(3) }),
            "Failed to get first output from CSV line!",
        );

        assert_eq!(
            outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(4) }),
            "Failed to get second output from CSV line!",
        );

        assert_eq!(outputs.next(), None, "Got too many outputs from CSV line!");

        #[cfg(feature = "extended")]
        assert_eq!(
            test.char_inputs.len(),
            0,
            "Got too many char inputs from CSV line!",
        );

        #[cfg(feature = "extended")]
        assert_eq!(
            test.char_outputs.len(),
            0,
            "Got too many char outputs from CSV line!",
        );

        assert_eq!(
            test.max_cycles, 5,
            "Failed to get the max cycles from CSV line!",
        );
    }

    #[cfg(feature = "extended")]
    #[test]
    fn csv_line_extended() {
        let line = "name;1,2;3,4;ab;cd;5";

        let test = StdTest::from_csv_line(line).expect("failed to parse csv line");

        assert_eq!(test.name, Some("name"), "Failed to get name from CSV line!");

        let mut inputs = test.inputs;

        assert_eq!(
            inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(1) }),
            "Failed to get first input from CSV line!",
        );

        assert_eq!(
            inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(2) }),
            "Failed to get second input from CSV line!",
        );

        assert_eq!(inputs.next(), None, "Got too many inputs from CSV line!");

        let mut outputs = test.outputs;

        assert_eq!(
            outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(3) }),
            "Failed to get first output from CSV line!",
        );

        assert_eq!(
            outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(4) }),
            "Failed to get second output from CSV line!",
        );

        assert_eq!(outputs.next(), None, "Got too many outputs from CSV line!");

        let mut char_inputs = test.char_inputs;

        assert_eq!(
            char_inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(b'a'.into()) }),
            "Failed to get first char input from CSV line!",
        );

        assert_eq!(
            char_inputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(b'b'.into()) }),
            "Failed to get second char input from CSV line!",
        );

        assert_eq!(
            char_inputs.next(),
            None,
            "Got too many char inputs from CSV line!"
        );

        let mut char_outputs = test.char_outputs;

        assert_eq!(
            char_outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(b'c'.into()) }),
            "Failed to get first char output from CSV line!",
        );

        assert_eq!(
            char_outputs.next(),
            Some(unsafe { ThreeDigitNumber::from_unchecked(b'd'.into()) }),
            "Failed to get second char output from CSV line!",
        );

        assert_eq!(
            char_outputs.next(),
            None,
            "Got too many char outputs from CSV line!"
        );

        assert_eq!(
            test.max_cycles, 5,
            "Failed to get the max cycles from CSV line!",
        );
    }

    #[test]
    fn run() {
        let assembly = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fib.txt"));
        let tests_csv = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/fib_test.csv"
        ));

        let memory = assemble_from_text(assembly)
            .expect("failed to parse the assembly")
            .expect("failed to assemble the assembly");

        let mut computer = Computer::new(memory);

        let mut tests = StdTest::from_csv(tests_csv);

        let test = tests
            .next()
            .expect("failed to get the test")
            .expect("failed to parse the test");

        test.run(&mut computer).expect("first test failed");

        assert!(tests.next().is_none(), "Got too many tests!");
    }
}

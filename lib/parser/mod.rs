use core::mem::MaybeUninit;

use crate::{
    assembly::{Instruction, InstructionWithLabel, NumberOrLabel},
    errors::{self, InstructionNumber, LineNumber},
    helper::try_collect_into_array,
    num3::ThreeDigitNumber,
};

mod error;
pub use error::*;

impl<'a> InstructionWithLabel<'a, NumberOrLabel<'a>> {
    /// Parse between 1 and 3 words as an instruction
    ///
    /// # Errors
    /// See [Error]
    pub fn parse(words: (&'a str, Option<&'a str>, Option<&'a str>)) -> Result<Self, Error> {
        let mut label: Option<&str> = None;
        let mut instruction: Option<Instruction<()>> = None;
        let mut data: Option<NumberOrLabel> = None;

        // The first word should be an instruction or a label
        let first = words.0;
        if let Ok(inst) = Instruction::try_from(first) {
            instruction = Some(inst);
        } else {
            // Make sure the first word is not a number
            let NumberOrLabel::Label(lab) = first.into() else { return Err(Error::UnexpectedNumber) };

            label = Some(lab);
        }

        // The second word should be an instruction or data
        if let Some(second) = words.1 {
            if let Ok(inst) = Instruction::try_from(second) {
                if instruction.replace(inst).is_some() {
                    // If there was already an instruction, return an error
                    return Err(Error::MultipleInstructions);
                }
            } else {
                data = Some(second.into());
            }
        }

        // The third word must be data
        if let Some(third) = words.2 {
            // Make sure it is not an instruction
            if Instruction::<()>::try_from(third).is_ok() {
                return Err(Error::MultipleInstructions);
            }

            if data.replace(third.into()).is_some() {
                // If there was already data, return an error
                return Err(Error::TooManyWords);
            }
        }

        Ok(instruction
            .ok_or(Error::NoInstruction)?
            .try_insert_data(data)?
            .add_label(label))
    }
}

#[derive(Clone, Copy, Debug)]
/// Parse assembly text
pub struct Parser<'a> {
    parsed: [MaybeUninit<InstructionWithLabel<'a, NumberOrLabel<'a>>>; 100],
    instruction_number: usize,
}

impl<'a> Parser<'a> {
    #[must_use]
    /// Create a new [Parser]
    pub const fn new() -> Self {
        Self {
            parsed: unsafe { MaybeUninit::uninit().assume_init() },
            instruction_number: 0,
        }
    }

    /// Get the number of instructions parsed
    pub const fn len(&self) -> usize {
        self.instruction_number
    }

    /// Parse one line of assembly into the [Parser]
    ///
    /// # Errors
    /// Returns an [Error] with a [`LineNumber`].
    /// See [Error] for possible errors
    pub fn parse_line(
        &mut self,
        line: &'a str,
    ) -> Result<(), ErrorWithLocation<InstructionNumber>> {
        // Get the part of the line before any comments
        let Some(code) = line.split(&['#', ';'][..]).next()
            .filter(|code| !code.is_empty()) else { return Ok(()) };

        // Split the code into words
        let words_iter = code.split_whitespace().filter(|word| !word.is_empty());
        // Collect the words into an array
        let words: [Option<&str>; 3] = try_collect_into_array(words_iter).map_err(|_| {
            errors::ErrorWithLocation(
                InstructionNumber(self.instruction_number + 1),
                Error::TooManyWords,
            )
        })?;

        // Make sure there is a first word
        let words = (
            if let Some(first) = words[0] {
                first
            } else {
                return Ok(());
            },
            words[1],
            words[2],
        );

        // Make sure there is space for an instruction
        if self.instruction_number == 100 {
            return Err(errors::ErrorWithLocation(
                InstructionNumber(self.instruction_number + 1),
                Error::TooManyInstructions,
            ));
        }

        // Parse the instruction
        let instruction = InstructionWithLabel::<NumberOrLabel>::parse(words).map_err(|error| {
            errors::ErrorWithLocation(InstructionNumber(self.instruction_number + 1), error)
        })?;

        // Write the instruction
        self.parsed[self.instruction_number].write(instruction);
        self.instruction_number += 1;

        Ok(())
    }

    /// Parse assembly into a [Parser]
    ///
    /// # Errors
    /// Returns an [Error] with a [`LineNumber`].
    /// See [Error] for possible errors
    pub fn parse_text(text: &'a str) -> Result<Self, ErrorWithLocation<LineNumber>> {
        let mut parser = Self::new();

        // Parse each line
        text.lines()
            .enumerate()
            .try_for_each(|(line_number, line)| {
                parser
                    .parse_line(line)
                    // Add the line number as the error location
                    .map_err(|error| {
                        errors::ErrorWithLocation(LineNumber(line_number + 1), error.1)
                    })
            })?;

        Ok(parser)
    }

    /// Get the memory address for a label from the [Parser]
    ///
    /// # Errors
    /// See [`Error::UnknownLabel`]
    pub fn resolve_label(&self, label: &str) -> Result<ThreeDigitNumber, Error> {
        self.iter()
            .enumerate()
            .find_map(|(index, instruction)| {
                if instruction.label? == label {
                    #[allow(clippy::cast_possible_truncation)]
                    Some(unsafe { ThreeDigitNumber::from_unchecked(index as u16) })
                } else {
                    None
                }
            })
            .ok_or(Error::UnknownLabel)
    }

    #[must_use]
    /// Create an iterator over the parsed instructions in the [Parser]
    pub const fn iter(&'a self) -> ParsedIter<'a> {
        ParsedIter {
            parser: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for Parser<'a> {
    type Item = InstructionWithLabel<'a, NumberOrLabel<'a>>;
    type IntoIter = ParsedIntoIter<'a>;

    /// Convert the [Parser] into an iterator
    fn into_iter(self) -> Self::IntoIter {
        ParsedIntoIter {
            parser: self,
            index: 0,
        }
    }
}

#[derive(Clone, Debug)]
/// An iterator over the parsed instructions in a [Parser]
pub struct ParsedIter<'a> {
    parser: &'a Parser<'a>,
    index: usize,
}

impl<'a> Iterator for ParsedIter<'a> {
    type Item = &'a InstructionWithLabel<'a, NumberOrLabel<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.parser.instruction_number {
            return None;
        }

        let next = unsafe { self.parser.parsed[self.index].assume_init_ref() };
        self.index += 1;

        Some(next)
    }
}

#[derive(Clone, Debug)]
/// An iterator over the parsed instructions from a [Parser]
pub struct ParsedIntoIter<'a> {
    parser: Parser<'a>,
    index: usize,
}

impl<'a> Iterator for ParsedIntoIter<'a> {
    type Item = InstructionWithLabel<'a, NumberOrLabel<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.parser.instruction_number {
            return None;
        }

        let next = unsafe { self.parser.parsed[self.index].assume_init() };
        self.index += 1;

        Some(next)
    }
}

#[cfg(test)]
mod test {
    use core::assert_eq;

    use crate::num3::ThreeDigitNumber;

    use super::Parser;

    #[test]
    fn parse() {
        let assembly = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fib.txt"));

        let parser = Parser::parse_text(assembly).expect("failed to parse assembly");

        assert_eq!(
            parser
                .resolve_label("start")
                .expect("failed to resolve 'start' label"),
            ThreeDigitNumber::ZERO,
            "Failed to resolve the 'start' label correctly!"
        );

        assert_eq!(
            parser.len(),
            16,
            "Failed to parse the correct number of instructions!"
        );
    }
}

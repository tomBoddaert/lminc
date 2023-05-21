use core::fmt;

use crate::{
    assembly::{Instruction, NumberOrLabel},
    computer::Memory,
    errors::{self, InstructionNumber, LineNumber},
    num3::ThreeDigitNumber,
    parser::{self, Parser},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Assembler Errors
pub enum Error {
    /// Failed to resolve a label
    LabelResolve(parser::Error),
    /// An address was too large (> 99)
    AddressTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LabelResolve(error) => fmt::Display::fmt(error, f),
            Self::AddressTooLarge => write!(f, "Address is too large (> 99)!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LabelResolve(error) => Some(error),
            Self::AddressTooLarge => None,
        }
    }
}

pub type ErrorWithInstructionNumber = errors::ErrorWithLocation<Error, InstructionNumber>;

impl From<parser::Error> for Error {
    fn from(value: parser::Error) -> Self {
        Self::LabelResolve(value)
    }
}

/// Assemble one parsed instruction
///
/// # Errors
/// See [Error]
pub fn assemble_instruction(
    instruction: Instruction<NumberOrLabel>,
    parser: &Parser,
) -> Result<ThreeDigitNumber, Error> {
    let op_code = {
        let op_code = instruction.op_code();
        if u16::from(op_code) == 1 {
            0
        } else {
            op_code.into()
        }
    };

    Ok(unsafe {
        ThreeDigitNumber::from_unchecked(match instruction {
            Instruction::ADD(data)
            | Instruction::SUB(data)
            | Instruction::STO(data)
            | Instruction::LDA(data)
            | Instruction::BR(data)
            | Instruction::BRZ(data)
            | Instruction::BRP(data) => {
                let data = match data {
                    NumberOrLabel::Label(label) => parser.resolve_label(label)?,
                    NumberOrLabel::Number(number) => {
                        if number.is_2_digit() {
                            number
                        } else {
                            return Err(Error::AddressTooLarge);
                        }
                    }
                };

                op_code + u16::from(data)
            }

            Instruction::IN | Instruction::OUT | Instruction::HLT => op_code,

            #[cfg(feature = "extended")]
            Instruction::INA | Instruction::OUTA | Instruction::EXT => op_code,

            Instruction::DAT(data) => {
                let data: ThreeDigitNumber = match data {
                    NumberOrLabel::Label(label) => parser.resolve_label(label)?,
                    NumberOrLabel::Number(number) => number,
                };

                op_code + u16::from(data)
            }
        })
    })
}

/// Assemble from parsed assembly
///
/// # Errors
/// See [Error]
pub fn assemble_from_parser(parser: Parser) -> Result<Memory, ErrorWithInstructionNumber> {
    let mut memory: Memory = [ThreeDigitNumber::ZERO; 100];

    parser
        .iter()
        .enumerate()
        .try_for_each(|(index, instruction)| {
            memory[index] = assemble_instruction(instruction.instruction, &parser)
                .map_err(|error| errors::ErrorWithLocation(InstructionNumber(index + 1), error))?;
            Ok::<(), ErrorWithInstructionNumber>(())
        })?;

    Ok(memory)
}

/// Assemble from assembly text, with comments
///
/// # Errors
/// See [`parser::Error`] and [Error]
pub fn assemble_from_text(
    text: &str,
) -> Result<Result<Memory, ErrorWithInstructionNumber>, parser::ErrorWithLocation<LineNumber>> {
    let parser = parser::Parser::parse_text(text)?;
    Ok(assemble_from_parser(parser))
}

#[cfg(test)]
mod test {
    use core::mem;

    use super::*;

    #[test]
    fn empty_assembly() {
        let assembly = "";
        let memory = assemble_from_text(assembly)
            .expect("failed to parse")
            .expect("failed to assemble");

        assert!(
            memory.iter().all(|number| u16::from(*number) == 0),
            "Could not assemble empty assembly!"
        );
    }

    #[test]
    fn full_assembly() {
        let assembly = "OUT\n".repeat(100);
        let memory = assemble_from_text(&assembly)
            .expect("failed to parse")
            .expect("failed to assemble");

        assert!(
            memory.iter().all(|number| u16::from(*number) == 902),
            "Could not assemble full assembly!"
        );
    }

    #[test]
    fn fibonacci_assembly() {
        let assembly = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/fib.txt"));
        let memory = assemble_from_text(assembly)
            .expect("failed to parse")
            .expect("failed to assemble");
        let expected_memory: [u16; 100] = [
            512, 113, 902, 314, 513, 312, 514, 313, 515, 214, 800, 0, 0, 1, 0, 100, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let u16_memory: [u16; 100] = unsafe { mem::transmute(memory) };

        assert_eq!(u16_memory, expected_memory, "Failed to assemble Fibonacci!");
    }

    #[test]
    fn absolute_address_assembly() {
        let assembly = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/abs_addr.txt"
        ));
        let memory = assemble_from_text(assembly).unwrap().unwrap();
        let expected_memory: [u16; 100] = [
            509, 398, 109, 399, 598, 902, 599, 902, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert!(
            memory
                .iter()
                .zip(expected_memory)
                .all(|(number, expected)| u16::from(*number) == expected),
            "Could not assemble assembly with absolute address!"
        );
    }
}

use core::{
    fmt::{self, Display},
    num::ParseIntError,
};
use std::io::{self, stdin, stdout, BufRead, Write};

use crate::{
    computer::{Computer, Memory, State},
    num3::{self, ThreeDigitNumber},
};

#[derive(Debug)]
/// The error for [Runner]
pub enum Error {
    /// An io error occurred
    IoError(io::Error),
    /// The input was not a valid number
    ParseError(ParseIntError),
    /// The inputted number was too large
    TooLarge(num3::TryFromError),
    #[cfg(feature = "extended")]
    /// Multiple characters were inputted
    MultipleCharacters,
    #[cfg(feature = "extended")]
    /// The inputted character is not a valid input character
    InvalidInputCharacter,
    #[cfg(feature = "extended")]
    /// The outputted character is not a valid character
    InvalidOutputCharacter(ThreeDigitNumber),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(error) => write!(f, "Io error: {error}"),
            Self::ParseError(_) => write!(f, "Invalid number inputted!"),
            Self::TooLarge(_) => write!(f, "Inputted number is too large (> 999)!"),
            #[cfg(feature = "extended")]
            Self::MultipleCharacters => write!(f, "Multiple characters inputted!"),
            #[cfg(feature = "extended")]
            Self::InvalidInputCharacter => write!(f, "Invalid input character"),
            #[cfg(feature = "extended")]
            Self::InvalidOutputCharacter(number) => {
                write!(f, "Invalid character outputted: {number}!")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

impl From<num3::TryFromError> for Error {
    fn from(value: num3::TryFromError) -> Self {
        Self::TooLarge(value)
    }
}

/// A runner that uses stdio for inputs and outputs
pub struct Runner {
    computer: Computer,
    #[cfg(feature = "extended")]
    mid_char_sequence: bool,
}

impl Runner {
    #[must_use]
    /// Create a new [Runner] from [Memory]
    pub const fn new(memory: Memory) -> Self {
        Self {
            computer: Computer::new(memory),
            #[cfg(feature = "extended")]
            mid_char_sequence: false,
        }
    }

    #[must_use]
    /// Create a new [Runner] from a [Computer]
    pub const fn new_from_computer(computer: Computer) -> Self {
        Self {
            computer,
            #[cfg(feature = "extended")]
            mid_char_sequence: false,
        }
    }

    /// Step the computer, using stdio for inputs and outputs
    ///
    /// # Errors
    /// See [Error]
    pub fn step(&mut self) -> Result<State, Error> {
        match self.computer.step() {
            State::AwaitingInput => {
                #[cfg(feature = "extended")]
                if self.mid_char_sequence {
                    println!();
                    self.mid_char_sequence = false;
                }

                #[cfg(not(feature = "extended"))]
                print!("> ");
                #[cfg(feature = "extended")]
                print!("(i) > ");
                stdout().flush()?;

                let mut buffer = String::with_capacity(4);
                stdin().lock().read_line(&mut buffer)?;

                let num: ThreeDigitNumber = buffer.trim().parse::<u16>()?.try_into()?;

                self.computer
                    .input(num)
                    .expect("failed to give an input to a computer");
            }
            State::AwaitingOutput => {
                #[cfg(feature = "extended")]
                if self.mid_char_sequence {
                    println!();
                    self.mid_char_sequence = false;
                }

                let output: u16 = self
                    .computer
                    .output()
                    .expect("failed to get an output from a computer")
                    .into();
                println!("{output}");
            }
            #[cfg(feature = "extended")]
            State::AwaitingCharInput => {
                #[cfg(feature = "extended")]
                if self.mid_char_sequence {
                    println!();
                    self.mid_char_sequence = false;
                }

                print!("(c) > ");
                stdout().flush()?;

                let mut buffer = String::with_capacity(2);
                stdin().lock().read_line(&mut buffer)?;

                let mut chars = buffer.chars();

                let character = chars.next().unwrap_or('\n');

                let after: String = chars.collect();
                if !after.trim().is_empty() {
                    return Err(Error::MultipleCharacters);
                }

                let num = character as u32;
                if num >= 1000 {
                    return Err(Error::InvalidInputCharacter);
                }

                #[allow(clippy::cast_possible_truncation)]
                let num = unsafe { ThreeDigitNumber::from_unchecked(num as u16) };

                self.computer
                    .input_char(num)
                    .expect("failed to give a char input to a computer");
            }
            #[cfg(feature = "extended")]
            State::AwaitingCharOutput => {
                let num = self
                    .computer
                    .output_char()
                    .expect("failed to get a char output from a computer");

                let char = char::from_u32(u32::from(u16::from(num)))
                    .ok_or(Error::InvalidOutputCharacter(num))?;
                print!("{char}");

                #[cfg(feature = "extended")]
                if char == '\n' {
                    self.mid_char_sequence = false;
                }
            }
            _ => (),
        }

        Ok(self.computer.state())
    }

    /// Run the computer until a halt or error state is reached
    ///
    /// # Errors
    /// See [Error]
    pub fn run(&mut self) -> Result<State, Error> {
        loop {
            match self.step()? {
                State::Running => (),
                state => {
                    #[cfg(feature = "extended")]
                    if self.mid_char_sequence {
                        println!();
                        self.mid_char_sequence = false;
                    }
                    return Ok(state);
                }
            }
        }
    }
}

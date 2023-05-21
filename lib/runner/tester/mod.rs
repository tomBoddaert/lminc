use core::fmt;

use crate::{
    computer::{Computer, State},
    errors::ErrorWithLocation,
    num3::ThreeDigitNumber,
};

#[cfg(feature = "alloc")]
mod alloc_tester;
#[cfg(feature = "alloc")]
pub use alloc_tester::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Tests for programs
pub struct Test<
    'a,
    Inputs: Iterator<Item = ThreeDigitNumber>,
    Outputs: Iterator<Item = ThreeDigitNumber>,
    #[cfg(feature = "extended")] AInputs: Iterator<Item = ThreeDigitNumber>,
    #[cfg(feature = "extended")] AOutputs: Iterator<Item = ThreeDigitNumber>,
> {
    pub name: Option<&'a str>,
    pub max_cycles: u32,
    pub inputs: Inputs,
    pub outputs: Outputs,
    #[cfg(feature = "extended")]
    pub char_inputs: AInputs,
    #[cfg(feature = "extended")]
    pub char_outputs: AOutputs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Errors for tests
pub enum TestError {
    /// The number of cycles exceeded `max_cycles`
    RunOutOfCycles,

    /// The computer requested more inputs than expected
    RunOutOfInputs,
    /// The computer gave more outputs than expected
    RunOutOfOutputs(ThreeDigitNumber),
    #[cfg(feature = "extended")]
    /// The computer requested more char inputs than expected
    RunOutOfCharInputs,
    #[cfg(feature = "extended")]
    /// The computer gave more char outputs than expected
    RunOutOfCharOutputs(ThreeDigitNumber, Option<char>),

    /// An output from the computer did not match the expected output
    DifferentOutput {
        expected: ThreeDigitNumber,
        got: ThreeDigitNumber,
    },
    #[cfg(feature = "extended")]
    /// A char output from the computer did not match the expected char output
    DifferentCharOutput {
        expected: ThreeDigitNumber,
        expected_char: Option<char>,
        got: ThreeDigitNumber,
        got_char: Option<char>,
    },

    /// The computer requested less inputs than expected
    ExpectedMoreInputs,
    /// The computer gave less outputs than expected
    ExpectedMoreOutputs,
    #[cfg(feature = "extended")]
    /// The computer requested less char inputs than expected
    ExpectedMoreCharInputs,
    #[cfg(feature = "extended")]
    /// The computer gave less char outputs than expected
    ExpectedMoreCharOutputs,

    /// The computer errored
    ComputerError(State),
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RunOutOfCycles => write!(f, "Ran out of cycles!"),

            Self::RunOutOfInputs => write!(f, "Requested more inputs than expected!"),
            Self::RunOutOfOutputs(output) => {
                write!(f, "Gave more outputs than expected (output: {output})!")
            }
            #[cfg(feature = "extended")]
            Self::RunOutOfCharInputs => write!(f, "Requested more char inputs than expected!"),
            #[cfg(feature = "extended")]
            Self::RunOutOfCharOutputs(output_number, output_char) => {
                write!(
                    f,
                    "Gave more char outputs than expected (output: {output_number}"
                )?;
                if let Some(character) = output_char {
                    write!(f, " = {character:?}")?;
                }
                write!(f, ")!")
            }

            Self::DifferentOutput { expected, got } => {
                write!(
                    f,
                    "Different output than expected (expected {expected}, got {got})"
                )
            }
            #[cfg(feature = "extended")]
            Self::DifferentCharOutput {
                expected,
                expected_char,
                got,
                got_char,
            } => {
                write!(f, "Different output than expected (expected {expected}",)?;
                if let Some(character) = expected_char {
                    write!(f, " = {character:?}")?;
                }

                write!(f, ", got {got}")?;
                if let Some(character) = got_char {
                    write!(f, " = {character:?}")?;
                }

                write!(f, ")!")
            }

            Self::ExpectedMoreInputs => write!(f, "Expected more inputs!"),
            Self::ExpectedMoreOutputs => write!(f, "Expected more outputs!"),
            #[cfg(feature = "extended")]
            Self::ExpectedMoreCharInputs => write!(f, "Expected more char inputs!"),
            #[cfg(feature = "extended")]
            Self::ExpectedMoreCharOutputs => write!(f, "Expected more char outputs!"),

            Self::ComputerError(state) => write!(f, "Computer error: {state:?}!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TestError {}

crate::create_location_type!(
    "A number of cycles for use with [ErrorWithLocation]":
    AfterCycles(pub u32): number => "after {} cycles", number.0
);

pub type ErrorWithCycles = ErrorWithLocation<TestError, AfterCycles>;

crate::create_location_type!(
    "A test name for use with [ErrorWithLocation]":
    TestName<'a,>(pub &'a str): name => "test {}", name.0
);

pub type ErrorWithOptionalTestName<'a> = ErrorWithLocation<ErrorWithCycles, Option<TestName<'a>>>;

macro_rules! test_methods {
    () => {
        /// Run one step of a test.
        /// Only use this if you know what you are doing!
        /// You probably want `run` instead
        ///
        /// # Errors
        /// See [`TestError`]
        pub fn step(
            computer: &mut Computer,
            test: &mut Self,
            cycles: &mut u32,
        ) -> Result<bool, ErrorWithOptionalTestName<'a>> {
            if *cycles == test.max_cycles {
                return Err(ErrorWithLocation(
                    test.name.map(TestName),
                    ErrorWithLocation(AfterCycles(*cycles), TestError::RunOutOfCycles),
                ));
            }

            let done = match computer.step() {
                State::Running => false,

                State::AwaitingInput => {
                    let input = test.inputs.next().ok_or_else(|| {
                        ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(AfterCycles(*cycles), TestError::RunOutOfInputs),
                        )
                    })?;

                    computer
                        .input(input)
                        .expect("failed to give an input to a computer");

                    false
                }

                State::AwaitingOutput => {
                    let output = computer
                        .output()
                        .expect("failed to get an output from a computer");

                    let expected = test.outputs.next().ok_or_else(|| {
                        ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(
                                AfterCycles(*cycles),
                                TestError::RunOutOfOutputs(output),
                            ),
                        )
                    })?;

                    if output != expected {
                        return Err(ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(
                                AfterCycles(*cycles),
                                TestError::DifferentOutput {
                                    expected,
                                    got: output,
                                },
                            ),
                        ));
                    }

                    false
                }

                #[cfg(feature = "extended")]
                State::AwaitingCharInput => {
                    let input = test.char_inputs.next().ok_or_else(|| {
                        ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(AfterCycles(*cycles), TestError::RunOutOfCharInputs),
                        )
                    })?;

                    computer
                        .input_char(input)
                        .expect("failed to give a char input to a computer");

                    false
                }

                #[cfg(feature = "extended")]
                State::AwaitingCharOutput => {
                    let output = computer
                        .output_char()
                        .expect("failed to get a char output from a computer");

                    let expected = test.char_outputs.next().ok_or_else(|| {
                        ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(
                                AfterCycles(*cycles),
                                TestError::RunOutOfCharOutputs(
                                    output,
                                    char::from_u32(u16::from(output).into()),
                                ),
                            ),
                        )
                    })?;

                    if output != expected {
                        return Err(ErrorWithLocation(
                            test.name.map(TestName),
                            ErrorWithLocation(
                                AfterCycles(*cycles),
                                TestError::DifferentCharOutput {
                                    expected,
                                    expected_char: char::from_u32(u16::from(expected).into()),
                                    got: output,
                                    got_char: char::from_u32(u16::from(output).into()),
                                },
                            ),
                        ));
                    }

                    false
                }

                State::Halted | State::ReachedEnd => true,

                state => {
                    return Err(ErrorWithLocation(
                        test.name.map(TestName),
                        ErrorWithLocation(AfterCycles(*cycles), TestError::ComputerError(state)),
                    ))
                }
            };

            *cycles += 1;

            Ok(done)
        }

        /// Run the test with the given memory
        ///
        /// # Errors
        /// See [`TestError`]
        pub fn run(
            mut self,
            computer: &mut Computer,
        ) -> Result<u32, ErrorWithOptionalTestName<'a>> {
            let mut cycles = 0;

            while !Self::step(computer, &mut self, &mut cycles)? {}

            // Make sure all the inputs and outputs were used

            if self.inputs.next().is_some() {
                return Err(ErrorWithLocation(
                    self.name.map(TestName),
                    ErrorWithLocation(AfterCycles(cycles), TestError::ExpectedMoreInputs),
                ));
            }

            if self.outputs.next().is_some() {
                return Err(ErrorWithLocation(
                    self.name.map(TestName),
                    ErrorWithLocation(AfterCycles(cycles), TestError::ExpectedMoreOutputs),
                ));
            }

            #[cfg(feature = "extended")]
            if self.char_inputs.next().is_some() {
                return Err(ErrorWithLocation(
                    self.name.map(TestName),
                    ErrorWithLocation(AfterCycles(cycles), TestError::ExpectedMoreCharInputs),
                ));
            }

            #[cfg(feature = "extended")]
            if self.char_outputs.next().is_some() {
                return Err(ErrorWithLocation(
                    self.name.map(TestName),
                    ErrorWithLocation(AfterCycles(cycles), TestError::ExpectedMoreCharOutputs),
                ));
            }

            Ok(cycles)
        }
    };
}

#[cfg(not(feature = "extended"))]
impl<'a, Inputs: Iterator<Item = ThreeDigitNumber>, Outputs: Iterator<Item = ThreeDigitNumber>>
    Test<'a, Inputs, Outputs>
{
    test_methods!();
}

#[cfg(feature = "extended")]
impl<
        'a,
        Inputs: Iterator<Item = ThreeDigitNumber>,
        Outputs: Iterator<Item = ThreeDigitNumber>,
        AInputs: Iterator<Item = ThreeDigitNumber>,
        AOutputs: Iterator<Item = ThreeDigitNumber>,
    > Test<'a, Inputs, Outputs, AInputs, AOutputs>
{
    test_methods!();
}

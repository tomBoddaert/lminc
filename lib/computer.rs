use core::fmt;

use crate::num3::ThreeDigitNumber;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The computer that runs programs
pub struct Computer {
    state: State,
    memory: Memory,
    counter: usize,
    register: ThreeDigitNumber,
    negative_flag: bool,
    #[cfg(feature = "extended")]
    extended_mode_flag: bool,
}

pub type Memory = [ThreeDigitNumber; 100];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// The states for [Computer]s
pub enum State {
    #[default]
    Running,
    AwaitingInput,
    AwaitingOutput,
    #[cfg(feature = "extended")]
    AwaitingCharInput,
    #[cfg(feature = "extended")]
    AwaitingCharOutput,
    Halted,
    ReachedEnd,
    InvalidInstruction,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "is running"),
            Self::AwaitingInput => write!(f, "is awaiting input"),
            Self::AwaitingOutput => write!(f, "is awaiting output"),
            #[cfg(feature = "extended")]
            Self::AwaitingCharInput => write!(f, "is awaiting char input"),
            #[cfg(feature = "extended")]
            Self::AwaitingCharOutput => write!(f, "is awaiting char output"),
            Self::Halted => write!(f, "halted"),
            Self::ReachedEnd => write!(f, "reached the end of its memory"),
            Self::InvalidInstruction => write!(f, "reached an invalid instruction"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Errors for [Computer] Io
pub enum Error {
    /// The computer was not waiting for an input, but one was given
    UnexpectedInput,
    /// The computer was not waiting to output, but one was requested
    NoOutput,
    #[cfg(feature = "extended")]
    /// The computer was not waiting for a char input, but one was given
    UnexpectedCharInput,
    #[cfg(feature = "extended")]
    /// The computer was not waiting to output a char, but one was requested
    NoCharOutput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedInput => write!(
                f,
                "The computer was not waiting for an input, but received one!"
            ),
            Self::NoOutput => write!(
                f,
                "The computer was not waiting to output, but one was requested!"
            ),
            #[cfg(feature = "extended")]
            Self::UnexpectedCharInput => write!(
                f,
                "The computer was not waiting for a char input, but received one!"
            ),
            #[cfg(feature = "extended")]
            Self::NoCharOutput => write!(
                f,
                "The computer was not waiting to output a char, but one was requested!"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Errors for `set_counter`
pub enum SetCounterError {
    /// The given value is too large (> 99)
    TooLarge,
}

impl fmt::Display for SetCounterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge => write!(f, "The given value for the counter was too large (> 99)!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SetCounterError {}

impl Computer {
    #[must_use]
    /// Create a new [Computer] from [Memory]
    pub const fn new(memory: Memory) -> Self {
        Self {
            state: State::Running,
            memory,
            counter: 0,
            register: ThreeDigitNumber::ZERO,
            negative_flag: false,
            #[cfg(feature = "extended")]
            extended_mode_flag: false,
        }
    }

    /// Run one instruction on the computer
    pub fn step(&mut self) -> State {
        if self.state != State::Running {
            return self.state;
        }

        if self.counter == 100 {
            self.state = State::ReachedEnd;
            return self.state;
        }

        let instruction = u16::from(self.memory[self.counter]);
        let op_code = instruction / 100;
        let data = instruction % 100;

        match op_code {
            // ADD
            1 => {
                self.register += self.memory[data as usize];
            }
            // SUB
            2 => {
                let (register, negative_flag) = self.register - self.memory[data as usize];
                self.register = register;
                self.negative_flag = negative_flag;
            }
            // STO
            3 => {
                self.memory[data as usize] = self.register;
            }
            // LDA
            5 => {
                self.register = self.memory[data as usize];
            }
            // BR
            6 => {
                self.counter = data as usize;
                return self.state;
            }
            // BRZ
            7 => {
                if self.register == ThreeDigitNumber::ZERO {
                    self.counter = data as usize;
                    return self.state;
                }
            }
            // BRP
            8 => {
                if !self.negative_flag {
                    self.counter = data as usize;
                    return self.state;
                }
            }
            // IO
            9 => {
                match data {
                    // IN
                    1 => {
                        self.state = State::AwaitingInput;
                    }
                    // OUT
                    2 => {
                        self.state = State::AwaitingOutput;
                    }
                    // INA
                    #[cfg(feature = "extended")]
                    11 if self.extended_mode_flag => {
                        self.state = State::AwaitingCharInput;
                    }
                    // OUTA
                    #[cfg(feature = "extended")]
                    12 if self.extended_mode_flag => {
                        self.state = State::AwaitingCharOutput;
                    }
                    // Invalid IO Operation
                    _ => {
                        self.state = State::InvalidInstruction;
                        return self.state;
                    }
                }
            }
            // HLT
            0 => {
                #[cfg(feature = "extended")]
                if data == 10 {
                    self.extended_mode_flag = true;
                } else {
                    self.state = State::Halted;
                }
                #[cfg(not(feature = "extended"))]
                {
                    self.state = State::Halted;
                }
            }
            // Invalid Instruction
            _ => {
                self.state = State::InvalidInstruction;
                return self.state;
            }
        }

        self.counter += 1;
        self.state
    }

    /// Run the [Computer] until its state is not [`State::Running`]
    pub fn run(&mut self) -> State {
        while self.step() != State::Running {}
        self.state
    }

    /// Give an input to the [Computer]
    ///
    /// # Errors
    /// [`Error::UnexpectedInput`] - the computer is not awaiting an input
    pub fn input(&mut self, input: ThreeDigitNumber) -> Result<(), Error> {
        if self.state == State::AwaitingInput {
            self.register = input;
            self.state = State::Running;
            Ok(())
        } else {
            Err(Error::UnexpectedInput)
        }
    }

    /// Take an output from the [Computer]
    ///
    /// # Errors
    /// [`Error::NoOutput`] - there is no output to be taken
    pub fn output(&mut self) -> Result<ThreeDigitNumber, Error> {
        if self.state == State::AwaitingOutput {
            self.state = State::Running;
            Ok(self.register)
        } else {
            Err(Error::NoOutput)
        }
    }

    #[cfg(feature = "extended")]
    /// Give a char input to the [Computer]
    ///
    /// # Errors
    /// [`Error::UnexpectedCharInput`] - the computer is not awaiting a char input
    pub fn input_char(&mut self, input: ThreeDigitNumber) -> Result<(), Error> {
        if self.state == State::AwaitingCharInput {
            self.register = input;
            self.state = State::Running;
            Ok(())
        } else {
            Err(Error::UnexpectedCharInput)
        }
    }

    #[cfg(feature = "extended")]
    /// Take a char output from the [Computer]
    ///
    /// # Errors
    /// [`Error::NoCharOutput`] - there is no char output to be taken
    pub fn output_char(&mut self) -> Result<ThreeDigitNumber, Error> {
        if self.state == State::AwaitingCharOutput {
            self.state = State::Running;
            Ok(self.register)
        } else {
            Err(Error::NoCharOutput)
        }
    }

    // Functions that take `computer` rather than `self` are
    //  "hidden" functions of the computer, they are not intended
    //  for normal use.

    #[must_use]
    /// Get the [Computer]'s current [State]
    pub const fn state(&self) -> State {
        self.state
    }

    /// Set a [Computer]'s [State]
    pub fn set_state(computer: &mut Self, value: State) {
        computer.state = value;
    }

    /// Reset the [Computer] without resetting the [Memory]
    pub fn reset(&mut self) {
        self.state = State::Running;
        self.counter = 0;
        self.register = ThreeDigitNumber::ZERO;
        self.negative_flag = false;
        #[cfg(feature = "extended")]
        {
            self.extended_mode_flag = false;
        }
    }

    #[must_use]
    /// Get the [Computer]'s [Memory]
    pub const fn get_memory(&self) -> &Memory {
        &self.memory
    }

    /// Mutably get a [Computer]'s [Memory]
    pub fn get_memory_mut(computer: &mut Self) -> &mut Memory {
        &mut computer.memory
    }

    #[must_use]
    /// Get the [Computer]'s counter
    pub const fn counter(&self) -> usize {
        self.counter
    }

    /// Set a [Computer]'s counter
    ///
    /// # Errors
    /// See [`SetCounterError`]
    pub fn set_counter(computer: &mut Self, value: usize) -> Result<(), SetCounterError> {
        if value > 100 {
            Err(SetCounterError::TooLarge)
        } else {
            computer.counter = value;
            Ok(())
        }
    }

    #[must_use]
    /// Get the [Computer]'s register
    pub const fn register(&self) -> ThreeDigitNumber {
        self.register
    }

    /// Set a [Computer]'s register
    pub fn set_register(computer: &mut Self, value: ThreeDigitNumber) {
        computer.register = value;
    }

    #[must_use]
    /// Get the [Computer]'s negative flag
    pub const fn negative_flag(&self) -> bool {
        self.negative_flag
    }

    /// Set a [Computer]'s negative flag
    pub fn set_negative_flag(computer: &mut Self, value: bool) {
        computer.negative_flag = value;
    }

    #[cfg(feature = "extended")]
    #[must_use]
    /// Get the [Computer]'s extended mode flag
    pub const fn extended_mode_flag(&self) -> bool {
        self.extended_mode_flag
    }

    #[cfg(feature = "extended")]
    /// Set a [Computer]'s extended mode flag
    pub fn set_extended_mode_flag(computer: &mut Self, value: bool) {
        computer.extended_mode_flag = value;
    }
}

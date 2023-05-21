use core::fmt;

use crate::{helper::case_insensitive::Str, num3::ThreeDigitNumber};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u16)]
/// The assembly instructions
pub enum Instruction<Data> {
    /// Add the contents of the memory at the specified address / label to the register
    ADD(Data) = 100,
    /// Subtract the contents of the memory at the specified address / label from the register,
    /// setting the negative flag if the result underflows otherwise clearing it
    SUB(Data) = 200,

    /// Store the register in the memory at the specified address / label
    STO(Data) = 300,
    /// Load the memory at the specified address / label into the register
    LDA(Data) = 500,

    /// Go to the specified address / label
    BR(Data) = 600,
    /// If the register is zero, go to the specified address / label
    BRZ(Data) = 700,
    /// If the negative flag is not set, go to the specified address / label
    BRP(Data) = 800,

    /// Take an input and store it in the register
    IN = 901,
    /// Output the register
    OUT = 902,
    #[cfg(feature = "extended")]
    /// Take a char input and store it in the register
    INA = 911,
    #[cfg(feature = "extended")]
    /// Output the register as a char
    OUTA = 912,

    #[default]
    /// Halt the computer
    HLT = 1,

    #[cfg(feature = "extended")]
    /// Enable extended mode
    EXT = 10,

    /// Store the specified data
    DAT(Data) = 0,
}

impl<Data> Instruction<Data> {
    /// Get the op-code of the [Instruction]
    pub const fn op_code(&self) -> ThreeDigitNumber {
        let op_code = unsafe { *(self as *const Self).cast::<u16>() };
        if op_code == 1 {
            ThreeDigitNumber::ZERO
        } else {
            unsafe { ThreeDigitNumber::from_unchecked(op_code) }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct InstructionWithLabel<'a, Data> {
    pub label: Option<&'a str>,
    pub instruction: Instruction<Data>,
}

pub type RawInstruction = Instruction<ThreeDigitNumber>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NumberOrLabel<'a> {
    Number(ThreeDigitNumber),
    Label(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidInstructionError {
    InvalidInstruction,
}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction => write!(f, "Failed to parse an unknown instruction!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidInstructionError {}

impl TryFrom<&str> for Instruction<()> {
    type Error = InvalidInstructionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Str::from(value) {
            i if i == "ADD" => Ok(Self::ADD(())),
            i if i == "SUB" => Ok(Self::SUB(())),

            i if i == "STO" || i == "STA" => Ok(Self::STO(())),
            i if i == "LDA" => Ok(Self::LDA(())),

            i if i == "BR" || i == "BRA" => Ok(Self::BR(())),
            i if i == "BRZ" => Ok(Self::BRZ(())),
            i if i == "BRP" => Ok(Self::BRP(())),

            i if i == "IN" || i == "INP" => Ok(Self::IN),
            i if i == "OUT" => Ok(Self::OUT),
            #[cfg(feature = "extended")]
            i if i == "INA" => Ok(Self::INA),
            #[cfg(feature = "extended")]
            i if i == "OTA" => Ok(Self::OUTA),

            i if i == "HLT" => Ok(Self::HLT),

            #[cfg(feature = "extended")]
            i if i == "EXT" => Ok(Self::EXT),

            i if i == "DAT" => Ok(Self::DAT(())),

            _ => Err(InvalidInstructionError::InvalidInstruction),
        }
    }
}

impl<'a> From<&'a str> for NumberOrLabel<'a> {
    fn from(value: &'a str) -> Self {
        value
            .parse::<u16>()
            .ok()
            .and_then(|number| ThreeDigitNumber::try_from(number).ok())
            .map_or(Self::Label(value), Self::Number)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A data presence error
pub enum Error {
    /// The instruction expected data but did not receive any
    ExpectedData,
    /// The instruction did not expect data but did receive some
    UnexpectedData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedData => write!(f, "Expected label / number!"),
            Self::UnexpectedData => write!(f, "Unexpected label / number!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl Instruction<()> {
    // This cannot be const as the destructor for Data may not be const
    #[allow(clippy::missing_const_for_fn)]
    /// Try to insert data into an instruction
    ///
    /// # Errors
    /// See [Error]
    pub fn try_insert_data<Data>(self, data: Option<Data>) -> Result<Instruction<Data>, Error> {
        use Error::{ExpectedData, UnexpectedData};
        #[cfg(feature = "extended")]
        use Instruction::{ADD, BR, BRP, BRZ, DAT, EXT, HLT, IN, INA, LDA, OUT, OUTA, STO, SUB};
        #[cfg(not(feature = "extended"))]
        use Instruction::{ADD, BR, BRP, BRZ, DAT, HLT, IN, LDA, OUT, STO, SUB};

        match (self, data) {
            (ADD(()), Some(data)) => Ok(ADD(data)),
            (ADD(()), None) => Err(ExpectedData),
            (SUB(()), Some(data)) => Ok(SUB(data)),
            (SUB(()), None) => Err(ExpectedData),

            (STO(()), Some(data)) => Ok(STO(data)),
            (STO(()), None) => Err(ExpectedData),
            (LDA(()), Some(data)) => Ok(LDA(data)),
            (LDA(()), None) => Err(ExpectedData),

            (BR(()), Some(data)) => Ok(BR(data)),
            (BR(()), None) => Err(ExpectedData),
            (BRP(()), Some(data)) => Ok(BRP(data)),
            (BRP(()), None) => Err(ExpectedData),
            (BRZ(()), Some(data)) => Ok(BRZ(data)),
            (BRZ(()), None) => Err(ExpectedData),

            (IN, Some(_)) => Err(UnexpectedData),
            (IN, None) => Ok(IN),
            (OUT, Some(_)) => Err(UnexpectedData),
            (OUT, None) => Ok(OUT),
            #[cfg(feature = "extended")]
            (INA, Some(_)) => Err(UnexpectedData),
            #[cfg(feature = "extended")]
            (INA, None) => Ok(INA),
            #[cfg(feature = "extended")]
            (OUTA, Some(_)) => Err(UnexpectedData),
            #[cfg(feature = "extended")]
            (OUTA, None) => Ok(OUTA),

            (HLT, Some(_)) => Err(UnexpectedData),
            (HLT, None) => Ok(HLT),

            #[cfg(feature = "extended")]
            (EXT, Some(_)) => Err(UnexpectedData),
            #[cfg(feature = "extended")]
            (EXT, None) => Ok(EXT),

            (DAT(()), Some(data)) => Ok(DAT(data)),
            (DAT(()), None) => Err(ExpectedData),
        }
    }
}

impl<Data> Instruction<Data> {
    /// Add a label to an instruction
    pub const fn add_label(self, label: Option<&str>) -> InstructionWithLabel<Data> {
        InstructionWithLabel {
            label,
            instruction: self,
        }
    }
}

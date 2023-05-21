use core::{
    fmt::{self, Binary, Display, LowerHex, Octal, UpperHex},
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
/// A three digit number (0..=999)
pub struct ThreeDigitNumber(u16);

impl ThreeDigitNumber {
    pub const ZERO: Self = Self(0);

    #[must_use]
    /// Checks if the number also a valid two digit number
    pub const fn is_2_digit(self) -> bool {
        self.0 < 100
    }

    #[must_use]
    /// Makes a [`ThreeDigitNumber`] from a [`u16`] without performing any checks
    ///
    /// # Safety
    /// The caller must make sure that `value` is strictly less than 1000 (within `(0..=999)`)
    pub const unsafe fn from_unchecked(value: u16) -> Self {
        Self(value)
    }
}

// Formatting impls

macro_rules! fmt_impl {
    ( $trait:path, $fmt:path ) => {
        impl $trait for ThreeDigitNumber {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                $fmt(&self.0, f)
            }
        }
    };
}

fmt_impl!(Display, Display::fmt);
fmt_impl!(Octal, Octal::fmt);
fmt_impl!(LowerHex, LowerHex::fmt);
fmt_impl!(UpperHex, UpperHex::fmt);
fmt_impl!(Binary, Binary::fmt);

// Operation impls

impl Add for ThreeDigitNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % 1000)
    }
}

impl AddAssign for ThreeDigitNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 %= 1000;
    }
}

impl Sub for ThreeDigitNumber {
    type Output = (Self, bool);

    fn sub(self, rhs: Self) -> Self::Output {
        (Self((self.0 + 1000 - rhs.0) % 1000), self < rhs)
    }
}

// From impls

impl From<u8> for ThreeDigitNumber {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TryFromError {
    TooLarge,
}

impl fmt::Display for TryFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge => write!(f, "Number is too large to be converted to a three digit number (> 999)!"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromError {}

impl TryFrom<u16> for ThreeDigitNumber {
    type Error = TryFromError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1000 {
            Ok(Self(value))
        } else {
            Err(TryFromError::TooLarge)
        }
    }
}

// Into impls

impl From<ThreeDigitNumber> for u16 {
    fn from(value: ThreeDigitNumber) -> Self {
        value.0
    }
}

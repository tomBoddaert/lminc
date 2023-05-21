use core::fmt;

#[doc(hidden)]
#[macro_export]
/// Create a new location type for use with [`ErrorWithLocation`]
macro_rules! create_location_type {
    (
        $( $doc:literal: )? $name:ident $(< $( $lt:lifetime )*, >)? ($( $v:vis $t:ty ),*):
        $s:ident => $($arg:tt)*
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $(#[doc = $doc])?
        pub struct $name $(< $( $lt )*, >)? ($($v $t),*);

        impl $(< $( $lt )*, >)? fmt::Display for $name $(< $( $lt )*, >)? {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let $s = self;
                f.write_fmt(format_args!($($arg)*))
            }
        }
    };
}

create_location_type!(
    "A line number for use with [ErrorWithLocation]":
    LineNumber(pub usize): line => "line {}", line.0
);

create_location_type!(
    "An instruction number for use with [ErrorWithLocation] (usually when [`LineNumber`] cannot be used)":
    InstructionNumber(pub usize): number => "instruction {}", number.0
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// An error with a location in the source
pub struct ErrorWithLocation<Error, Location>(pub Location, pub Error);

impl<Error, Location> fmt::Display for ErrorWithLocation<Error, Location>
where
    Error: fmt::Display,
    Location: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.1, self.0)
    }
}

#[cfg(feature = "std")]
impl<Error, Location> std::error::Error for ErrorWithLocation<Error, Location>
where
    Error: std::error::Error,
    Location: fmt::Debug + fmt::Display,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.1.source()
    }
}

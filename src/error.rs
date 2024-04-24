use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;
use std::string::{FromUtf16Error, FromUtf8Error};
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    IntConversion(TryFromIntError),
    JsonParse(serde_json::Error),
    Utf8Parse(FromUtf8Error),
    Utf16Parse(FromUtf16Error),
    AttributeFormat,
    AttributeParse(u32),
    FactionParse(u32),
    SecurityLevelParse(u32),
    BlueprintIngredientListIndex(usize),
    ShipPositionCode(u16),
    PatchNotAvailableForBinaryVersion,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::IntConversion(error) => write!(f, "{error}"),
            Self::JsonParse(error) => write!(f, "{error}"),
            Self::Utf8Parse(error) => write!(f, "{error}"),
            Self::Utf16Parse(error) => write!(f, "{error}"),
            Self::AttributeFormat => write!(f, "invalid attribute format"),
            Self::AttributeParse(value) => write!(f, "failed to parse attribute value {value}"),
            Self::FactionParse(value) => write!(f, "failed to parse faction value {value}"),
            Self::SecurityLevelParse(value) => {
                write!(f, "failed to parse security level value {value}")
            }
            Self::BlueprintIngredientListIndex(index) => {
                write!(f, "blueprint ingredient list index {index} out of bounds")
            }
            Self::ShipPositionCode(code) => write!(f, "invalid ship position code {code}"),
            Self::PatchNotAvailableForBinaryVersion => {
                write!(f, "patch cannot be applied to this binary version")
            }
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<TryFromIntError> for Error {
    fn from(error: TryFromIntError) -> Self {
        Self::IntConversion(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParse(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::Utf8Parse(error)
    }
}

impl From<FromUtf16Error> for Error {
    fn from(error: FromUtf16Error) -> Self {
        Self::Utf16Parse(error)
    }
}

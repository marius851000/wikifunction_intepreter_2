use std::{
    fmt::Display,
    num::{NonZero, NonZeroU32, ParseIntError, TryFromIntError},
};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub struct Zid(pub NonZero<u32>);

/// A Zid, a.k.a a reference to a persistent object
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ZidParseError {
    #[error("Zid does not start with the letter Z")]
    DoNotStartWithZ,
    #[error("The Zid is an empty string")]
    Empty,
    #[error("Can’t parse the number")]
    CantParse(#[source] ParseIntError),
    #[error("Zid can’t be zero")]
    IsZero(#[source] TryFromIntError),
}

impl Zid {
    pub fn get_z(&self) -> NonZero<u32> {
        self.0
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(source: &str) -> Result<Self, ZidParseError> {
        let mut chars_iter = source.chars();
        if let Some(first_char) = chars_iter.next() {
            if first_char != 'Z' {
                return Err(ZidParseError::DoNotStartWithZ);
            }
        } else {
            return Err(ZidParseError::Empty);
        }

        let number: u32 = chars_iter
            .as_str()
            .parse()
            .map_err(ZidParseError::CantParse)?;

        Self::from_u32(number)
    }

    pub fn from_u32(source: u32) -> Result<Self, ZidParseError> {
        Ok(Self(source.try_into().map_err(ZidParseError::IsZero)?))
    }

    pub const fn from_u32_panic(source: u32) -> Self {
        Self(NonZeroU32::new(source).unwrap())
    }
}

impl Display for Zid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Z{}", self.0)
    }
}

macro_rules! zid {
    ($z:expr) => {{
        const ZID: $crate::Zid = $crate::Zid::from_u32_panic($z);
        ZID
    }};
}

#[cfg(test)]
mod tests {
    use crate::Zid;

    #[test]
    fn test_from_str() {
        assert_eq!(Zid::from_str("Z4").unwrap(), zid!(4));
        assert_eq!(Zid::from_str("Z9324").unwrap(), zid!(9324));
        Zid::from_str("K4").unwrap_err();
        Zid::from_str("").unwrap_err();
        Zid::from_str("K3K").unwrap_err();
    }
}

use std::{
    fmt::{Debug, Display},
    num::{NonZeroU32, ParseIntError, TryFromIntError},
};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum KeyIndexParseError {
    #[error("the input key reference is empty")]
    InputEmpty,
    #[error("the first character should be a Z or a K")]
    FirstNotZOrK,
    #[error("the text contain more text than necessary")]
    TooMuchText,
    #[error("no text before K")]
    NoTextBeforeK,
    #[error("Z and K should not be both undefined")]
    ZAndKUndefined,
    #[error("Can’t parse the Z-part as a number")]
    CantParseZ(#[source] ParseIntError),
    #[error("Can’t parse the K-part as a number")]
    CantParseK(#[source] ParseIntError),
    #[error("Z-part shouldn’t be 0")]
    PartZZero(#[source] TryFromIntError),
    #[error("K-part shouldn’t be 0")]
    PartKZero(#[source] TryFromIntError),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// At least one of the value is Some
pub struct KeyIndex(Option<NonZeroU32>, Option<NonZeroU32>); //TODO: disallow using Z alone?

impl KeyIndex {
    pub fn get_z(&self) -> Option<NonZeroU32> {
        self.0
    }

    pub fn get_k(&self) -> Option<NonZeroU32> {
        self.1
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &str) -> Result<Self, KeyIndexParseError> {
        let mut k_splitted = text.split('K');

        let before_key = k_splitted.next().ok_or(KeyIndexParseError::InputEmpty)?;

        let z = if !before_key.is_empty() {
            let mut char_id_iter = before_key.chars();
            if char_id_iter
                .next()
                .expect("we already checked the this string is not emtpy")
                != 'Z'
            {
                return Err(KeyIndexParseError::FirstNotZOrK);
            }
            Some(
                char_id_iter
                    .as_str()
                    .parse()
                    .map_err(KeyIndexParseError::CantParseZ)?,
            )
        } else {
            None
        };

        let k = if let Some(second_part) = k_splitted.next() {
            Some(
                second_part
                    .parse()
                    .map_err(KeyIndexParseError::CantParseK)?,
            )
        } else {
            None
        };

        if k_splitted.next().is_some() {
            return Err(KeyIndexParseError::FirstNotZOrK);
        }

        KeyIndex::from_u32s(z, k)
    }

    pub fn from_u32s(z: Option<u32>, k: Option<u32>) -> Result<Self, KeyIndexParseError> {
        if z.is_none() && k.is_none() {
            return Err(KeyIndexParseError::ZAndKUndefined);
        }
        Ok(Self(
            if let Some(z) = z {
                Some(NonZeroU32::try_from(z).map_err(KeyIndexParseError::PartZZero)?)
            } else {
                None
            },
            if let Some(k) = k {
                Some(NonZeroU32::try_from(k).map_err(KeyIndexParseError::PartKZero)?)
            } else {
                None
            },
        ))
    }

    pub const fn from_u32s_panic(z: Option<u32>, k: Option<u32>) -> Self {
        if z.is_none() && k.is_none() {
            panic!("z and k should not be both None");
        }
        Self(
            if let Some(z) = z {
                Some(NonZeroU32::new(z).unwrap())
            } else {
                None
            },
            if let Some(k) = k {
                Some(NonZeroU32::new(k).unwrap())
            } else {
                None
            },
        )
    }

    fn to_string_inner(self) -> String {
        if let Some(z) = self.0 {
            if let Some(k) = self.1 {
                format!("Z{}K{}", z, k)
            } else {
                format!("Z{}", z)
            }
        } else if let Some(k) = self.1 {
            format!("K{}", k)
        } else {
            unreachable!("z and k shouldn’t be both null");
        }
    }
}

impl Display for KeyIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_inner())
    }
}

impl Debug for KeyIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // That’s probably good. This reference has quite a specific and recogniseable syntax
        f.write_str(&self.to_string())
    }
}

macro_rules! keyindex {
    ($z:expr) => {{
        const KEYINDEX: crate::KeyIndex = crate::KeyIndex::from_u32s_panic(Some($z), None);
        KEYINDEX
    }};
    ($z:expr, $k:expr) => {{
        const KEYINDEX: crate::KeyIndex = crate::KeyIndex::from_u32s_panic(Some($z), Some($k));
        KEYINDEX
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(KeyIndex::from_str("Z156").unwrap(), keyindex!(156));
        assert_eq!(KeyIndex::from_str("Z30K4").unwrap(), keyindex!(30, 4),);
        assert_eq!(
            KeyIndex::from_str("K1").unwrap(),
            KeyIndex(None, Some(NonZeroU32::new(1)).unwrap())
        );
        assert!(KeyIndex::from_str("T156").is_err());
        assert!(KeyIndex::from_str("Z").is_err());
        assert!(KeyIndex::from_str("Z-9").is_err());
        assert!(KeyIndex::from_str("Z1a").is_err());
        assert!(KeyIndex::from_str("Za1").is_err());
        assert!(KeyIndex::from_str("").is_err());
        assert!(KeyIndex::from_str("Z30K4Z1").is_err());
        assert!(KeyIndex::from_str("Z30K4K1").is_err());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(keyindex!(156).to_string(), "Z156");
        assert_eq!(keyindex!(30, 4).to_string(), "Z30K4");
    }

    #[test]
    fn test_proc_macro() {
        assert_eq!(keyindex!(6), KeyIndex::from_u32s(Some(6), None).unwrap());
        assert_eq!(
            keyindex!(6, 2),
            KeyIndex::from_u32s(Some(6), Some(2)).unwrap()
        )
    }
}

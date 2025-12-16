use std::{
    fmt::{Debug, Display},
    num::NonZeroU32,
};

use anyhow::{Context, anyhow};
use serde::{Deserialize, de::Visitor};

use crate::EvalErrorKind;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// At least one of the value is Some
pub struct Zid(Option<NonZeroU32>, Option<NonZeroU32>);

impl Zid {
    pub fn get_z(&self) -> Option<NonZeroU32> {
        self.0
    }

    pub fn get_k(&self) -> Option<NonZeroU32> {
        self.1
    }

    pub fn from_zid(text: &str) -> Result<Self, EvalErrorKind> {
        let mut k_splitted = text.split('K');

        let before_key = k_splitted
            .next()
            .context("input text should not be empty")
            .map_err(|e| EvalErrorKind::ParseZid(e))?;

        let z = if !before_key.is_empty() {
            let mut char_id_iter = before_key.chars();
            if char_id_iter
                .next()
                .context("text before K/end of string should not be empty")
                .map_err(|e| EvalErrorKind::ParseZid(e))?
                != 'Z'
            {
                return Err(EvalErrorKind::ParseZid(anyhow!(
                    "First character should be Z"
                )));
            }
            Some(
                u32::from_str_radix(char_id_iter.as_str(), 10)
                    .context("Can’t convert the first number part of the ZID to a u32 number")
                    .map_err(|e| EvalErrorKind::ParseZid(e))?,
            )
        } else {
            None
        };

        let k = if let Some(second_part) = k_splitted.next() {
            Some(
                u32::from_str_radix(second_part, 10)
                    .context("Could not parse post-key text as u32")
                    .map_err(|e| EvalErrorKind::ParseZid(e))?,
            )
        } else {
            None
        };

        if k_splitted.next().is_some() {
            return Err(EvalErrorKind::ParseZid(anyhow!(
                "Text contain extra characters"
            )));
        }

        Ok(Zid::from_u32s(z, k)?)
    }

    pub fn from_u32s(z: Option<u32>, k: Option<u32>) -> Result<Self, EvalErrorKind> {
        if z.is_none() && k.is_none() {
            return Err(EvalErrorKind::ParseZid(anyhow!(
                "z and k should not be both None"
            )));
        }
        Ok(Self(
            if let Some(z) = z {
                Some(
                    NonZeroU32::try_from(z)
                        .context("z should be non-zero")
                        .map_err(|e| EvalErrorKind::ParseZid(e))?,
                )
            } else {
                None
            },
            if let Some(k) = k {
                Some(
                    NonZeroU32::try_from(k)
                        .context("k should be non-zero")
                        .map_err(|e| EvalErrorKind::ParseZid(e))?,
                )
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

    pub fn to_zid(&self) -> String {
        if let Some(z) = self.0 {
            if let Some(k) = self.1 {
                format!("Z{}K{}", z, k)
            } else {
                format!("Z{}", z)
            }
        } else {
            if let Some(k) = self.1 {
                format!("K{}", k)
            } else {
                unreachable!("z and k should be both null");
            }
        }
    }
}

impl Display for Zid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_zid())
    }
}

impl Debug for Zid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // That’s probably good. This reference has quite a specific and recogniseable syntax
        f.write_str(&self.to_zid())
    }
}

#[derive(Default)]
pub(crate) struct ReferenceVisitor {}

impl<'de> Visitor<'de> for ReferenceVisitor {
    type Value = Zid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a ZID")
    }

    fn visit_borrowed_str<E>(self, t: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Zid::from_zid(t) {
            Ok(v) => Ok(v),

            Err(err) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(t),
                &err.to_string().as_str(),
            )),
        }
    }
}

impl<'de> Deserialize<'de> for Zid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ReferenceVisitor::default())
    }
}

macro_rules! zid {
    ($z:expr) => {{
        const ZID: crate::Zid = crate::Zid::from_u32s_panic(Some($z), None);
        ZID
    }};
    ($z:expr, $k:expr) => {{
        const ZID: crate::Zid = crate::Zid::from_u32s_panic(Some($z), Some($k));
        ZID
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_zid() {
        assert_eq!(Zid::from_zid("Z156").unwrap(), zid!(156));
        assert_eq!(Zid::from_zid("Z30K4").unwrap(), zid!(30, 4),);
        assert_eq!(
            Zid::from_zid("K1").unwrap(),
            Zid(None, Some(NonZeroU32::new(1)).unwrap())
        );
        assert!(Zid::from_zid("T156").is_err());
        assert!(Zid::from_zid("Z").is_err());
        assert!(Zid::from_zid("Z-9").is_err());
        assert!(Zid::from_zid("Z1a").is_err());
        assert!(Zid::from_zid("Za1").is_err());
        assert!(Zid::from_zid("").is_err());
        assert!(Zid::from_zid("Z30K4Z1").is_err());
        assert!(Zid::from_zid("Z30K4K1").is_err());
    }

    #[test]
    fn test_to_zid() {
        assert_eq!(zid!(156).to_zid(), "Z156");
        assert_eq!(zid!(30, 4).to_zid(), "Z30K4");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(serde_json::from_str::<Zid>("\"Z654\"").unwrap(), zid!(654),);
        assert_eq!(
            serde_json::from_str::<Zid>("\"Z30K5\"").unwrap(),
            zid!(30, 5),
        );
        assert!(serde_json::from_str::<Zid>("654").is_err());
        assert!(serde_json::from_str::<Zid>("Z1a").is_err());
    }

    #[test]
    fn test_proc_macro() {
        assert_eq!(zid!(6), Zid::from_u32s(Some(6), None).unwrap());
        assert_eq!(zid!(6, 2), Zid::from_u32s(Some(6), Some(2)).unwrap())
    }
}

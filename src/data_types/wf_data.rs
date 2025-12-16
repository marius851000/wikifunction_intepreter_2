use std::{collections::BTreeMap, num::NonZeroU32};

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{
        WfBoolean, WfDataType, WfReference, WfString, WfUntyped, types_def::WfTypeGeneric,
    },
};

/// A type that reference one of the data type in a memory efficient way. And also it isn’t dyn-compatible?
#[derive(Debug, Clone, PartialEq)]
pub enum WfData {
    WfBoolean(WfBoolean),
    WfReference(WfReference),
    WfString(WfString),
    WfUntyped(WfUntyped),
    WfType(WfTypeGeneric),
}

impl WfData {
    pub fn new_reference(zid: Zid) -> Self {
        Self::WfReference(WfReference::new(zid))
    }

    pub fn from_map(map: BTreeMap<Zid, WfData>) -> WfData {
        WfData::WfUntyped(WfUntyped::new(map))
    }

    pub fn into_map(
        self,
        context: &ExecutionContext,
    ) -> Result<BTreeMap<Zid, WfData>, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.into_map(context),
            Self::WfReference(d) => d.into_map(context),
            Self::WfString(d) => d.into_map(context),
            Self::WfUntyped(d) => d.into_map(context),
            Self::WfType(d) => d.into_map(context),
        }
    }

    pub fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        match self {
            Self::WfBoolean(d) => d.into_map_no_follow(),
            Self::WfReference(d) => d.into_map_no_follow(),
            Self::WfString(d) => d.into_map_no_follow(),
            Self::WfUntyped(d) => d.into_map_no_follow(),
            Self::WfType(d) => d.into_map_no_follow(),
        }
    }

    pub fn get_reference(self, context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.get_reference(context),
            Self::WfReference(d) => d.get_reference(context),
            Self::WfString(d) => d.get_reference(context),
            Self::WfUntyped(d) => d.get_reference(context),
            Self::WfType(d) => d.get_reference(context),
        }
    }

    pub fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.evaluate(context),
            Self::WfReference(d) => d.evaluate(context),
            Self::WfString(d) => d.evaluate(context),
            Self::WfUntyped(d) => d.evaluate(context),
            Self::WfType(_d) => todo!("type evaluate"),
        }
    }

    pub fn parse_boolean(self, context: &ExecutionContext) -> Result<WfBoolean, (EvalError, Self)> {
        match self {
            Self::WfBoolean(ready) => Ok(ready),
            other => WfBoolean::parse(other.into_map(context)?, context),
        }
    }

    pub fn parse_type(
        self,
        context: &ExecutionContext,
    ) -> Result<WfTypeGeneric, (EvalError, Self)> {
        match self {
            Self::WfType(ready) => Ok(ready),
            other => WfTypeGeneric::parse(other, context),
        }
    }

    pub fn is_fully_realised(&self) -> bool {
        match self {
            Self::WfBoolean(v) => v.is_fully_realised(),
            Self::WfReference(v) => v.is_fully_realised(),
            Self::WfString(v) => v.is_fully_realised(),
            Self::WfType(v) => v.is_fully_realised(),
            Self::WfUntyped(v) => v.is_fully_realised(),
        }
    }

    /// Error return: first is self, second is other. The last bool is true if the error originate from self, false if it originate from other.
    pub fn equality(
        self,
        other: WfData,
        context: &ExecutionContext,
    ) -> Result<(bool, WfData, WfData), (EvalError, WfData, WfData, bool)> {
        // fast path
        if self == other {
            return Ok((true, self, other));
        } else if self.is_fully_realised() && other.is_fully_realised() {
            return Ok((false, self, other));
        }

        // slow path, compare key-by-key
        let mut entries_first = match self.into_map(context) {
            Ok(v) => v,
            Err((e, self_s)) => return Err((e, self_s, other, true)),
        };
        let mut entries_second = match other.into_map(context) {
            Ok(v) => v,
            Err((e, other)) => return Err((e, WfData::from_map(entries_first), other, false)),
        };

        if !entries_first.keys().eq(entries_second.keys()) {
            return Ok((
                false,
                WfData::from_map(entries_first),
                WfData::from_map(entries_second),
            ));
        }

        let keys: Vec<Zid> = entries_first.keys().cloned().collect();

        for key in keys {
            let value_first = entries_first.remove(&key).expect("key missing?");
            let value_second = entries_second.remove(&key).expect("key missing?");

            let (this_equality, value_first, value_second) =
                match value_first.equality(value_second, context) {
                    Ok(v) => v,
                    Err((e, value_first, value_second, error_source_which)) => {
                        entries_first.insert(key, value_first);
                        entries_second.insert(key, value_second);
                        return Err((
                            e.inside(key.clone()),
                            WfData::from_map(entries_first),
                            WfData::from_map(entries_second),
                            error_source_which,
                        ));
                    }
                };

            entries_first.insert(key, value_first);
            entries_second.insert(key, value_second);

            if this_equality == false {
                return Ok((
                    false,
                    WfData::from_map(entries_first),
                    WfData::from_map(entries_second),
                ));
            }
        }

        Ok((
            true,
            WfData::from_map(entries_first),
            WfData::from_map(entries_second),
        ))
    }

    pub fn check_type_by_zid(
        self,
        expected_zid: Zid,
        context: &ExecutionContext,
    ) -> Result<WfData, (EvalError, WfData)> {
        let (got_zid, value) = self.get_type_zid(context)?;
        if expected_zid != got_zid {
            Err((
                EvalError::from_kind(EvalErrorKind::WrongType(got_zid, expected_zid)),
                value,
            ))
        } else {
            Ok(value)
        }
    }

    pub fn get_type_zid(
        self,
        context: &ExecutionContext,
    ) -> Result<(Zid, WfData), (EvalError, WfData)> {
        match &self {
            Self::WfReference(reference) => {
                if let Some(z) = reference.to.get_z() {
                    if z < NonZeroU32::new(100).unwrap() {
                        return Ok((reference.to, self));
                    }
                }
            }
            _ => (),
        }
        // slow path
        let r#type = self.parse_type(context)?;
        match r#type.get_type_zid() {
            Ok(zid) => Ok((zid, WfData::WfType(r#type))),
            Err(e) => Err((e, WfData::WfType(r#type))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext,
        data_types::{WfBoolean, WfData, WfDataType},
    };

    #[test]
    fn test_equality() {
        let global_context = Arc::new(GlobalContext::default_for_test());
        let context = ExecutionContext::default_for_global(global_context);

        // test fast equality. Those object are invalid, and will fail if evaluated, but it won’t need to.
        let test1_data = WfData::from_map(btree_map! {});
        let test1_data_clone = test1_data.clone();
        assert!(test1_data.equality(test1_data_clone, &context).unwrap().0);

        // test can spot different boolean as well as reconstruction
        let test2_first_true = WfBoolean::new(true).into_wf_data();
        let test2_second_false = WfBoolean::new(false).into_wf_data();
        let (test2_eq, test2_first_true, _test2_second_false) = test2_first_true
            .equality(test2_second_false, &context)
            .unwrap();
        assert!(!test2_eq);
        assert!(
            test2_first_true
                .equality(WfBoolean::new(true).into_wf_data(), &context)
                .unwrap()
                .0
        );
    }
}

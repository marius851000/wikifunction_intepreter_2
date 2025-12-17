use std::{collections::BTreeMap, num::NonZeroU32};

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{
        WfBoolean, WfDataType, WfInvalid, WfReference, WfString, WfUntyped,
        types_def::WfTypeGeneric,
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
    WfInvalid(WfInvalid),
}

impl_wf_data_type!(
    WfData,
    |this| this,
    WfBoolean(d),
    WfReference(d),
    WfString(d),
    WfUntyped(d),
    WfType(d),
    WfInvalid(d)
);

impl WfData {
    pub fn new_reference(zid: Zid) -> Self {
        Self::WfReference(WfReference::new(zid))
    }

    pub fn from_map(map: BTreeMap<Zid, WfData>) -> WfData {
        WfData::WfUntyped(WfUntyped::new(map))
    }

    pub fn parse_type(
        self,
        context: &ExecutionContext,
    ) -> Result<WfTypeGeneric, (EvalError, WfData)> {
        match self {
            Self::WfType(ready) => Ok(ready),
            other => WfTypeGeneric::parse(other, context),
        }
    }

    /// Error return: The last bool is true if the error originate from self, false if it originate from other.
    /// TODO: make use of the identity reference
    pub fn equality(
        self,
        other: WfData,
        context: &ExecutionContext,
    ) -> Result<bool, (EvalError, bool)> {
        // fast path
        if self == other {
            return Ok(true);
        } else if self.is_fully_realised() && other.is_fully_realised() {
            return Ok(false);
        }

        // slow path, compare key-by-key
        let first = self.evaluate(context).unwrap(); //TODO: error handling
        let other = other.evaluate(context).unwrap(); //TODO: error handling
        let keys_first = first.list_keys();
        let keys_second = other.list_keys();

        if !keys_first.iter().eq(keys_second.iter()) {
            return Ok(false);
        }

        for key in keys_first {
            let value_first = first
                .get_key(key)
                .expect("key should be guaranteed to exist");
            let value_second = other
                .get_key(key)
                .expect("key should be guaranteed to exist");

            let this_equality = match value_first.equality(value_second, context) {
                Ok(v) => v,
                Err((e, error_source_which)) => {
                    return Err((e.inside(key), error_source_which));
                }
            };

            if !this_equality {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // TODO: move to a check_identity?
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
        if let Self::WfReference(reference) = &self
            && let Some(z) = reference.to.get_z()
            && z < NonZeroU32::new(100).unwrap()
        {
            return Ok((reference.to, self));
        }
        // slow path
        let r#type = self.parse_type(context)?;
        match r#type.get_type_zid() {
            Ok(zid) => Ok((zid, WfData::WfType(r#type))),
            Err(e) => Err((e, WfData::WfType(r#type))),
        }
    }

    #[cfg(test)]
    pub fn unvalid(reason: EvalErrorKind) -> Self {
        Self::WfInvalid(WfInvalid::new(reason))
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
        // TODO: is that intended behavior
        let test1_data = WfData::from_map(btree_map! {});
        let test1_data_clone = test1_data.clone();
        assert!(test1_data.equality(test1_data_clone, &context).unwrap());

        // test can spot different boolean
        let test2_first_true = WfBoolean::new(true).into_wf_data();
        let test2_second_false = WfBoolean::new(false).into_wf_data();
        assert!(
            !test2_first_true
                .equality(test2_second_false, &context)
                .unwrap()
        );
    }
}

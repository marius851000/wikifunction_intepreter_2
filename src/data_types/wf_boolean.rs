use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WfBoolean {
    value: bool,
}

impl WfBoolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    /// assume the data is dereferenced (but may be untyped)
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfBoolean(v) = data {
            return Ok(v);
        };
        match data.get_key(keyindex!(1, 1)) {
            None => return Err((EvalError::missing_key(keyindex!(1, 1)), data)),
            Some(r#type) => {
                if let Err((e, _content)) = r#type.check_type_by_zid(zid!(40), context) {
                    return Err((e.inside(keyindex!(1, 1)), data));
                }
            }
        };

        let identity = match data.get_key(keyindex!(40, 1)) {
            Some(i) => i,
            None => return Err((EvalError::missing_key(keyindex!(40, 1)), data)),
        };

        match identity.get_reference(context) {
            Ok(reference) => {
                if reference == zid!(41) {
                    Ok(Self { value: false })
                } else if reference == zid!(42) {
                    Ok(Self { value: true })
                } else {
                    Err((
                        EvalError::from_kind(EvalErrorKind::IncorrectIdentityForBoolean(reference)),
                        data,
                    ))
                }
            }
            Err((_err, identity)) => match Self::parse(identity, context) {
                Ok(v) => Ok(v),
                Err((e, _)) => Err((e.inside(keyindex!(40, 1)), data)),
            },
        }
    }
}

impl WfDataType for WfBoolean {
    fn get_identity_key(&self) -> Option<KeyIndex> {
        Some(keyindex!(40, 1))
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(40)))
        } else if key == keyindex!(40, 1) {
            match self.value {
                true => Some(WfData::new_reference(zid!(41))),
                false => Some(WfData::new_reference(zid!(42))),
            }
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![keyindex!(1, 1), keyindex!(40, 1)]
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfBoolean(self)
    }
}

#[cfg(test)]
mod tests {
    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext, RcI,
        data_types::{WfBoolean, WfData, WfDataType},
    };

    #[test]
    fn test_parse() {
        let global_context = RcI::new(GlobalContext::default_for_test());
        let context = ExecutionContext::default_for_global(global_context);

        // simple case, direct reference
        let boolean_construct = WfData::from_map(btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(40)),
            keyindex!(40, 1) => WfData::new_reference(zid!(42))
        });
        assert_eq!(
            WfBoolean::parse(boolean_construct, &context).unwrap(),
            WfBoolean::new(true)
        );

        // simple case, reference
        assert_eq!(
            WfBoolean::parse(
                WfData::new_reference(zid!(41)).evaluate(&context).unwrap(),
                &context
            )
            .unwrap(),
            WfBoolean::new(true)
        );

        // test simple failure
        let incorrect_boolean = WfData::from_map(btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(40)),
            keyindex!(40, 1) => WfData::new_reference(zid!(39))
        });
        WfBoolean::parse(incorrect_boolean, &context).unwrap_err();
    }
}

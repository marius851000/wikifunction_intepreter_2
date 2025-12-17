use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
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
        match data {
            WfData::WfBoolean(v) => return Ok(v),
            _ => (),
        };
        match data.get_key(zid!(1, 1)) {
            None => return Err((EvalError::missing_key(zid!(1, 1)), data)),
            Some(r#type) => match r#type.check_type_by_zid(zid!(40), context) {
                Err((e, content)) => {
                    return Err((e.inside(zid!(1, 1)), data));
                }
                Ok(_) => (),
            },
        };

        let identity = match data.get_key(zid!(40, 1)) {
            Some(i) => i,
            None => return Err((EvalError::missing_key(zid!(40, 1)), data)),
        };

        match identity.get_reference(context) {
            Ok(reference) => {
                if reference == zid!(41) {
                    Ok(Self { value: true })
                } else if reference == zid!(42) {
                    Ok(Self { value: false })
                } else {
                    return Err((
                        EvalError::from_kind(EvalErrorKind::IncorrectIdentityForBoolean(reference)),
                        data,
                    ));
                }
            }
            Err((_err, identity)) => match Self::parse(identity, context) {
                Ok(v) => Ok(v),
                Err((e, _)) => {
                    return Err((e.inside(zid!(40, 1)), data));
                }
            },
        }
    }
}

impl WfDataType for WfBoolean {
    fn get_identity_key(&self) -> Option<Zid> {
        Some(zid!(40, 1))
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        if key == zid!(1, 1) {
            Some(WfData::new_reference(zid!(40)))
        } else if key == zid!(40, 1) {
            match self.value {
                false => Some(WfData::new_reference(zid!(41))),
                true => Some(WfData::new_reference(zid!(42))),
            }
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<Zid> {
        vec![zid!(1, 1), zid!(40, 1)]
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
    use std::sync::Arc;

    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext,
        data_types::{WfBoolean, WfData, WfDataType},
    };

    #[test]
    fn test_parse() {
        let global_context = Arc::new(GlobalContext::default_for_test());
        let context = ExecutionContext::default_for_global(global_context);

        // simple case, direct reference
        let boolean_construct = WfData::from_map(btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(40)),
            zid!(40, 1) => WfData::new_reference(zid!(42))
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
            zid!(1, 1) => WfData::new_reference(zid!(40)),
            zid!(40, 1) => WfData::new_reference(zid!(39))
        });
        WfBoolean::parse(incorrect_boolean, &context).unwrap_err();
    }
}

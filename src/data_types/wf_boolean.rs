use std::collections::BTreeMap;

use map_macro::btree_map;

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WfBoolean {
    value: bool,
}

impl WfBoolean {
    pub fn parse(
        mut data: BTreeMap<Zid, WfData>,
        context: &ExecutionContext,
    ) -> Result<Self, (EvalError, WfData)> {
        match data.remove(&zid!(1, 1)) {
            None => return Err((EvalError::missing_key(zid!(1, 1)), WfData::from_map(data))),
            Some(r#type) => match r#type.check_type_by_zid(zid!(40), context) {
                Err((e, content)) => {
                    return Err((e.inside(zid!(1, 1)), {
                        data.insert(zid!(1, 1), content);
                        WfData::from_map(data)
                    }));
                }
                Ok(_) => (),
            },
        };

        let identity = match data.remove(&zid!(40, 1)) {
            Some(i) => i,
            None => return Err((EvalError::missing_key(zid!(40, 1)), WfData::from_map(data))),
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
                        {
                            data.insert(zid!(40, 1), WfData::new_reference(reference));
                            data.insert(zid!(1, 1), WfData::new_reference(zid!(40)));
                            WfData::from_map(data)
                        },
                    ));
                }
            }
            Err((_err, identity)) => match identity.parse_boolean(context) {
                Ok(v) => Ok(v),
                Err((e, previous)) => {
                    return Err((e.inside(zid!(40, 1)), {
                        data.insert(zid!(40, 1), previous);
                        data.insert(zid!(1, 1), WfData::new_reference(zid!(40)));
                        WfData::from_map(data)
                    }));
                }
            },
        }
    }
}

impl WfDataType for WfBoolean {
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(40)),
            zid!(40, 1) => match self.value {
                false => WfData::new_reference(zid!(41)),
                true => WfData::new_reference(zid!(42))
            }
        }
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
        data_types::{WfBoolean, WfData},
    };

    #[test]
    fn test_parse() {
        let global_context = Arc::new(GlobalContext::default());

        // simple case, direct reference
        let boolean_construct = WfData::from_map(btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(40)),
            zid!(40, 1) => WfData::new_reference(zid!(42))
        });
        assert_eq!(
            boolean_construct
                .parse_boolean(&ExecutionContext::default_for_global(global_context))
                .unwrap(),
            WfBoolean { value: true }
        );
    }
}

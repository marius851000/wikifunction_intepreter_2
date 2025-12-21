use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfTypedListType {
    r#type: RcI<WfTypeGeneric>,
}

impl WfTypedListType {
    pub fn new(inner_type: WfTypeGeneric) -> Self {
        Self {
            r#type: RcI::new(inner_type),
        }
    }

    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfType(WfTypeGeneric::WfTypedListType(v)) = data {
            return Ok(v);
        }
        data.assert_evaluated();
        // check type of this
        match data.get_key_err(keyindex!(1, 1)) {
            Ok(this_type) => {
                if let Err((e, _)) = this_type.check_identity_zid(context, zid!(7)) {
                    return Err((e.inside(keyindex!(1, 1)), data));
                }
            }
            Err(e) => return Err((e, data)),
        };

        // check function to be called
        match data.get_key_err(keyindex!(7, 1)) {
            Ok(this_function) => {
                if let Err((e, _)) = this_function.check_identity_zid(context, zid!(881)) {
                    return Err((e.inside(keyindex!(7, 1)), data));
                }
            }
            Err(e) => return Err((e, data)),
        };

        // obtain type
        let r#type = match data.get_key_err(keyindex!(881, 1)) {
            Err(e) => return Err((e, data)),
            Ok(unparsed_type) => match unparsed_type.evaluate(context) {
                Err((e, _)) => return Err((e.inside(keyindex!(881, 1)), data)),
                Ok(unparsed_type) => match WfTypeGeneric::parse(unparsed_type, context) {
                    Err((e, _)) => return Err((e.inside(keyindex!(881, 1)), data)),
                    Ok(v) => v,
                },
            },
        };

        Ok(Self {
            r#type: RcI::new(r#type),
        })
    }
}

impl WfDataType for WfTypedListType {
    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfTypedListType(self))
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn get_identity_zid_key(&self) -> Option<crate::KeyIndex> {
        None
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        // map that as a function call
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(7)))
        } else if key == keyindex!(7, 1) {
            Some(WfData::new_reference(zid!(881)))
        } else if key == keyindex!(881, 1) {
            Some((*self.r#type).clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![keyindex!(1, 1), keyindex!(7, 1), keyindex!(881, 1)]
    }

    fn substitute_function_arguments<I: crate::data_types::util::SubstitutionInfo>(
        self,
        info: &I,
        context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        Ok(Self::new(
            match WfTypeGeneric::parse(
                (&*self.r#type)
                    .clone()
                    .substitute_function_arguments(info, context)
                    .map_err(|e| e.inside(keyindex!(881, 1)))?,
                context,
            ) {
                Ok(v) => v,
                Err((e, _)) => return Err(e.inside(keyindex!(881, 1))),
            },
        )
        .into_wf_data())
    }
}

#[cfg(test)]
mod tests {
    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext, RcI,
        data_types::{
            WfData, WfDataType,
            types_def::{WfTypeGeneric, WfTypedListType},
        },
    };

    #[test]
    fn test_get_list_key() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));
        let boolean_type = WfData::new_reference(zid!(40)).evaluate(&context).unwrap();
        let boolean_type_clone = boolean_type.clone();
        let test_typed_list_typed = WfTypedListType {
            r#type: RcI::new(WfTypeGeneric::parse(boolean_type, &context).unwrap()),
        };
        assert!(test_typed_list_typed.list_keys().contains(&keyindex!(7, 1)));
        assert!(
            test_typed_list_typed
                .list_keys()
                .contains(&keyindex!(881, 1))
        );
        assert!(
            test_typed_list_typed
                .get_key(keyindex!(881, 1))
                .unwrap()
                .into_wf_data()
                .equality(boolean_type_clone, &context)
                .unwrap()
        );
        assert!(
            test_typed_list_typed
                .get_key(keyindex!(1, 1))
                .unwrap()
                .into_wf_data()
                .equality(WfData::new_reference(zid!(7)), &context)
                .unwrap()
        );
    }

    #[test]
    fn test_parse() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));
        let type_def = WfData::from_map(btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(7)),
            keyindex!(7, 1) => WfData::new_reference(zid!(881)),
            keyindex!(881, 1) => WfData::new_reference(zid!(40))
        });

        assert_eq!(
            WfTypedListType::parse(type_def.clone(), &context)
                .unwrap()
                .r#type
                .get_identity_zid(&context, keyindex!(4, 1))
                .unwrap(),
            zid!(40)
        );

        let evaluated = type_def.clone().evaluate(&context).unwrap();

        assert!(evaluated.equality(type_def, &context).unwrap())
    }
}

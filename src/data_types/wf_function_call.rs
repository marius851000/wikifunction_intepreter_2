use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{
        WfData, WfDataType, WfFunction,
        types_def::{WfTypeGeneric, WfTypedListType},
    },
};

#[derive(Debug)]
pub enum FunctionCallOrType {
    FunctionCall(WfFunctionCall),
    Type(WfTypeGeneric),
}

#[derive(Debug, PartialEq, Clone)]
pub struct WfFunctionCallInner {
    function: WfFunction,
    args: Vec<WfData>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WfFunctionCall(RcI<WfFunctionCallInner>);

#[allow(unused_assignments)] //TODO: remove once parse is fully implemented
impl WfFunctionCall {
    /// May return a type as a shortcut for typed function function call
    pub fn parse(
        mut data: WfData,
        context: &ExecutionContext,
    ) -> Result<FunctionCallOrType, (EvalError, WfData)> {
        if let WfData::WfFunctionCall(v) = data {
            return Ok(FunctionCallOrType::FunctionCall(v));
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

        // get function reference (just as shortcut for checked typed)
        let function_unevaluated = match data.get_key_err(keyindex!(7, 1)) {
            Err(e) => return Err((e, data)),
            Ok(function_uneval) => function_uneval,
        };

        let (function_identity, function_unevaluated) =
            match function_unevaluated.get_reference(context) {
                Err((_, d)) => (None, d),
                Ok((reference, f)) => (Some(reference), f),
            };

        if let Some(function_reference) = function_identity {
            if function_reference == zid!(881) {
                data = match WfTypedListType::parse(data, context) {
                    Ok(typed_list_type) => {
                        return Ok(FunctionCallOrType::Type(WfTypeGeneric::WfTypedListType(
                            typed_list_type,
                        )));
                    }
                    Err((_, d)) => d,
                };
            }
        }

        let function_evaluated = match function_unevaluated.evaluate(context) {
            Ok(v) => v,
            Err((e, _)) => return Err((e.inside(keyindex!(7, 1)), data)),
        };

        let function = match WfFunction::parse(function_evaluated, context) {
            Ok(v) => v,
            Err((e, _)) => return Err((e.inside(keyindex!(7, 1)), data)),
        };

        let function_identity = function.0.identity;

        let num_argument = function.0.arguments.entries.len();

        let num_argument_plus_one_as_u32: u32 = match num_argument.saturating_add(1).try_into() {
            Ok(v) => v,
            _ => {
                return Err((
                    EvalError::from_kind(EvalErrorKind::TooManyArgsInFunction),
                    data,
                ));
            }
        };

        let mut args = Vec::new();
        for i in 1..num_argument_plus_one_as_u32 {
            let key = KeyIndex::from_u32s_panic(Some(function_identity.0.into()), Some(i)); // should not panic, z is already a valid Zid and i is always strictly greater than 0
            let parameter = match data.get_key_err(key) {
                Ok(k) => k,
                Err(e) => return Err((e, data)),
            };
            args.push(parameter)
        }

        Ok(FunctionCallOrType::FunctionCall(Self(RcI::new(
            WfFunctionCallInner { function, args },
        ))))
    }
}

impl WfDataType for WfFunctionCall {
    fn list_keys(&self) -> Vec<KeyIndex> {
        todo!();
    }

    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        todo!();
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfFunctionCall(self)
    }

    fn is_fully_realised(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext, RcI,
        data_types::{
            WfBoolean, WfData, WfDataType, WfFunctionCall, wf_function_call::FunctionCallOrType,
        },
    };

    #[test]
    fn test_parse_function_call() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let unparsed_tree = btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(7)),
            keyindex!(7, 1) => WfData::new_reference(zid!(844)),
            // boolean equality function
            keyindex!(844, 1) => WfBoolean::new(false).into_wf_data(),
            keyindex!(844, 2) => WfBoolean::new(true).into_wf_data(),
        };

        let unparsed = WfData::from_map(unparsed_tree.clone());

        let parsed = WfFunctionCall::parse(unparsed.clone(), &context).unwrap();
        let parsed = if let FunctionCallOrType::FunctionCall(fc) = parsed {
            fc
        } else {
            panic!();
        };

        assert_eq!(parsed.0.args.len(), 2);
        assert_eq!(
            parsed.0.args,
            vec![
                WfBoolean::new(false).into_wf_data(),
                WfBoolean::new(true).into_wf_data()
            ]
        );
        assert!(
            parsed
                .0
                .function
                .clone()
                .into_wf_data()
                .equality(WfData::new_reference(zid!(844)), &context)
                .unwrap()
        );
        //TODO: this actually run the function, which wouldn’t be a bad idea if it were implemented
        //assert!(parsed.into_wf_data().equality(unparsed, &context).unwrap());

        let mut invalid_unparsed = unparsed_tree;
        invalid_unparsed.remove(&keyindex!(844, 1));
        WfData::from_map(invalid_unparsed)
            .evaluate(&context)
            .unwrap_err();
    }
}

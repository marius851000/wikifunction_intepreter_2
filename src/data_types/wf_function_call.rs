use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{
        WfData, WfDataType,
        types_def::{WfTypeGeneric, WfTypedListType},
    },
};

pub enum FunctionCallOrType {
    FunctionCall(WfFunctionCall),
    Type(WfTypeGeneric),
}
#[derive(Debug, PartialEq, Clone)]
pub struct WfFunctionCallInner {
    function: WfData, // unevaluated (TODO: for now? Store as a zid?)
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

        let (function_reference, _function_unevaluated) =
            match function_unevaluated.get_reference(context) {
                Err((_, d)) => (None, d),
                Ok((reference, f)) => (Some(reference), f),
            };

        if let Some(function_reference) = function_reference {
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

        todo!();
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

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{
        WfData, WfDataType, WfFunctionCall,
        types_def::{WfStandardType, WfTypedListType},
        wf_function_call::FunctionCallOrType,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum WfTypeGeneric {
    WfStandardType(WfStandardType),
    WfTypedListType(WfTypedListType), // TODO: typed pair
}

impl WfTypeGeneric {
    /// assume the data is dereferenced (but may be untyped)
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        // we know this isn’t a reference, not it is a function. The type-function (linked list, etc) should already be dereferenced.
        match data {
            WfData::WfType(r#type) => return Ok(r#type),
            _ => (),
        };

        // slow path
        let type_zid = match data.get_key_err(keyindex!(1, 1)) {
            Err(e) => return Err((e, data)),
            Ok(k) => match k.get_type_zid(context) {
                Err((e, _)) => return Err((e.inside(keyindex!(1, 1)), data)),
                Ok((z, _)) => z,
            },
        };

        if type_zid == zid!(7) {
            match WfFunctionCall::parse(data, context) {
                Ok(FunctionCallOrType::Type(t)) => Ok(t),
                Ok(FunctionCallOrType::FunctionCall(f)) => Err((
                    EvalError::from_kind(EvalErrorKind::ExpectedTypeGotFunction),
                    f.into_wf_data(),
                )),
                Err((e, data)) => Err((e, data)),
            }
        } else if type_zid == zid!(4) {
            todo!();
        } else {
            Err((
                EvalError::from_kind(EvalErrorKind::WrongTypeZidForType).inside(keyindex!(1, 1)),
                data,
            ))
        }
    }

    pub fn get_type_zid(&self) -> Result<Zid, EvalError> {
        match self {
            Self::WfStandardType(standard) => Ok(standard.inner.identity_ref),
            _ => Err(EvalError::from_kind(EvalErrorKind::NotStandardType)),
        }
    }
}

impl_wf_data_type!(
    WfTypeGeneric,
    WfData::WfType,
    WfStandardType(d),
    WfTypedListType(d)
);

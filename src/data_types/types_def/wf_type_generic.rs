use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex,
    data_types::{
        WfData,
        types_def::{WfStandardType, WfTypedListType},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum WfTypeGeneric {
    WfStandardType(WfStandardType),
    WfTypedListType(WfTypedListType), // TODO: typed pair
}

impl WfTypeGeneric {
    /// assume the data is dereferenced (but may be untyped)
    pub fn parse(data: WfData, _context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        // we know this isn’t a reference, not it is a function. The type-function (linked list, etc) should already be dereferenced.
        match data {
            WfData::WfType(r#type) => Ok(r#type),
            _other => todo!(),
        }
    }

    pub fn get_type_zid(&self) -> Result<KeyIndex, EvalError> {
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

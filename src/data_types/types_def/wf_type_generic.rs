use crate::{
    EvalError, ExecutionContext, Zid,
    data_types::{WfData, types_def::WfStandardType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum WfTypeGeneric {
    WfStandardType(WfStandardType),
    // TODO: typed pair
    // TODO: typed list
}

impl WfTypeGeneric {
    /// assume the data is dereferenced (but may be untyped)
    pub fn parse(data: WfData, _context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        // we know this isn’t a reference, not it is a function. The type-function (linked list, etc) should already be dereferenced.
        match data {
            WfData::WfType(r#type) => return Ok(r#type),
            _other => todo!(),
        };
    }

    pub fn get_type_zid(&self) -> Result<Zid, EvalError> {
        match self {
            Self::WfStandardType(standard) => Ok(standard.inner.identity_ref),
        }
    }
}

impl_wf_data_type!(
    WfTypeGeneric,
    |this| WfData::WfType(this),
    WfStandardType(d)
);

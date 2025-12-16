use std::collections::BTreeMap;

use enum_dispatch::enum_dispatch;

use crate::{
    EvalError, ExecutionContext, Zid,
    data_types::{WfData, WfDataType, types_def::WfStandardType},
};

#[derive(Debug, Clone, PartialEq)]
#[enum_dispatch(WfDataType)]
pub enum WfTypeGeneric {
    WfStandardType(Box<WfStandardType>),
    // TODO: typed pair
    // TODO: typed list
}

impl WfTypeGeneric {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        let data = data.evaluate(context)?;
        // we know this isn’t a reference, not it is a function. The type-function (linked list, etc) should already be dereferenced.
        match data {
            WfData::WfType(r#type) => return Ok(r#type),
            other => todo!(),
        };
    }

    pub fn get_type_zid(&self) -> Result<Zid, EvalError> {
        match self {
            Self::WfStandardType(standard) => Ok(standard.identity_ref),
        }
    }
}

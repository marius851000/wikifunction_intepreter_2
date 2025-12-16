use std::collections::BTreeMap;

use crate::{
    EvalError, ExecutionContext, Zid,
    data_types::{WfData, WfDataType, types_def::WfStandardType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum WfTypeGeneric {
    WfStandardType(Box<WfStandardType>),
    // TODO: typed pair
    // TODO: typed list
}

impl WfTypeGeneric {
    pub fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        match self {
            Self::WfStandardType(d) => d.into_map_no_follow(),
        }
    }

    pub fn get_reference(self, context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        match self {
            Self::WfStandardType(d) => d.get_reference(context),
        }
    }

    pub fn into_map(
        self,
        context: &ExecutionContext,
    ) -> Result<BTreeMap<Zid, WfData>, (EvalError, WfData)> {
        match self {
            Self::WfStandardType(d) => d.into_map(context),
        }
    }

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

    pub fn is_fully_realised(&self) -> bool {
        match self {
            Self::WfStandardType(v) => v.is_fully_realised(),
        }
    }
}

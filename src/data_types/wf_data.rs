use std::{collections::BTreeMap, num::NonZeroU32};

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, Zid,
    data_types::{
        WfBoolean, WfDataType, WfReference, WfString, WfUntyped, types_def::WfTypeGeneric,
    },
};

/// A type that reference one of the data type in a memory efficient way. And also it isn’t dyn-compatible?
#[derive(Debug, Clone)]
pub enum WfData {
    WfBoolean(WfBoolean),
    WfReference(WfReference),
    WfString(WfString),
    WfUntyped(WfUntyped),
    WfType(WfTypeGeneric),
}

impl WfData {
    pub fn new_reference(zid: Zid) -> Self {
        Self::WfReference(WfReference::new(zid))
    }

    pub fn from_map(map: BTreeMap<Zid, WfData>) -> WfData {
        WfData::WfUntyped(WfUntyped::new(map))
    }

    pub fn into_map(
        self,
        context: &ExecutionContext,
    ) -> Result<BTreeMap<Zid, WfData>, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.into_map(context),
            Self::WfReference(d) => d.into_map(context),
            Self::WfString(d) => d.into_map(context),
            Self::WfUntyped(d) => d.into_map(context),
            Self::WfType(d) => d.into_map(context),
        }
    }

    pub fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        match self {
            Self::WfBoolean(d) => d.into_map_no_follow(),
            Self::WfReference(d) => d.into_map_no_follow(),
            Self::WfString(d) => d.into_map_no_follow(),
            Self::WfUntyped(d) => d.into_map_no_follow(),
            Self::WfType(d) => d.into_map_no_follow(),
        }
    }

    pub fn get_reference(self, context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.get_reference(context),
            Self::WfReference(d) => d.get_reference(context),
            Self::WfString(d) => d.get_reference(context),
            Self::WfUntyped(d) => d.get_reference(context),
            Self::WfType(d) => d.get_reference(context),
        }
    }

    pub fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, WfData)> {
        match self {
            Self::WfBoolean(d) => d.evaluate(context),
            Self::WfReference(d) => d.evaluate(context),
            Self::WfString(d) => d.evaluate(context),
            Self::WfUntyped(d) => d.evaluate(context),
            Self::WfType(_d) => todo!("type evaluate"),
        }
    }

    pub fn parse_boolean(self, context: &ExecutionContext) -> Result<WfBoolean, (EvalError, Self)> {
        match self {
            Self::WfBoolean(ready) => Ok(ready),
            other => WfBoolean::parse(other.into_map(context)?, context),
        }
    }

    pub fn parse_type(
        self,
        context: &ExecutionContext,
    ) -> Result<WfTypeGeneric, (EvalError, Self)> {
        match self {
            Self::WfType(ready) => Ok(ready),
            other => WfTypeGeneric::parse(other, context),
        }
    }

    pub fn check_type_by_zid(
        self,
        expected_zid: Zid,
        context: &ExecutionContext,
    ) -> Result<WfData, (EvalError, WfData)> {
        let (got_zid, value) = self.get_type_zid(context)?;
        if expected_zid != got_zid {
            Err((
                EvalError::from_kind(EvalErrorKind::WrongType(got_zid, expected_zid)),
                value,
            ))
        } else {
            Ok(value)
        }
    }

    pub fn get_type_zid(
        self,
        context: &ExecutionContext,
    ) -> Result<(Zid, WfData), (EvalError, WfData)> {
        match &self {
            Self::WfReference(reference) => {
                if let Some(z) = reference.to.get_z() {
                    if z < NonZeroU32::new(100).unwrap() {
                        return Ok((reference.to, self));
                    }
                }
            }
            _ => (),
        }
        // slow path
        let r#type = self.parse_type(context)?;
        match r#type.get_type_zid() {
            Ok(zid) => Ok((zid, WfData::WfType(r#type))),
            Err(e) => Err((e, WfData::WfType(r#type))),
        }
    }
}

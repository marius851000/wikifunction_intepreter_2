use std::collections::BTreeMap;

use map_macro::btree_map;

use crate::{
    EvalError, ExecutionContext, Zid,
    data_types::{WfData, WfDataType, WfString},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfReference {
    pub to: Zid,
}

impl WfReference {
    pub fn new(to: Zid) -> Self {
        Self { to }
    }
}

impl WfDataType for WfReference {
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(9)),
            zid!(9, 1) => WfString::new(self.to.to_string()).into_wf_data(),
        }
    }

    fn is_fully_realised(&self) -> bool {
        return false;
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfReference(self)
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, WfData)> {
        match context.get_global().get_object_value_clone(&self.to) {
            Err(e) => Err((e, self.into_wf_data())),
            Ok(v) => Ok(v),
        }
    }

    fn get_reference(self, _context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        Ok(self.to)
    }
}

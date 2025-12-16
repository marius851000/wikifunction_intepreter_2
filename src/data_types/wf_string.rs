use std::collections::BTreeMap;

use map_macro::btree_map;

use crate::{
    Zid,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone)]
pub struct WfString {
    text: String,
}

impl WfString {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl WfDataType for WfString {
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(6)),
            zid!(6, 1) => self.clone().into_wf_data(),
        }
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfString(self)
    }
}

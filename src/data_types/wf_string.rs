use crate::{
    RcI, Zid,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfString {
    text: RcI<str>,
}

impl WfString {
    pub fn new(text: String) -> Self {
        Self { text: text.into() }
    }
}

impl WfDataType for WfString {
    fn get_identity_key(&self) -> Option<Zid> {
        Some(zid!(6, 1)) // that’s quite a special case, but still a valid identity
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        if key == zid!(1, 1) {
            Some(WfData::new_reference(zid!(6)))
        } else if key == zid!(6, 1) {
            Some(self.clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<Zid> {
        vec![zid!(1, 1), zid!(6, 1)]
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfString(self)
    }
}

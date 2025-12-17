use crate::{
    KeyIndex, RcI,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfString {
    text: RcI<str>,
}

impl WfString {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }
}

impl WfDataType for WfString {
    fn get_identity_key(&self) -> Option<KeyIndex> {
        Some(keyindex!(6, 1)) // that’s quite a special case, but still a valid identity
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(6)))
        } else if key == keyindex!(6, 1) {
            Some(self.clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![keyindex!(1, 1), keyindex!(6, 1)]
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfString(self)
    }
}

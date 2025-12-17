use crate::{
    EvalErrorKind, KeyIndex,
    data_types::{WfData, WfDataType},
};

#[derive(Clone, Debug, PartialEq)]
pub struct WfInvalid {
    reason: EvalErrorKind,
}

impl WfInvalid {
    pub fn new(reason: EvalErrorKind) -> Self {
        Self { reason }
    }
}

impl WfDataType for WfInvalid {
    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        None
    }

    fn get_identity_key(&self) -> Option<KeyIndex> {
        None
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        Vec::new()
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfInvalid(self)
    }

    fn is_fully_realised(&self) -> bool {
        true
    }
}

use crate::{
    RcI, Zid,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfTypedListType {
    r#type: RcI<WfTypeGeneric>, // this Rc is to avoid recursive declaration
}

impl WfDataType for WfTypedListType {
    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfTypedListType(self))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn get_identity_key(&self) -> Option<crate::Zid> {
        None
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        // map that as a function call
        if key == zid!(1, 1) {
            Some(WfData::new_reference(zid!(7)))
        } else if key == zid!(7, 1) {
            Some(WfData::new_reference(zid!(881)))
        } else if key == zid!(881, 1) {
            Some((*self.r#type).clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<Zid> {
        vec![zid!(1, 1), zid!(7, 1), zid!(881, 1)]
    }
}

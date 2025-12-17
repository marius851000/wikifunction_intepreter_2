use crate::{
    RcI, Zid,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, PartialEq, Clone)]
pub struct WfStandardTypeInner {
    pub identity_ref: Zid,
    pub keys: WfData,
    pub validator: WfData,
    pub equality: WfData,
    pub display_function: WfData,
    pub reading_function: WfData,
    pub type_converters_to_code: WfData,
    pub type_converters_from_code: WfData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WfStandardType {
    pub inner: RcI<WfStandardTypeInner>,
}

impl From<WfStandardTypeInner> for WfStandardType {
    fn from(value: WfStandardTypeInner) -> Self {
        Self {
            inner: RcI::new(value),
        }
    }
}

impl WfDataType for WfStandardType {
    fn get_identity_key(&self) -> Option<Zid> {
        Some(zid!(4, 1))
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        if key == zid!(1, 1) {
            Some(WfData::new_reference(zid!(4)))
        } else if key == zid!(4, 1) {
            Some(WfData::new_reference(self.inner.identity_ref))
        } else if key == zid!(4, 2) {
            Some(self.inner.keys.clone())
        } else if key == zid!(4, 3) {
            Some(self.inner.validator.clone())
        } else if key == zid!(4, 4) {
            Some(self.inner.equality.clone())
        } else if key == zid!(4, 5) {
            Some(self.inner.display_function.clone())
        } else if key == zid!(4, 6) {
            Some(self.inner.reading_function.clone())
        } else if key == zid!(4, 7) {
            Some(self.inner.type_converters_to_code.clone())
        } else if key == zid!(4, 8) {
            Some(self.inner.type_converters_from_code.clone())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<Zid> {
        vec![
            zid!(1, 1),
            zid!(4, 1),
            zid!(4, 2),
            zid!(4, 3),
            zid!(4, 4),
            zid!(4, 5),
            zid!(4, 6),
            zid!(4, 7),
            zid!(4, 8),
        ]
    }
    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfStandardType(self))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }
}

use crate::{
    KeyIndex, RcI, Zid,
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
    fn get_identity_key(&self) -> Option<KeyIndex> {
        Some(keyindex!(4, 1))
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(4)))
        } else if key == keyindex!(4, 1) {
            Some(WfData::new_reference(self.inner.identity_ref))
        } else if key == keyindex!(4, 2) {
            Some(self.inner.keys.clone())
        } else if key == keyindex!(4, 3) {
            Some(self.inner.validator.clone())
        } else if key == keyindex!(4, 4) {
            Some(self.inner.equality.clone())
        } else if key == keyindex!(4, 5) {
            Some(self.inner.display_function.clone())
        } else if key == keyindex!(4, 6) {
            Some(self.inner.reading_function.clone())
        } else if key == keyindex!(4, 7) {
            Some(self.inner.type_converters_to_code.clone())
        } else if key == keyindex!(4, 8) {
            Some(self.inner.type_converters_from_code.clone())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![
            keyindex!(1, 1),
            keyindex!(4, 1),
            keyindex!(4, 2),
            keyindex!(4, 3),
            keyindex!(4, 4),
            keyindex!(4, 5),
            keyindex!(4, 6),
            keyindex!(4, 7),
            keyindex!(4, 8),
        ]
    }
    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfStandardType(self))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }
}

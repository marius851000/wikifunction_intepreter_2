use std::collections::BTreeMap;

use map_macro::btree_map;

use crate::{
    Zid,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Clone, Debug, PartialEq)]
pub struct WfStandardType {
    pub identity_ref: Zid,
    keys: WfData,
    validator: WfData,
    equality: WfData,
    display_function: WfData,
    reading_function: WfData,
    type_converters_to_code: WfData,
    type_converters_from_code: WfData,
}

impl WfDataType for WfStandardType {
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        btree_map! {
            zid!(1, 1) => WfData::new_reference(zid!(4)),
            zid!(4, 1) => WfData::new_reference(self.identity_ref),
            zid!(4, 2) => self.keys,
            zid!(4, 3) => self.validator,
            zid!(4, 4) => self.equality,
            zid!(4, 5) => self.display_function,
            zid!(4, 6) => self.reading_function,
            zid!(4, 7) => self.type_converters_to_code,
            zid!(4, 8) => self.type_converters_from_code
        }
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfStandardType(Box::new(self)))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }
}

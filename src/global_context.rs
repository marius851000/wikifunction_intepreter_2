use std::collections::BTreeMap;

use crate::{EvalError, EvalErrorKind, KeyIndex, data_types::WfData};

#[derive(Default)]
pub struct GlobalContext {
    //TODO: I’m not sure wether I wan’t this to request page like an executor or store all data locally. For now, take the second option, it’s simpler. This should be kept in the future, for testing purposes.
    //TODO: persistent objects
    objects: BTreeMap<KeyIndex, WfData>,
}

impl GlobalContext {
    pub fn get_object_value(&self, zid: &KeyIndex) -> Result<WfData, EvalError> {
        self.objects
            .get(zid)
            .ok_or_else(|| EvalError::from_kind(EvalErrorKind::MissingPersistentObject(*zid)))
            .cloned()
    }

    #[cfg(test)]
    pub fn default_for_test() -> Self {
        use map_macro::btree_map;

        use crate::data_types::{
            WfBoolean, WfDataType,
            types_def::{WfStandardType, WfStandardTypeInner},
        };

        Self {
            objects: btree_map! {
                keyindex!(40) => <WfStandardType>::from(WfStandardTypeInner {
                    identity_ref: keyindex!(40),
                    keys: WfData::unvalid(EvalErrorKind::TestData),
                    validator: WfData::unvalid(EvalErrorKind::TestData),
                    equality: WfData::unvalid(EvalErrorKind::TestData),
                    display_function: WfData::unvalid(EvalErrorKind::TestData),
                    reading_function: WfData::unvalid(EvalErrorKind::TestData),
                    type_converters_to_code: WfData::unvalid(EvalErrorKind::TestData),
                    type_converters_from_code: WfData::unvalid(EvalErrorKind::TestData),
                }).into_wf_data(),
                keyindex!(41) => WfBoolean::new(true).into_wf_data(),
                keyindex!(42) => WfBoolean::new(false).into_wf_data()
            },
        }
    }
}

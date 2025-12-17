use std::collections::BTreeMap;

use crate::{EvalError, EvalErrorKind, Zid, data_types::WfData};

#[derive(Default)]
pub struct GlobalContext {
    //TODO: I’m not sure wether I wan’t this to request page like an executor or store all data locally. For now, take the second option, it’s simpler. This should be kept in the future, for testing purposes.
    //TODO: persistent objects
    objects: BTreeMap<Zid, WfData>,
}

impl GlobalContext {
    pub fn get_object_value(&self, zid: &Zid) -> Result<WfData, EvalError> {
        self.objects
            .get(zid)
            .ok_or_else(|| EvalError::from_kind(EvalErrorKind::MissingPersistentObject(*zid)))
            .cloned()
    }

    #[cfg(test)]
    pub fn default_for_test() -> Self {
        use map_macro::btree_map;

        use crate::data_types::{WfBoolean, WfDataType};

        Self {
            objects: btree_map! {
                zid!(41) => WfBoolean::new(true).into_wf_data(),
                zid!(42) => WfBoolean::new(false).into_wf_data()
            },
        }
    }
}

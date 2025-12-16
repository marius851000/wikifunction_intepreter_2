use std::collections::BTreeMap;

use crate::{EvalError, EvalErrorKind, Zid, data_types::WfData};

#[derive(Default)]
pub struct GlobalContext {
    //TODO: I’m not sure wether I wan’t this to request page like an executor or store all data locally. For now, take the second option, it’s simpler. This should be kept in the future, for testing purposes.
    //TODO: persistent objects
    objects: BTreeMap<Zid, WfData>,
}

impl GlobalContext {
    pub fn get_object_value_clone(&self, zid: &Zid) -> Result<WfData, EvalError> {
        Ok(self
            .objects
            .get(zid)
            .ok_or_else(|| {
                EvalError::from_kind(EvalErrorKind::MissingPersistentObject(zid.clone()))
            })
            .map(|x| x.clone())?)
    }
}

use std::collections::BTreeMap;

use crate::{
    EvalError, ExecutionContext, RcI, Zid,
    data_types::{WfBoolean, WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfUntyped {
    entry: RcI<BTreeMap<Zid, WfData>>,
}

impl WfUntyped {
    pub fn new(entry: BTreeMap<Zid, WfData>) -> Self {
        Self {
            entry: RcI::new(entry),
        }
    }

    /// Will need fail (outside of eventual panic due to program error related to wrongly implemented interface)
    pub fn parse(data: WfData) -> Self {
        match data {
            WfData::WfUntyped(this) => return this,
            _ => (),
        };

        let mut result = BTreeMap::new();
        for key in data.list_keys() {
            result.insert(key, data.get_key(key).unwrap().clone());
        }
        Self {
            entry: RcI::new(result),
        }
    }
}

impl WfDataType for WfUntyped {
    fn get_identity_key(&self) -> Option<Zid> {
        todo!("identity for Untyped? Is that even possible without consumption?")
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        self.entry.get(&key).map(|x| x.clone())
    }

    fn list_keys(&self) -> Vec<Zid> {
        self.entry.keys().map(|x| *x).collect()
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfUntyped(self)
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        let z1k1 = match self.entry.get(&zid!(1, 1)) {
            None => return Err((EvalError::missing_key(zid!(1, 1)), self)),
            Some(z1k1) => z1k1.clone(),
        };
        match z1k1.get_type_zid(context) {
            Ok((type_zid, _z1k1)) => {
                if type_zid == zid!(40) {
                    match WfBoolean::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                }
                todo!();
            }
            // ignore the error. A more complete analysis will be done that will itself return an error as appropriate
            Err((_e, z1k1)) => {
                let r#type = match z1k1.parse_type(context) {
                    Err((e, _)) => {
                        return Err((e.inside(zid!(1, 1)), self));
                    }
                    Ok(r#type) => r#type,
                };

                match r#type {
                    WfTypeGeneric::WfStandardType(_) => {
                        unreachable!("standard type with zid should be reached earlier!")
                    }
                }
            }
        }
    }

    fn get_reference(self, _context: &ExecutionContext) -> Result<Zid, (EvalError, Self)> {
        todo!();
    }

    //todo: get_reference: look up we are function call or an argument reference
}

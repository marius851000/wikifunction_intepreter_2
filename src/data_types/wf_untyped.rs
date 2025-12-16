use std::collections::BTreeMap;

use crate::{
    EvalError, ExecutionContext, Zid,
    data_types::{WfBoolean, WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfUntyped {
    entry: BTreeMap<Zid, WfData>,
}

impl WfUntyped {
    pub fn new(entry: BTreeMap<Zid, WfData>) -> Self {
        Self { entry }
    }
}

impl WfDataType for WfUntyped {
    // should not resolve themself if they are reference
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData> {
        self.entry
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfUntyped(self)
    }

    fn evaluate(mut self, context: &ExecutionContext) -> Result<WfData, (EvalError, WfData)> {
        let z1k1 = match self.entry.remove(&zid!(1, 1)) {
            None => return Err((EvalError::missing_key(zid!(1, 1)), self.into_wf_data())),
            Some(z1k1) => z1k1,
        };
        match z1k1.get_type_zid(context) {
            Ok((type_zid, z1k1)) => {
                self.entry.insert(zid!(1, 1), z1k1);

                if type_zid == zid!(40) {
                    return Ok(WfBoolean::parse(self.entry, context)?.into_wf_data());
                }
                todo!();
            }
            // ignore the error. A more complete analysis will be done that will itself return an error as appropriate
            Err((_e, z1k1)) => {
                let r#type = match z1k1.parse_type(context) {
                    Err((e, value)) => {
                        return Err((e.inside(zid!(1, 1)), {
                            self.entry.insert(zid!(1, 1), value);
                            WfData::from_map(self.entry)
                        }));
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

    fn get_reference(self, _context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        todo!();
    }

    //todo: get_reference: look up we are function call or an argument reference
}

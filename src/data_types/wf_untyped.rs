use std::collections::BTreeMap;

use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI, Zid,
    data_types::{
        WfArgumentReference, WfBoolean, WfData, WfDataType, WfFunction, WfFunctionCall,
        WfImplementation, WfTestCase,
        types_def::{WfStandardType, WfTypeGeneric},
        wf_function_call::FunctionCallOrType,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfUntyped {
    entry: RcI<BTreeMap<KeyIndex, WfData>>,
}

impl WfUntyped {
    pub fn new(entry: BTreeMap<KeyIndex, WfData>) -> Self {
        Self {
            entry: RcI::new(entry),
        }
    }

    /// Will need fail (outside of eventual panic due to program error related to wrongly implemented interface)
    pub fn parse(data: WfData) -> Self {
        if let WfData::WfUntyped(this) = data {
            return this;
        };

        let mut result = BTreeMap::new();
        for key in data.list_keys() {
            result.insert(key, data.get_key(key).unwrap().clone());
        }
        Self {
            entry: RcI::new(result),
        }
    }

    /// If recurse is false, this function will not evaluate function call, reference and co
    fn evaluate_inner(
        self,
        context: &ExecutionContext,
        recurse: bool,
    ) -> Result<WfData, (EvalError, Self)> {
        let z1k1 = match self.entry.get(&keyindex!(1, 1)) {
            Some(z1k1) => z1k1.clone(),
            _ => return Err((EvalError::missing_key(keyindex!(1, 1)), self)),
        };
        match z1k1.get_type_zid(context) {
            Ok((type_zid, _z1k1)) => {
                if type_zid == zid!(4) {
                    match WfStandardType::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                }
                if type_zid == zid!(7) {
                    // function call, but may also be one of the typed type.
                    let fc_or_type = match WfFunctionCall::parse(self.into_wf_data(), context) {
                        Ok(v) => v,
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    };
                    let function_call = match fc_or_type {
                        FunctionCallOrType::FunctionCall(f) => f,
                        FunctionCallOrType::Type(t) => return Ok(t.into_wf_data()),
                    };
                    if !recurse {
                        return Ok(function_call.into_wf_data());
                    }

                    match function_call.evaluate(context) {
                        Ok(v) => return Ok(v),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data.into_wf_data()))),
                    }
                } else if type_zid == zid!(8) {
                    match WfFunction::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                } else if type_zid == zid!(14) {
                    match WfImplementation::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                } else if type_zid == zid!(18) {
                    match WfArgumentReference::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                } else if type_zid == zid!(20) {
                    let test_case = match WfTestCase::parse(self.into_wf_data(), context) {
                        Ok(v) => v,
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    };
                    match test_case.evaluate(context) {
                        Ok(v) => return Ok(v),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data.into_wf_data()))),
                    };
                } else if type_zid == zid!(40) {
                    match WfBoolean::parse(self.into_wf_data(), context) {
                        Ok(v) => return Ok(v.into_wf_data()),
                        Err((e, data)) => return Err((e, WfUntyped::parse(data))),
                    }
                }
                todo!("parsing for {}", type_zid);
            }
            // ignore the error. A more complete analysis will be done that will itself return an error as appropriate
            Err((_e, z1k1)) => {
                let r#type = match z1k1.parse_type(context) {
                    Err((e, _)) => {
                        return Err((e.inside_key(keyindex!(1, 1)), self));
                    }
                    Ok(r#type) => r#type,
                };

                match r#type {
                    WfTypeGeneric::WfStandardType(_) => {
                        unreachable!("standard type with zid should be reached earlier!")
                    }
                    WfTypeGeneric::WfTypedListType(_) => todo!("zobject typed list parsing"),
                }
            }
        }
    }
}

impl WfDataType for WfUntyped {
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        todo!("identity for Untyped? Is that even possible without consumption?")
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        self.entry.get(&key).cloned()
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        self.entry.keys().copied().collect()
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfUntyped(self)
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        self.evaluate_inner(context, true)
    }

    fn get_reference(
        self,
        context: &ExecutionContext,
    ) -> Result<(Zid, WfData), (EvalError, WfData)> {
        match self.evaluate_inner(context, false) {
            Err((e, this)) => return Err((e, this.into_wf_data())),
            Ok(this) => Ok(this.get_reference(context)?),
        }
    }
    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        info: &I,
        context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        let mut new_entries = BTreeMap::new();
        for (k, v) in self.entry.iter() {
            new_entries.insert(
                k.clone(),
                v.clone()
                    .substitute_function_arguments(info, context)
                    .map_err(|e| e.inside_key(k.clone()))?,
            );
        }
        Ok(Self::new(new_entries).into_wf_data())
    }
}

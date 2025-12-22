use std::fmt::Debug;

use crate::{
    EvalError, ExecutionContext, KeyIndex, Zid,
    data_types::{WfData, WfDataType},
};

#[derive(Clone, PartialEq)]
pub struct WfReference {
    pub to: Zid,
}

impl WfReference {
    pub fn new(to: Zid) -> Self {
        Self { to }
    }
}

impl Debug for WfReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WfReference({})", self.to)
    }
}

impl WfDataType for WfReference {
    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        unreachable!("shouldn’t access a reference directly (for ref {:?})", self);
    }

    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        unreachable!("shouldn’t access a reference directly (for ref {:?})", self);
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        unreachable!("shouldn’t access a reference directly (for ref {:?})", self);
    }

    fn should_be_evaluated_before_parsing(&self) -> bool {
        return true;
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfReference(self)
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        match context.get_global().get_object_value(&self.to) {
            Err(e) => Err((e, self)),
            Ok(v) => match v.evaluate(context) {
                Err((e, _data)) => Err((
                    e.inside_key(keyindex!(9, 1)).inside_reference_to(self.to),
                    self,
                )),
                Ok(v) => Ok(v),
            },
        }
    }

    fn get_reference(
        self,
        _context: &ExecutionContext,
    ) -> Result<(Zid, WfData), (EvalError, WfData)> {
        Ok((self.to, self.into_wf_data()))
    }

    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        _info: &I,
        _context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        Ok(self.into_wf_data())
    }
}

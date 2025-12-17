use crate::{
    EvalError, ExecutionContext, KeyIndex,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfReference {
    pub to: KeyIndex,
}

impl WfReference {
    pub fn new(to: KeyIndex) -> Self {
        Self { to }
    }
}

impl WfDataType for WfReference {
    fn get_identity_key(&self) -> Option<KeyIndex> {
        unreachable!("shouldn’t access a reference directly")
    }

    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        unreachable!("shouldn’t access a reference directly")
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        unreachable!("shouldn’t access a reference directly")
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
                Err((e, _data)) => Err((e.inside(keyindex!(9, 1)), self)),
                Ok(v) => Ok(v),
            },
        }
    }

    fn get_reference(self, _context: &ExecutionContext) -> Result<KeyIndex, (EvalError, Self)> {
        Ok(self.to)
    }
}

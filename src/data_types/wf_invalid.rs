use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex,
    data_types::{WfData, WfDataType},
    eval_error::TraceEntry,
    util::MaybeVec,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WfInvalid {
    reason: EvalErrorKind,
}

impl WfInvalid {
    pub fn new(reason: EvalErrorKind) -> Self {
        Self { reason }
    }
}

impl WfDataType for WfInvalid {
    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        None
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        Vec::new()
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfInvalid(self)
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn evaluate_one_step(
        self,
        _context: &ExecutionContext,
    ) -> Result<(WfData, bool, MaybeVec<TraceEntry>), (EvalError, Self)> {
        return Err((EvalError::from_kind(self.reason.clone()), self));
    }

    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        _info: &I,
        _context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        Ok(self.into_wf_data())
    }
}

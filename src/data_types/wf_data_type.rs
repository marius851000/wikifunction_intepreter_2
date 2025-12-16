use std::{collections::BTreeMap, fmt::Debug};

use crate::{EvalError, EvalErrorKind, ExecutionContext, Zid, data_types::WfData};

pub trait WfDataType: Debug + Clone {
    fn into_map_no_follow(self) -> BTreeMap<Zid, WfData>;
    fn into_wf_data(self) -> WfData;
    /// used to know that this structure is one of the final type. Used to know that inequality mean two object with this property does not represent the same thing.
    fn is_fully_realised(&self) -> bool;

    /// Follow references and all that -- recursively. Default to returning self.
    /// Also need to guarantee the data is correct. It shouldn’t return a WfUntyped.
    fn evaluate(self, _context: &ExecutionContext) -> Result<WfData, (EvalError, WfData)> {
        Ok(self.into_wf_data())
    }
    fn into_map(
        self,
        context: &ExecutionContext,
    ) -> Result<BTreeMap<Zid, WfData>, (EvalError, WfData)> {
        Ok(self.evaluate(context)?.into_map_no_follow())
    }

    fn get_reference(self, _context: &ExecutionContext) -> Result<Zid, (EvalError, WfData)> {
        Err((
            EvalError::from_kind(EvalErrorKind::NotAReference),
            self.into_wf_data(),
        ))
    }
}

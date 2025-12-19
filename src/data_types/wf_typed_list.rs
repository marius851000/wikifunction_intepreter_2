use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{MaybeEvaluated, WfData, WfDataType, types_def::WfTypeGeneric},
};

/// The type may be either evaluated and checked to be valid, or unevaluated.
/// When evaluating a WfTypedList whose type is unparsed, it parse it, but does not further check the entries correspond to that type until they are themselve evaluated.
#[derive(Debug, Clone, PartialEq)]
// into two separated Rc cause this way I can change the type without cloning the entries
pub struct WfTypedList {
    pub entries: RcI<Vec<WfData>>,
    // directly point to the inner data, not a WfTypedListType (unless for a list of list)
    pub inner_type: RcI<MaybeEvaluated<WfTypeGeneric>>,
}

impl WfTypedList {
    pub fn new(r#type: MaybeEvaluated<WfTypeGeneric>, entries: Vec<WfData>) -> Self {
        Self {
            inner_type: RcI::new(r#type),
            entries: RcI::new(entries),
        }
    }

    pub fn parse(data: WfData, _context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        match data {
            WfData::WfTypedList(d) => return Ok(d),
            _ => (),
        };
        todo!("parsing list from keys from {:?}", data);
    }
}

impl WfDataType for WfTypedList {
    fn into_wf_data(self) -> WfData {
        WfData::WfTypedList(self)
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn get_key(&self, _key: KeyIndex) -> Option<WfData> {
        todo!();
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        todo!();
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn should_be_evaluated_before_parsing(&self) -> bool {
        if let MaybeEvaluated::Unchecked(_) = &*self.inner_type {
            true
        } else {
            false
        }
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        if let MaybeEvaluated::Unchecked(_) = &*self.inner_type {
            // two level so we avoid this useless clone if already checked
            if let MaybeEvaluated::Unchecked(type_unchecked) = (*self.inner_type).clone() {
                let type_evaluated = match type_unchecked.evaluate(context) {
                    Ok(v) => v,
                    Err((e, _)) => return Err((e.inside(keyindex!(1, 1)), self)),
                };

                let checked_type = match WfTypeGeneric::parse(type_evaluated, context) {
                    Ok(v) => v,
                    Err((e, _)) => return Err((e.inside(keyindex!(1, 1)), self)),
                };

                Ok((Self {
                    entries: self.entries,
                    inner_type: RcI::new(MaybeEvaluated::Valid(checked_type)),
                })
                .into_wf_data())
            } else {
                unreachable!();
            }
        } else {
            Ok(self.into_wf_data())
        }
    }
}

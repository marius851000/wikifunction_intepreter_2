use crate::{
    EvalError, ExecutionContext, KeyIndex, RcI,
    data_types::{WfData, WfDataType},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfUncheckedTypedListInner {
    pub r#type: WfData, //TODO: make than an enum of WfData, WfGenericType so UncheckedTypedList could be turned to just UncheckedList with an unchecked variant. That might need to put the vec behing it’s own Rc.
    // store them like this here. This is just a temporary representation until evaluate is called and the values are checked.
    // Or we might make reference to typed list entry?
    pub entries: Vec<WfData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WfUncheckedTypedList(pub RcI<WfUncheckedTypedListInner>);

impl WfDataType for WfUncheckedTypedList {
    fn into_wf_data(self) -> WfData {
        WfData::WfUncheckedTypedList(self)
    }

    fn get_identity_key(&self) -> Option<KeyIndex> {
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

    fn evaluate(self, _context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        todo!();
    }
}

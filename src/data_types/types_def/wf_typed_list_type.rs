use crate::{
    RcI, Zid,
    data_types::{WfData, WfDataType, types_def::WfTypeGeneric},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfTypedListType {
    r#type: RcI<WfTypeGeneric>, // this Rc is to avoid recursive declaration
}

impl WfDataType for WfTypedListType {
    fn into_wf_data(self) -> WfData {
        WfData::WfType(WfTypeGeneric::WfTypedListType(self))
    }

    fn is_fully_realised(&self) -> bool {
        true
    }

    fn get_identity_key(&self) -> Option<crate::Zid> {
        None
    }

    fn get_key(&self, key: Zid) -> Option<WfData> {
        // map that as a function call
        if key == zid!(1, 1) {
            Some(WfData::new_reference(zid!(7)))
        } else if key == zid!(7, 1) {
            Some(WfData::new_reference(zid!(881)))
        } else if key == zid!(881, 1) {
            Some((*self.r#type).clone().into_wf_data())
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<Zid> {
        vec![zid!(1, 1), zid!(7, 1), zid!(881, 1)]
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        ExecutionContext, GlobalContext, RcI,
        data_types::{
            WfData, WfDataType,
            types_def::{WfTypeGeneric, WfTypedListType},
        },
    };

    #[test]
    fn test_get_list_key() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(Arc::new(global_context));
        let boolean_type = WfData::new_reference(zid!(40)).evaluate(&context).unwrap();
        let boolean_type_clone = boolean_type.clone();
        let test_typed_list_typed = WfTypedListType {
            r#type: RcI::new(WfTypeGeneric::parse(boolean_type, &context).unwrap()),
        };
        assert!(test_typed_list_typed.list_keys().contains(&zid!(7, 1)));
        assert!(test_typed_list_typed.list_keys().contains(&zid!(881, 1)));
        assert!(
            test_typed_list_typed
                .get_key(zid!(881, 1))
                .unwrap()
                .into_wf_data()
                .equality(boolean_type_clone, &context)
                .unwrap()
        );
        assert!(
            test_typed_list_typed
                .get_key(zid!(1, 1))
                .unwrap()
                .into_wf_data()
                .equality(WfData::new_reference(zid!(7)), &context)
                .unwrap()
        );
    }
}

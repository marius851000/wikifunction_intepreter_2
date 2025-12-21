use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI, Zid,
    data_types::{
        ImplementationByKind, WfData, WfDataType, WfImplementation, WfTypedList,
        types_def::WfTypeGeneric, util::SubstitutionInfo,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfFunctionInner {
    pub arguments: WfTypedList, //TODO: WfArguments
    pub return_type: WfTypeGeneric,
    pub testers: WfData,              // unevaluated
    pub implementations: WfTypedList, //TODO: WfImplementations (or Typed List). Or keep it WfData so can be parsed without accessing the data?
    pub identity: Zid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WfFunction(pub RcI<WfFunctionInner>);

impl WfFunction {
    pub fn parse(data: WfData, context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        if let WfData::WfFunction(v) = data {
            return Ok(v);
        }
        data.assert_evaluated();

        // check type
        match data.get_key(keyindex!(1, 1)) {
            Some(r#type) => {
                if let Err((e, _)) = r#type.check_identity_zid(context, zid!(8)) {
                    return Err((e.inside(keyindex!(1, 1)), data));
                }
            }
            _ => return Err((EvalError::missing_key(keyindex!(1, 1)), data)),
        };

        let arguments = match data.get_key_err(keyindex!(8, 1)) {
            Err(e) => return Err((e, data)),
            Ok(v) => match v.evaluate(context) {
                Err((e, _)) => return Err((e.inside(keyindex!(8, 1)), data)),
                Ok(v) => match WfTypedList::parse(v, context) {
                    Err((e, _)) => return Err((e.inside(keyindex!(8, 1)), data)),
                    Ok(v) => v,
                },
            },
        };

        let return_type = match data.get_key_err(keyindex!(8, 2)) {
            Err(e) => return Err((e, data)),
            Ok(v) => match v.evaluate(context) {
                Err((e, _)) => return Err((e.inside(keyindex!(8, 2)), data)),
                Ok(v) => match WfTypeGeneric::parse(v, context) {
                    Err((e, _)) => return Err((e.inside(keyindex!(8, 2)), data)),
                    Ok(v) => v,
                },
            },
        };

        let testers = match data.get_key_err(keyindex!(8, 3)) {
            Err(e) => return Err((e, data)),
            Ok(v) => v,
        };

        //TODO: verify type of this list to match expected value
        let implementations = match data.get_key_err(keyindex!(8, 4)) {
            Err(e) => return Err((e, data)),
            Ok(v) => match v.evaluate(context) {
                Err((e, _)) => return Err((e.inside(keyindex!(8, 4)), data)),
                Ok(v) => match WfTypedList::parse(v, context) {
                    Err((e, _)) => return Err((e.inside(keyindex!(8, 4)), data)),
                    Ok(v) => v,
                },
            },
        };

        let identity = match data.get_identity_zid(&context, keyindex!(8, 5)) {
            Ok(k) => k,
            Err(e) => return Err((e, data)),
        };

        Ok(WfFunction(RcI::new(WfFunctionInner {
            arguments,
            return_type,
            testers,
            implementations,
            identity,
        })))
    }

    pub fn get_preffered_implementation(
        &self,
        context: &ExecutionContext,
    ) -> Result<WfImplementation, EvalError> {
        let mut implementations = Vec::new();
        for (pos, implementation) in self.0.implementations.entries.iter().enumerate() {
            let implementation = match implementation.clone().evaluate(context) {
                Ok(v) => v,
                Err((e, _)) => return Err(e.inside(keyindex!(8, 4)).inside_list(pos)),
            };

            let implementation = match WfImplementation::parse(implementation, context) {
                Ok(i) => i,
                Err((e, _)) => return Err(e.inside(keyindex!(8, 4)).inside_list(pos)),
            };

            implementations.push(implementation);
        }

        let mut best_builtin_implementation = None;
        let mut best_composition_implementation = None;
        let mut best_code_implementation = None;

        for implementation in implementations.into_iter() {
            //TODO: better implementation choice
            match implementation.0.r#impl {
                ImplementationByKind::Composition(_) => {
                    best_composition_implementation = Some(implementation)
                }
                ImplementationByKind::Code(_) => best_code_implementation = Some(implementation),
                ImplementationByKind::Builtin(_) => {
                    best_builtin_implementation = Some(implementation)
                }
            }
        }

        if let Some(r#impl) = best_builtin_implementation {
            Ok(r#impl)
        } else if let Some(r#impl) = best_composition_implementation {
            Ok(r#impl)
        } else if let Some(r#impl) = best_code_implementation {
            Ok(r#impl)
        } else {
            Err(EvalError::from_kind(
                EvalErrorKind::NoImplementationForFunction,
            ))
        }
    }
}

impl WfDataType for WfFunction {
    fn into_wf_data(self) -> WfData {
        WfData::WfFunction(self)
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        Some(keyindex!(8, 5))
    }

    fn is_fully_realised(&self) -> bool {
        self.0.arguments.is_fully_realised()
            && self.0.return_type.is_fully_realised()
            && self.0.testers.is_fully_realised()
            && self.0.implementations.is_fully_realised()
    }

    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(8)))
        } else if key == keyindex!(8, 1) {
            Some(self.0.arguments.clone().into_wf_data())
        } else if key == keyindex!(8, 2) {
            Some(self.0.return_type.clone().into_wf_data())
        } else if key == keyindex!(8, 3) {
            Some(self.0.testers.clone())
        } else if key == keyindex!(8, 4) {
            Some(self.0.implementations.clone().into_wf_data())
        } else if key == keyindex!(8, 5) {
            Some(WfData::new_reference(self.0.identity))
        } else {
            None
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        vec![
            keyindex!(1, 1),
            keyindex!(8, 1),
            keyindex!(8, 2),
            keyindex!(8, 3),
            keyindex!(8, 4),
            keyindex!(8, 5),
        ]
    }

    fn substitute_function_arguments<I: SubstitutionInfo>(
        self,
        _info: &I,
        _context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        todo!(); // given the identity relation, this should not normally be called on a function. return Self or still propagate thought implementations, arguments, return type and testers?
    }
}

#[cfg(test)]
mod tests {
    use std::u32;

    use map_macro::btree_map;

    use crate::{
        EvalErrorKind, ExecutionContext, GlobalContext, RcI,
        data_types::{MaybeEvaluated, WfData, WfDataType, WfFunction, WfTypedList},
    };

    #[test]
    fn test_parse_function() {
        let mut global_context = GlobalContext::default_for_test();

        let unv = WfData::unvalid(EvalErrorKind::TestData);
        let function_unparsed = WfData::from_map(btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(8)),
            keyindex!(8, 1) => WfTypedList::new(MaybeEvaluated::Unchecked(WfData::new_reference(zid!(3))), Vec::new()).into_wf_data(),
            keyindex!(8, 2) => WfData::new_reference(zid!(40)),
            keyindex!(8, 3) => unv.clone(),
            keyindex!(8, 4) => WfTypedList::new(MaybeEvaluated::Unchecked(WfData::new_reference(zid!(14))), Vec::new()).into_wf_data(),
            keyindex!(8, 5) => WfData::new_reference(zid!(u32::MAX))
        });
        global_context.add_direct_no_persistent_data(zid!(u32::MAX), function_unparsed.clone());
        // need to be here for recursive identity lookup.

        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let function = WfFunction::parse(function_unparsed.clone(), &context).unwrap();

        //assert_eq!(function.0.arguments, unv.clone());
        assert_eq!(function.0.return_type.get_type_zid().unwrap(), zid!(40));
        assert_eq!(function.0.testers, unv.clone());
        //assert_eq!(function.0.implementations, unv.clone());
        assert_eq!(function.0.identity, zid!(u32::MAX));
    }
}

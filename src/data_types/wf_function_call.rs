use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{
        ImplementationByKind, WfData, WfDataType, WfFunction,
        types_def::{WfTypeGeneric, WfTypedListType},
        util::SubstitutionInfo,
    },
    functions::dispatch_builtins,
};

#[derive(Debug)]
pub enum FunctionCallOrType {
    FunctionCall(WfFunctionCall),
    Type(WfTypeGeneric),
}

#[derive(Debug, PartialEq, Clone)]
pub struct WfFunctionCallInner {
    pub function: WfFunction,
    pub args: Vec<WfData>, // unevaluated
}

#[derive(Debug, PartialEq, Clone)]
pub struct WfFunctionCall(pub RcI<WfFunctionCallInner>);

#[allow(unused_assignments)] //TODO: remove once parse is fully implemented
impl WfFunctionCall {
    pub fn parse(
        data: WfData,
        context: &ExecutionContext,
    ) -> Result<FunctionCallOrType, (EvalError, WfData)> {
        Self::parse_inner(data, context, None)
    }

    pub fn parse_for_test(
        data: WfData,
        context: &ExecutionContext,
        substitute_first_arg: WfData,
    ) -> Result<FunctionCallOrType, (EvalError, WfData)> {
        Self::parse_inner(data, context, Some(substitute_first_arg))
    }

    /// May return a type as a shortcut for typed function function call
    pub fn parse_inner(
        mut data: WfData,
        context: &ExecutionContext,
        mut substitute_first_arg: Option<WfData>,
    ) -> Result<FunctionCallOrType, (EvalError, WfData)> {
        if let WfData::WfFunctionCall(v) = data {
            return Ok(FunctionCallOrType::FunctionCall(v));
        }
        data.assert_evaluated();

        // check type of this
        match data.get_key_err(keyindex!(1, 1)) {
            Ok(this_type) => {
                if let Err((e, _)) = this_type.check_identity_zid(context, zid!(7)) {
                    return Err((e.inside_key(keyindex!(1, 1)), data));
                }
            }
            Err(e) => return Err((e, data)),
        };

        // get function reference (just as shortcut for checked typed)
        let function_unevaluated = match data.get_key_err(keyindex!(7, 1)) {
            Err(e) => return Err((e, data)),
            Ok(function_uneval) => function_uneval,
        };

        let (function_identity, function_unevaluated) =
            match function_unevaluated.get_reference(context) {
                Err((_, d)) => (None, d),
                Ok((reference, f)) => (Some(reference), f),
            };

        if let Some(function_reference) = function_identity {
            if function_reference == zid!(881) {
                data = match WfTypedListType::parse(data, context) {
                    Ok(typed_list_type) => {
                        return Ok(FunctionCallOrType::Type(WfTypeGeneric::WfTypedListType(
                            typed_list_type,
                        )));
                    }
                    Err((_, d)) => d,
                };
            }
        }

        let function_evaluated = match function_unevaluated.evaluate(context) {
            Ok(v) => v,
            Err((e, _)) => return Err((e.inside_key(keyindex!(7, 1)), data)),
        };

        let function = match WfFunction::parse(function_evaluated, context) {
            Ok(v) => v,
            Err((e, _)) => return Err((e.inside_key(keyindex!(7, 1)), data)),
        };

        let function_identity = function.0.identity;

        let num_argument = function.0.arguments.len();

        let num_argument_plus_one_as_u32: u32 = match num_argument.saturating_add(1).try_into() {
            Ok(v) => v,
            _ => {
                return Err((
                    EvalError::from_kind(EvalErrorKind::TooManyArgsInFunction),
                    data,
                ));
            }
        };

        let mut args = Vec::new();
        //TODO: I’m not sure this is correct. I think keys also might be listed as K1, K2, etc... If that is changed, remember to change get_key and list_key
        for i in 1..num_argument_plus_one_as_u32 {
            let parameter = if i == 1
                && let Some(substitute_first_arg) = substitute_first_arg.take()
            {
                substitute_first_arg
            } else {
                let key = KeyIndex::from_u32s_panic(Some(function_identity.0.into()), Some(i)); // should not panic, z is already a valid Zid and i is always strictly greater than 0
                match data.get_key_err(key) {
                    Ok(k) => k,
                    Err(e) => return Err((e, data)),
                }
            };
            args.push(parameter)
        }

        Ok(FunctionCallOrType::FunctionCall(Self(RcI::new(
            WfFunctionCallInner { function, args },
        ))))
    }
}

impl WfDataType for WfFunctionCall {
    fn get_key(&self, key: KeyIndex) -> Option<WfData> {
        if key == keyindex!(1, 1) {
            Some(WfData::new_reference(zid!(7)))
        } else if key == keyindex!(7, 1) {
            Some(self.0.function.clone().into_wf_data())
        } else {
            for (pos, value) in self.0.args.iter().enumerate() {
                if key
                    == KeyIndex::from_u32s_panic(
                        Some(self.0.function.0.identity.0.get()),
                        Some(pos as u32 + 1),
                    )
                {
                    return Some(value.clone());
                }
            }
            return None;
        }
    }

    fn list_keys(&self) -> Vec<KeyIndex> {
        let mut result = vec![keyindex!(1, 1), keyindex!(7, 1)];
        for pos in 0..self.0.args.len() {
            result.push(KeyIndex::from_u32s_panic(
                Some(self.0.function.0.identity.0.get()),
                Some(pos as u32 + 1),
            ))
        }
        result
    }

    fn get_identity_zid_key(&self) -> Option<KeyIndex> {
        None
    }

    fn into_wf_data(self) -> WfData {
        WfData::WfFunctionCall(self)
    }

    fn is_fully_realised(&self) -> bool {
        false
    }

    fn evaluate(self, context: &ExecutionContext) -> Result<WfData, (EvalError, Self)> {
        let implementation = match self
            .0
            .function
            .get_preffered_implementation(context)
            .map_err(|e| e.inside_key(keyindex!(7, 1)))
        {
            Ok(i) => i,
            Err(e) => return Err((e, self)),
        };

        match &implementation.0.r#impl {
            ImplementationByKind::Composition(inner) => {
                let inner_substituted =
                    match inner.clone().substitute_function_arguments(&self, context) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err((
                                e.inside_function_call(
                                    self.0.function.0.identity,
                                    //implementation.0.r#impl.clone(),
                                ),
                                self,
                            ));
                        }
                    };
                let inner = match inner_substituted.evaluate(context) {
                    Ok(v) => v,
                    Err((e, _)) => {
                        return Err((
                            e.inside_function_call(
                                self.0.function.0.identity,
                                //implementation.0.r#impl.clone(),
                            ),
                            self,
                        ));
                    }
                };
                Ok(inner)
            }
            ImplementationByKind::Code(_) => {
                return Err((
                    EvalError::unimplemented(format!(
                        "code implementaiton (for {})",
                        self.0.function.0.identity
                    )),
                    self,
                ));
            }
            ImplementationByKind::Builtin(_) => {
                match dispatch_builtins(self.0.function.0.identity, &self, context) {
                    Ok(v) => Ok(v),
                    Err(e) => Err((e, self)),
                }
            }
        }
    }

    fn should_be_evaluated_before_parsing(&self) -> bool {
        true
    }

    fn substitute_function_arguments<I: super::util::SubstitutionInfo>(
        self,
        info: &I,
        context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        let mut new_args = Vec::with_capacity(self.0.args.len());
        for arg in self.0.args.iter() {
            let arg = arg
                .clone()
                .substitute_function_arguments(info, context)
                .map_err(|e| e.inside_key(todo!()))?;
            new_args.push(arg);
        }
        Ok(Self(RcI::new(WfFunctionCallInner {
            function: self.0.function.clone(),
            args: new_args,
        }))
        .into_wf_data())
    }
}

impl SubstitutionInfo for WfFunctionCall {
    fn get_for_pos(&self, pos: u32) -> Result<WfData, EvalError> {
        match self.0.args.get(pos as usize) {
            Some(v) => Ok(v.clone()),
            _ => Err(EvalError::from_kind(
                EvalErrorKind::ArgumentReferenceTooLarge(pos),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use map_macro::btree_map;

    use crate::{
        ExecutionContext, GlobalContext, KeyIndex, RcI,
        data_types::{
            WfBoolean, WfData, WfDataType, WfFunctionCall, wf_function_call::FunctionCallOrType,
        },
    };

    fn get_unparsed_boolean_equality_true_false() -> BTreeMap<KeyIndex, WfData> {
        btree_map! {
            keyindex!(1, 1) => WfData::new_reference(zid!(7)),
            keyindex!(7, 1) => WfData::new_reference(zid!(844)),
            // boolean equality function
            keyindex!(844, 1) => WfBoolean::new(false).into_wf_data(),
            keyindex!(844, 2) => WfBoolean::new(true).into_wf_data(),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let unparsed_tree = get_unparsed_boolean_equality_true_false();
        let unparsed = WfData::from_map(unparsed_tree.clone());

        let parsed = WfFunctionCall::parse(unparsed.clone(), &context).unwrap();
        let parsed = if let FunctionCallOrType::FunctionCall(fc) = parsed {
            fc
        } else {
            panic!();
        };

        assert_eq!(parsed.0.args.len(), 2);
        assert_eq!(
            parsed.0.args,
            vec![
                WfBoolean::new(false).into_wf_data(),
                WfBoolean::new(true).into_wf_data()
            ]
        );
        assert!(
            parsed
                .0
                .function
                .clone()
                .into_wf_data()
                .equality(WfData::new_reference(zid!(844)), &context)
                .unwrap()
        );
        //TODO: this actually run the function, which wouldn’t be a bad idea if it were implemented
        //assert!(parsed.into_wf_data().equality(unparsed, &context).unwrap());

        let mut invalid_unparsed = unparsed_tree;
        invalid_unparsed.remove(&keyindex!(844, 1));
        WfData::from_map(invalid_unparsed)
            .evaluate(&context)
            .unwrap_err();
    }

    #[test]
    fn test_evaluate_builtin_function() {
        let global_context = GlobalContext::default_for_test();
        let context = ExecutionContext::default_for_global(RcI::new(global_context));

        let unparsed_tree = get_unparsed_boolean_equality_true_false();
        let unparsed = WfData::from_map(unparsed_tree.clone());
        let evaluated = unparsed.evaluate(&context).unwrap();
        assert_eq!(evaluated, WfBoolean::new(false).into_wf_data());
    }
}

use crate::{
    EvalError, EvalErrorKind, ExecutionContext, KeyIndex, RcI,
    data_types::{
        MaybeEvaluated, WfData, WfDataType, types_def::WfTypeGeneric, util::SubstitutionInfo,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct WfTypedListInner {
    /// should always have at least one entry if chain_into is set. poping to the last entry mean switching chain.
    entries: RcI<Vec<WfData>>,
    /// this field point to another entry of itself.
    /// When the end of entries is reached, this next WfTypedListInner is to be used. If it is None, then the end of the list is reached.
    /// Note: you probably should wait for at least 10 entries or so to be in the list before creating a new chain. As a mix between linked list and Vec.
    chain_into: Option<RcI<WfTypedListInner>>,
}

//TODO: study if that is really better than a linked list.

/// The type may be either evaluated and checked to be valid, or unevaluated.
/// When evaluating a WfTypedList whose type is unparsed, it parse it, but does not further check the entries correspond to that type until they are themselve evaluated.
#[derive(Debug, Clone, PartialEq)]
// into two separated Rc cause this way I can change the type without cloning the entries
pub struct WfTypedList {
    inner: RcI<WfTypedListInner>,
    // directly point to the inner data, not a WfTypedListType (unless for a list of list)
    pub inner_type: RcI<MaybeEvaluated<WfTypeGeneric>>,
    /// should never be greater than entries (except if there is no chain_into)
    start_position: usize,
}

impl WfTypedList {
    pub fn new(r#type: MaybeEvaluated<WfTypeGeneric>, entries: Vec<WfData>) -> Self {
        Self {
            inner: RcI::new(WfTypedListInner {
                entries: RcI::new(entries),
                chain_into: None,
            }),
            inner_type: RcI::new(r#type),
            start_position: 0,
        }
    }

    pub fn parse(data: WfData, _context: &ExecutionContext) -> Result<Self, (EvalError, WfData)> {
        match data {
            WfData::WfTypedList(d) => return Ok(d),
            _ => (),
        };
        todo!("parsing list from keys from {:?}", data);
    }

    pub fn len(&self) -> usize {
        let mut size = self.inner.entries.len().saturating_sub(self.start_position);
        let mut next_chain = self.inner.chain_into.as_ref();
        while let Some(chain) = next_chain {
            size += chain.entries.len();
            next_chain = chain.chain_into.as_ref();
        }
        size
    }

    /// the one function that work start_position is out of bound. replace it in-bound if posible, potentially going up to exausting all chains.
    pub fn switch_to_next_entry_group_as_needed(&mut self) {
        while self.inner.entries.get(self.start_position).is_none()
            && let Some(next_inner) = self.inner.chain_into.as_ref()
        {
            let past_entries_len = self.inner.entries.len();
            self.inner = next_inner.clone();
            self.start_position = self.start_position.checked_sub(past_entries_len).unwrap(); // should normally not panic
        }
    }

    /// return true if the is at least one entry remaining
    pub fn is_empty(&self) -> bool {
        self.inner.entries.get(self.start_position).is_none()
    }

    /// First WfData of result is head, second element (Self) is tail.
    /// return an error if the list is empty. May still return an empty list as tail if just one element is present.
    /// check the type of the head, which requires it being evaluated (will only be evaluated if check_type is true)
    /// As such, the context will also only be used if check_type is true
    pub fn split_first_element(
        mut self,
        check_type: bool,
        context: &ExecutionContext,
    ) -> Result<(WfData, Self), (EvalError, Self)> {
        let mut head = if let Some(e) = self.inner.entries.get(self.start_position) {
            e.clone()
        } else {
            return Err((
                EvalError::from_kind(EvalErrorKind::CantGetHeadOfEmptyList),
                self,
            ));
        };

        if check_type {
            head = match head.evaluate(context) {
                Ok(v) => v,
                Err((e, _)) => return Err((e, self)),
            };
            match head.check_type_compatibility(
                match (&*self.inner_type).clone() {
                    MaybeEvaluated::Valid(v) => v.clone(),
                    MaybeEvaluated::Unchecked(_) => todo!(), // I think this should be unreacheable. Should it?
                },
                context,
            ) {
                Ok(()) => (),
                Err(e) => return Err((e, self)),
            };
        }

        self.start_position += 1;
        self.switch_to_next_entry_group_as_needed();

        Ok((head, self))
    }

    /// Does not check type validity
    pub fn iter(&self) -> WfTypedListIterator {
        WfTypedListIterator {
            typed_list: self.clone(),
        }
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
                    Err((e, _)) => return Err((e.inside_key(keyindex!(1, 1)), self)),
                };

                let checked_type = match WfTypeGeneric::parse(type_evaluated, context) {
                    Ok(v) => v,
                    Err((e, _)) => return Err((e.inside_key(keyindex!(1, 1)), self)),
                };

                Ok((Self {
                    inner: self.inner,
                    inner_type: RcI::new(MaybeEvaluated::Valid(checked_type)),
                    start_position: self.start_position,
                })
                .into_wf_data())
            } else {
                unreachable!();
            }
        } else {
            Ok(self.into_wf_data())
        }
    }

    fn substitute_function_arguments<I: SubstitutionInfo>(
        self,
        info: &I,
        context: &ExecutionContext,
    ) -> Result<WfData, EvalError> {
        let mut new_entries = Vec::new();

        for (pos, entry) in self.iter().enumerate() {
            new_entries.push(
                entry
                    .substitute_function_arguments(info, context)
                    .map_err(|e| e.inside_list(pos))?,
            )
        }

        let inner_type = RcI::new(match (&*self.inner_type).clone() {
            MaybeEvaluated::Unchecked(v) => MaybeEvaluated::Unchecked(
                v.substitute_function_arguments(info, context)
                    .map_err(|e| e.inside_key(keyindex!(1, 1)))?,
            ),
            MaybeEvaluated::Valid(v) => MaybeEvaluated::Valid(
                match WfTypeGeneric::parse(
                    v.substitute_function_arguments(info, context)
                        .map_err(|e| e.inside_key(keyindex!(1, 1)))?,
                    context,
                ) {
                    Ok(v) => v,
                    Err((e, _)) => return Err(e.inside_key(keyindex!(1, 1))),
                },
            ),
        });

        Ok((Self {
            inner: RcI::new(WfTypedListInner {
                entries: RcI::new(new_entries),
                chain_into: None,
            }),
            inner_type,
            start_position: 0,
        })
        .into_wf_data())
    }
}

/// This iterator just contain a copy of the list inside (with all the Rc that goes with it)
pub struct WfTypedListIterator {
    typed_list: WfTypedList,
}

impl Iterator for WfTypedListIterator {
    type Item = WfData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.typed_list.is_empty() {
            None
        } else {
            let element = self
                .typed_list
                .inner
                .entries
                .get(self.typed_list.start_position)
                .unwrap()
                .clone();
            self.typed_list.start_position += 1;
            self.typed_list.switch_to_next_entry_group_as_needed();
            Some(element)
        }
    }
}

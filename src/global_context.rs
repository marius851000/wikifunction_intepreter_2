use std::{collections::BTreeMap, io::BufRead};

use anyhow::Context;
use sonic_rs::Object;

use crate::{EvalError, EvalErrorKind, Zid, data_types::WfData, parsing::parse_json};

#[derive(Default)]
pub struct GlobalContext {
    //TODO: I’m not sure wether I wan’t this to request page like an executor or store all data locally. For now, take the second option, it’s simpler. This should be kept in the future, for testing purposes.
    //TODO: persistent objects
    objects: BTreeMap<Zid, WfData>,
}

impl GlobalContext {
    pub fn get_object_value(&self, zid: &Zid) -> Result<WfData, EvalError> {
        self.objects
            .get(zid)
            .ok_or_else(|| EvalError::from_kind(EvalErrorKind::MissingPersistentObject(*zid)))
            .cloned()
    }

    pub fn add_from_json(&mut self, page_title: &str, body: &str) -> Result<(), anyhow::Error> {
        let zid = Zid::from_str(page_title).context("parsing a page title")?;
        let body_value: Object = sonic_rs::from_str(body).context("parsing json of a page")?;

        let data = parse_json::parse_value(
            body_value
                .get(&"Z2K2") // TODO: directly store the persistent object (evaluated?). Or maybe put the persistent object data in a separate map? They won’t be needed often, after all? No. The WfData will be directly accessible and will still need to be cloned, it’s likely just a matter of an pointer arithmetic, which is negligeable.
                .context("trying to get the persistent object’s value")?,
        )
        .context("convert page to IR")?;

        self.objects.insert(zid, data);

        Ok(())
    }

    pub fn from_wikifunction_dump<F: BufRead>(reader: F) -> Result<Self, anyhow::Error> {
        let mut global_context = Self::default();
        for result in parse_mediawiki_dump_reboot::parse(reader) {
            let result = result.unwrap();
            if result.model.unwrap() == "zobject" {
                global_context
                    .add_from_json(&result.title, &result.text)
                    .with_context(|| format!("parsing page {}", result.title))?;
            }
        }
        Ok(global_context)
    }

    #[cfg(test)]
    pub fn default_for_test() -> Self {
        use map_macro::btree_map;

        use crate::data_types::{
            WfBoolean, WfDataType,
            types_def::{WfStandardType, WfStandardTypeInner},
        };

        Self {
            objects: btree_map! {
                zid!(40) => <WfStandardType>::from(WfStandardTypeInner {
                    identity_ref: zid!(40),
                    keys: WfData::unvalid(EvalErrorKind::TestData),
                    validator: WfData::unvalid(EvalErrorKind::TestData),
                    equality: WfData::unvalid(EvalErrorKind::TestData),
                    display_function: WfData::unvalid(EvalErrorKind::TestData),
                    reading_function: WfData::unvalid(EvalErrorKind::TestData),
                    type_converters_to_code: WfData::unvalid(EvalErrorKind::TestData),
                    type_converters_from_code: WfData::unvalid(EvalErrorKind::TestData),
                }).into_wf_data(),
                zid!(41) => WfBoolean::new(true).into_wf_data(),
                zid!(42) => WfBoolean::new(false).into_wf_data()
            },
        }
    }
}

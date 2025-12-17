use std::{fs::File, io::BufReader};

use anyhow::Context;
use interpreter2::{
    ExecutionContext, GlobalContext, RcI, Zid,
    data_types::{WfData, WfDataType},
};

fn main() -> anyhow::Result<()> {
    let file =
        BufReader::new(File::open("./wikifunctionswiki-20251201-pages-meta-current.xml").unwrap());
    let global_context =
        RcI::new(GlobalContext::from_wikifunction_dump(file).context("loading dump")?);

    let execution_context = ExecutionContext::default_for_global(global_context.clone());

    println!(
        "boolean directly from entry: {:?}",
        global_context
            .get_object_value(&Zid::from_u32_panic(41))
            .unwrap()
    );
    println!(
        "boolean from data: {:?}",
        WfData::new_reference(Zid::from_u32_panic(41))
            .evaluate(&execution_context)
            .unwrap()
    );

    Ok(())
}

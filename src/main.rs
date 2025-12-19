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

    let to_get = Zid::from_u32_panic(40);
    println!(
        "{} directly from entry: {:?}",
        to_get,
        global_context.get_object_value(&to_get).unwrap()
    );
    println!(
        "{} evaluated: {:?}",
        to_get,
        WfData::new_reference(to_get)
            .evaluate(&execution_context)
            .unwrap()
    );

    Ok(())
}

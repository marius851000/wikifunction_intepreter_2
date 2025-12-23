use std::{fs::File, io::BufReader};

use anyhow::Context;
use interpreter2::{ExecutionContext, GlobalContext, RcI, Zid, data_types::WfTestCase, zid};

fn run_test_case(zid: Zid, context: &ExecutionContext) {
    let test_case = WfTestCase::parse(
        context.get_global().get_object_value(&zid).unwrap(),
        context,
    )
    .unwrap();
    test_case.run_test(context).unwrap()
}

fn main() -> anyhow::Result<()> {
    let file =
        BufReader::new(File::open("./wikifunctionswiki-20251201-pages-meta-current.xml").unwrap());
    let global_context =
        RcI::new(GlobalContext::from_wikifunction_dump(file).context("loading dump")?);

    let execution_context = ExecutionContext::default_for_global(global_context.clone());

    run_test_case(zid!(10192), &execution_context);

    Ok(())
}

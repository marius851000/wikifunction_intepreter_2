use interpreter2::{ExecutionContext, GlobalContext, RcI, Zid, data_types::WfTestCase};
use std::{collections::BTreeMap, fs::File, io::BufReader};

fn run_test_case(zid: Zid, context: &ExecutionContext) {
    let test_case = WfTestCase::parse(
        context.get_global().get_object_value(&zid).unwrap(),
        context,
    )
    .unwrap();
    test_case.run_test(context).unwrap()
}

fn main() {
    let file =
        BufReader::new(File::open("./wikifunctionswiki-20251201-pages-meta-current.xml").unwrap());
    let global_context = RcI::new(GlobalContext::from_wikifunction_dump(file).unwrap());

    let execution_context = ExecutionContext::default_for_global(global_context.clone());

    let mut test_to_run_file = File::open("./test_case_to_test.json").unwrap();

    let tests: BTreeMap<String, Vec<u32>> = serde_json::from_reader(&mut test_to_run_file).unwrap();

    for (test_category, test_list) in tests {
        println!("running tests for {}", test_category);
        for test_id in test_list.iter() {
            println!("    Test Z{} ...", test_id);
            run_test_case(Zid::from_u32_panic(*test_id), &execution_context);
        }
    }
}

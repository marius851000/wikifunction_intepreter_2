use interpreter2::{
    ExecutionContext, GlobalContext, RcI,
    data_types::{WfDataType, WfTestCase},
    replay,
};
use std::{fs::File, io::BufReader};

fn main() {
    let file =
        BufReader::new(File::open("./wikifunctionswiki-20251201-pages-meta-current.xml").unwrap());
    let global_context = RcI::new(GlobalContext::from_wikifunction_dump(file).unwrap());

    let execution_context = ExecutionContext::default_for_global(global_context.clone());

    for (key, entry) in execution_context.get_global().objects.iter() {
        if key.get_z().get() > 20000 {
            break; // I have some issue with Z10192 where it recursively call the list_equality function, and that’s not the tested behavior. I’ll care about once I have better tracing.
        }
        let test_case = match WfTestCase::parse(entry.clone(), &execution_context) {
            Ok(t) => t,
            Err(_) => continue,
        };

        println!("{}", key);
        match test_case.clone().run_test(&execution_context) {
            Ok(_) => println!("suceeded test case {}", key),
            Err(e) => {
                println!("------------");
                println!("{:?}", e);
                let replay_info =
                    replay::generate_replay(test_case.into_wf_data(), &execution_context, &e);
                println!("{}", replay_info.pretty_trace());
            }
        }
    }
}

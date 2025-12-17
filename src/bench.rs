#[cfg(test)]
mod benches {
    extern crate test;
    use std::{collections::BTreeMap, rc::Rc, sync::Arc};

    use map_macro::btree_map;
    use test::Bencher;

    fn create_test_data<T: Clone, F: Fn() -> T>(f: F) -> Vec<T> {
        let mut source_vec: Vec<T> = Vec::with_capacity(1000);
        for i in 0..5 {
            let contended: T = f();
            for _ in 0..100 {
                source_vec.push(contended.clone());
            }
        }
        for _ in 0..500 {
            source_vec.push(f());
        }
        source_vec
    }

    fn bench_clone<T: Clone, F: Fn() -> T>(f: F, b: &mut Bencher) {
        let source_vec = create_test_data(f);
        b.iter(|| {
            let mut dest_vec = Vec::with_capacity(1000);
            let source_vec = test::black_box(&source_vec);
            for source in source_vec {
                dest_vec.push(source.clone());
            }
        });
    }

    #[bench]
    fn clone_1000_rc(b: &mut Bencher) {
        bench_clone(|| Rc::new("hello".to_string()), b)
    }

    #[bench]
    fn clone_1000_string(b: &mut Bencher) {
        bench_clone(|| "hello".to_string(), b)
    }

    #[bench]
    fn clone_1000_btreemap(b: &mut Bencher) {
        bench_clone(
            || {
                btree_map! {
                    zid!(1, 1) => true
                }
            },
            b,
        );
    }

    #[bench]
    fn clone_1000_arc(b: &mut Bencher) {
        bench_clone(|| Arc::new("hello".to_string()), b);
    }

    #[bench]
    fn clone_1000_boolean(b: &mut Bencher) {
        bench_clone(|| true, b);
    }

    #[bench]
    fn create_1000_rc(b: &mut Bencher) {
        b.iter(|| {
            let mut alloc_vec = Vec::with_capacity(1000);
            for _ in 0..1000 {
                alloc_vec.push(Rc::new(false));
            }
        });
    }

    #[bench]
    fn create_1000_arc(b: &mut Bencher) {
        b.iter(|| {
            let mut alloc_vec = Vec::with_capacity(1000);
            for _ in 0..1000 {
                alloc_vec.push(Arc::new(false));
            }
        });
    }
}

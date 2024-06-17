use std::collections::{HashMap, HashSet};

use hydroflow::util::collect_ready;
use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use lattices::ght::GeneralizedHashTrie;
use lattices::ght_lattice::{DeepJoinLatticeBimorphism, GhtBimorphism};
use lattices::map_union::{KeyedBimorphism, MapUnionHashMap, MapUnionSingletonMap};
use lattices::set_union::{CartesianProductBimorphism, SetUnionHashSet, SetUnionSingletonSet};
use lattices::{GhtNodeType, GhtType};
use multiplatform_test::multiplatform_test;
use variadics::{var_expr, UnrefCloneVariadic};

#[multiplatform_test]
pub fn test_cartesian_product() {
    let (out_send, out_recv) = hydroflow::util::unbounded_channel::<_>();

    let mut df = hydroflow_syntax! {
        lhs = source_iter_delta(0..3)
            -> map(SetUnionSingletonSet::new_from)
            -> state::<'static, SetUnionHashSet<usize>>();
        rhs = source_iter_delta(3..5)
            -> map(SetUnionSingletonSet::new_from)
            -> state::<'static, SetUnionHashSet<usize>>();

        lhs[items] -> [0]my_join;
        rhs[items] -> [1]my_join;

        my_join = lattice_bimorphism(CartesianProductBimorphism::<HashSet<_>>::default(), #lhs, #rhs)
            -> lattice_reduce()
            -> for_each(|x| out_send.send(x).unwrap());
    };

    assert_graphvis_snapshots!(df);
    df.run_available();

    assert_eq!(
        &[SetUnionHashSet::new(HashSet::from_iter([
            (0, 3),
            (0, 4),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
        ]))],
        &*collect_ready::<Vec<_>, _>(out_recv)
    );
}

#[multiplatform_test]
pub fn test_join() {
    let (out_send, out_recv) = hydroflow::util::unbounded_channel::<_>();

    let mut df = hydroflow_syntax! {
        lhs = source_iter_delta([(7, 1), (7, 2)])
            -> map(|(k, v)| MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v))))
            -> state::<'static, MapUnionHashMap<usize, SetUnionHashSet<usize>>>();
        rhs = source_iter_delta([(7, 0), (7, 1), (7, 2)])
            -> map(|(k, v)| MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v))))
            -> state::<'static, MapUnionHashMap<usize, SetUnionHashSet<usize>>>();

        lhs[items] -> [0]my_join;
        rhs[items] -> [1]my_join;

        my_join = lattice_bimorphism(KeyedBimorphism::<HashMap<_, _>, _>::new(CartesianProductBimorphism::<HashSet<_>>::default()), #lhs, #rhs)
            -> lattice_reduce()
            -> for_each(|x| out_send.send(x).unwrap());
    };

    assert_graphvis_snapshots!(df);
    df.run_available();

    assert_eq!(
        &[MapUnionHashMap::new(HashMap::from_iter([(
            7,
            SetUnionHashSet::new(HashSet::from_iter([
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]))
        )]))],
        &*collect_ready::<Vec<_>, _>(out_recv)
    );
}

#[test]
fn test_ght_join_bimorphism() {
    type MyGhtA = GhtType!(u32, u64, u16 => &'static str);
    type MyGhtB = GhtType!(u32, u64, u16 => &'static str);
    type MyGhtATrie = <MyGhtA as GeneralizedHashTrie>::Trie;
    type MyGhtBTrie = <MyGhtB as GeneralizedHashTrie>::Trie;
    type ResultGht = GhtType!(u32, u64, u16 => &'static str, &'static str);

    type MyNodeBim =
        <(MyGhtATrie, MyGhtBTrie) as DeepJoinLatticeBimorphism>::DeepJoinLatticeBimorphism;
    type MyBim = GhtBimorphism<MyNodeBim>;

    let (out_send, out_recv) = hydroflow::util::unbounded_channel::<_>();

    let mut hf = hydroflow_syntax! {
        lhs = source_iter_delta([
                var_expr!(123, 2, 5, "hello"),
                var_expr!(50, 1, 1, "hi"),
                var_expr!(5, 1, 7, "hi"),
                var_expr!(5, 1, 7, "bye"),
            ])
            -> map(|row| MyGhtA::new_from([row]))
            -> state::<'tick, MyGhtA>();
        rhs = source_iter_delta([
                var_expr!(5, 1, 8, "hi"),
                var_expr!(5, 1, 7, "world"),
                var_expr!(5, 1, 7, "folks"),
                var_expr!(10, 1, 2, "hi"),
                var_expr!(12, 10, 98, "bye"),
            ])
            -> map(|row| MyGhtB::new_from([row]))
            -> state::<'tick, MyGhtB>();

        lhs[items] -> [0]my_join;
        rhs[items] -> [1]my_join;

        my_join = lattice_bimorphism(MyBim::default(), #lhs, #rhs)
            -> lattice_reduce()
            -> for_each(|x| out_send.send(x).unwrap());
    };
    // hf.meta_graph().unwrap().open_mermaid(&Default::default());
    hf.run_available();
    assert_eq!(
        &[ResultGht::new_from(vec![
            var_expr!(5, 1, 7, "hi", "world"),
            var_expr!(5, 1, 7, "bye", "world"),
            var_expr!(5, 1, 7, "hi", "folks"),
            var_expr!(5, 1, 7, "bye", "folks"),
        ])],
        &*collect_ready::<Vec<_>, _>(out_recv)
    );
}

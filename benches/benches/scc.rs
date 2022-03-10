use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Cursor};
use std::time::Duration;

use criterion::Criterion;
use graph_mesh::{GraphMesh, GraphMeshWriter};
use once_cell::sync::OnceCell;
use rand::Rng;

fn graph_edges() -> &'static [(usize, usize)] {
    static GRAPH_DATA: OnceCell<Vec<(usize, usize)>> = OnceCell::new();
    &*GRAPH_DATA.get_or_init(|| {
        let cursor = Cursor::new(include_bytes!("scc_edges.txt"));
        let reader = BufReader::new(cursor);
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut nums = line.split_whitespace();
                let a = nums.next().unwrap().parse().unwrap();
                let b = nums.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect()
    })
}
fn edges_to_graphmesh(edges: &[(usize, usize)]) -> GraphMeshWriter {
    let mut graph = GraphMeshWriter::new();
    for &(a, b) in edges {
        graph.insert_edge(a, b);
    }
    graph
}

fn scc_labels() -> &'static [usize] {
    static SCC_LABELS: OnceCell<Vec<usize>> = OnceCell::new();
    &*SCC_LABELS.get_or_init(|| {
        let cursor = Cursor::new(include_bytes!("scc_labels.txt"));
        let reader = BufReader::new(cursor);

        reader
            .lines()
            .map(|line| {
                let label = line.unwrap().parse().unwrap();
                label
            })
            .collect()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DfsOrder {
    Forward,
    Back,
}
pub fn dfs(
    graph: &GraphMesh,
    order: DfsOrder,
    vertex: usize,
    preorder_visit: &mut impl FnMut(usize) -> bool,
    postorder_visit: &mut impl FnMut(usize),
) {
    if (preorder_visit)(vertex) {
        let nexts = match order {
            DfsOrder::Forward => graph.succs(vertex),
            DfsOrder::Back => graph.preds(vertex),
        };
        for next in nexts {
            dfs(graph, order, next, preorder_visit, postorder_visit)
        }
        (postorder_visit)(vertex);
    }
}

pub fn naive_n3<'a>(graph: &'a GraphMesh) -> impl 'a + Iterator<Item = (usize, usize)> {
    graph.vertices().map(|v| {
        let mut visited_forw = HashSet::new();
        let mut visited_back = HashSet::new();
        let mut label = v;

        dfs(
            graph,
            DfsOrder::Forward,
            v,
            &mut |v| visited_forw.insert(v),
            &mut |_| {},
        );
        dfs(
            graph,
            DfsOrder::Back,
            v,
            &mut |v| {
                if visited_back.insert(v) {
                    if visited_forw.contains(&v) {
                        label = std::cmp::max(label, v);
                    }
                    true
                } else {
                    false
                }
            },
            &mut |_| {},
        );

        (v, label)
    })
}

pub fn kosaraju<'a>(graph: &'a GraphMesh) -> impl 'a + Iterator<Item = (usize, usize)> {
    let mut seen = HashSet::new();
    let mut order = Vec::with_capacity(graph.vertices().len());
    for v in graph.vertices() {
        dfs(
            graph,
            DfsOrder::Forward,
            v,
            &mut |v| seen.insert(v),
            &mut |x| order.push(x),
        );
    }
    seen.clear();

    let mut roots = HashMap::new();
    let mut root_label = HashMap::new();
    for v in order.into_iter().rev() {
        let mut label = v;
        dfs(
            graph,
            DfsOrder::Back,
            v,
            &mut |v| seen.insert(v),
            &mut |x| {
                roots.insert(x, v);
                label = std::cmp::max(label, x);
            },
        );
        root_label.insert(v, label);
    }

    graph.vertices().map(move |v| {
        let root = roots[&v];
        let label = root_label[&root];
        (v, label)
    })
}

// pub fn fleischer_subroutine(
//     graph: &GraphMesh,
// ) -> (
//     (usize, Vec<usize>),
//     HashSet<usize>,
//     HashSet<usize>,
//     HashSet<usize>,
// ) {
//     let subgraph_vertices: Vec<_> = graph.vertices().collect();
//     assert!(!subgraph_vertices.is_empty());
//     let pivot = subgraph_vertices[rand::thread_rng().gen_range(0..subgraph_vertices.len())];

//     let mut subgraph_forw = HashSet::new();
//     dfs(forw, pivot, &mut |v| subgraph_forw.insert(v), &mut |_| {});

//     let mut scc_max = pivot;
//     let mut subgraph_scc = Vec::new();

//     let mut subgraph_back = HashSet::new();

//     let mut visited_back = HashSet::new();
//     dfs(
//         back,
//         pivot,
//         &mut |v| {
//             if visited_back.insert(v) {
//                 if subgraph_forw.take(&v).is_some() {
//                     scc_max = std::cmp::max(scc_max, v);
//                     subgraph_scc.push(v);
//                 } else {
//                     subgraph_back.insert(v);
//                 }
//                 true
//             } else {
//                 false
//             }
//         },
//         &mut |_| {},
//     );

//     let subgraph_rest: HashSet<usize> = subgraph_vertices
//         .into_iter()
//         .filter(|v| {
//             !subgraph_forw.contains(v) && !subgraph_back.contains(v) && !subgraph_scc.contains(v)
//         })
//         .collect();
//     let subgraph_forw = subgraph_forw;

//     (
//         (scc_max, subgraph_scc),
//         subgraph_rest,
//         subgraph_forw,
//         subgraph_back,
//     )
// }

// pub fn filter_subgraph(verts: &HashSet<usize>, adj_list: &AdjList) -> AdjList {
//     adj_list
//         .iter()
//         .filter(|&(k, _v)| verts.contains(k))
//         .map(|(&k, val)| {
//             (
//                 k,
//                 val.iter().filter(|&v| verts.contains(v)).copied().collect(),
//             )
//         })
//         .collect()
// }

// pub fn fleischer_single(
//     size: usize,
//     forw: &HashMap<usize, Vec<usize>>,
//     back: &HashMap<usize, Vec<usize>>,
// ) -> Vec<usize> {
//     let mut output: Vec<usize> = (0..size).collect();
//     let mut subgraph_stack = vec![(0..size).collect::<HashSet<_>>()];
//     while let Some(subgraph_vertices) = subgraph_stack.pop() {
//         if subgraph_vertices.is_empty() {
//             continue;
//         }
//         let curr_forw = filter_subgraph(&subgraph_vertices, forw);
//         let curr_back = filter_subgraph(&subgraph_vertices, back);

//         let ((scc_max, scc_members), subgraph_othr, subgraph_forw, subgraph_back) =
//             fleischer_subroutine(&curr_forw, &curr_back);
//         for v in scc_members {
//             output[v] = scc_max;
//         }
//         subgraph_stack.extend([subgraph_forw, subgraph_back, subgraph_othr]);
//     }
//     output
// }

// pub fn fleischer_multi(
//     size: usize,
//     forw: &HashMap<usize, Vec<usize>>,
//     back: &HashMap<usize, Vec<usize>>,
//     num_threads: usize,
// ) -> Vec<usize> {
//     use rayon::{Scope, ThreadPoolBuilder};
//     use std::sync::atomic::{AtomicUsize, Ordering};
//     // use std::sync::mpsc::{self, Sender};

//     fn fleischer_rayon<'a>(
//         scope: &Scope<'a>,
//         subgraph_vertices: HashSet<usize>,
//         forw: &'a HashMap<usize, Vec<usize>>,
//         back: &'a HashMap<usize, Vec<usize>>,
//         output: &'a [AtomicUsize],
//     ) {
//         if subgraph_vertices.is_empty() {
//             return;
//         }

//         let curr_forw = filter_subgraph(&subgraph_vertices, forw);
//         let curr_back = filter_subgraph(&subgraph_vertices, back);

//         let ((scc_max, scc_members), subgraph_othr, subgraph_forw, subgraph_back) =
//             fleischer_subroutine(&curr_forw, &curr_back);
//         for v in scc_members {
//             output[v].store(scc_max, Ordering::Relaxed);
//         }

//         let (output_a, output_b, output_c) = (output.clone(), output.clone(), output);
//         scope.spawn(|scope| fleischer_rayon(scope, subgraph_othr, forw, back, output_a));
//         scope.spawn(|scope| fleischer_rayon(scope, subgraph_forw, forw, back, output_b));
//         scope.spawn(|scope| fleischer_rayon(scope, subgraph_back, forw, back, output_c));
//     }

//     let threadpool = ThreadPoolBuilder::new()
//         .num_threads(num_threads)
//         .build()
//         .expect("Failed to build threadpool.");

//     let mut output: Vec<AtomicUsize> = Vec::with_capacity(size);
//     output.resize_with(size, Default::default);

//     threadpool.scope(|scope| {
//         let output_slice = &*output;
//         scope.spawn(move |scope| {
//             fleischer_rayon(scope, (0..size).collect(), forw, back, output_slice);
//         });
//     });
//     output.iter().map(|x| x.load(Ordering::Relaxed)).collect()
// }

pub fn crit_naive_n3(c: &mut Criterion) {
    let edges = graph_edges();
    let expected = scc_labels();

    {
        let graph = edges_to_graphmesh(edges);
        let graph = graph.finish();
        for v in graph.vertices() {
            println!("{}", v);
        }
    }

    c.bench_function("scc/naive_n3", |b| {
        b.iter(|| {
            let graph = edges_to_graphmesh(edges);
            let graph = graph.finish();

            let labels = naive_n3(&graph);
            let labels: Vec<_> = labels.collect::<BinaryHeap<_>>().into_sorted_vec();

            labels
                .iter()
                .zip(expected.iter())
                .for_each(|((v, label), expected)| {
                    assert_eq!(expected, label, "Vertex {} wrong", v)
                });
        });
    });
}

pub fn crit_kosaraju(c: &mut Criterion) {
    let edges = graph_edges();
    let expected = scc_labels();

    c.bench_function("scc/kosaraju", |b| {
        b.iter(|| {
            let graph = edges_to_graphmesh(edges);
            let graph = graph.finish();

            let labels = kosaraju(&graph);
            let labels: Vec<_> = labels.collect::<BinaryHeap<_>>().into_sorted_vec();

            labels
                .iter()
                .zip(expected.iter())
                .for_each(|((v, label), expected)| {
                    assert_eq!(expected, label, "Vertex {} wrong", v)
                });
        });
    });
}

// pub fn crit_fleischer_single(c: &mut Criterion) {
//     let edges = graph_edges();
//     let expected = scc_labels();

//     c.bench_function("scc/fleischer_single", |b| {
//         b.iter(|| {
//             let graph = edges_to_graphmesh(edges);
//             let graph = graph.finish();

//             let labels = fleischer_single(&graph);
//             expected
//                 .iter()
//                 .zip(labels.iter())
//                 .enumerate()
//                 .for_each(|(i, (a, b))| assert_eq!(a, b, "Fail on vertex {}", i));
//             assert_eq!(expected, &*labels);
//         });
//     });
// }

// pub fn crit_fleischer_8(c: &mut Criterion) {
//     let edges = graph_edges();
//     let expected = scc_labels();

//     // println!("{:?}", std::thread::available_parallelism());

//     c.bench_function("scc/fleischer_8", |b| {
//         b.iter(|| {
//             let graph = edges_to_graphmesh(edges);
//             let graph = graph.finish();

//             let labels = fleischer_multi(&graph, 8);
//             assert_eq!(expected, &*labels);
//         });
//     });
// }

criterion::criterion_group!(
    name = scc;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(10));
    targets =
        crit_naive_n3, // TOO SLOW
        crit_kosaraju,
        // crit_fleischer_8,
        // crit_fleischer_single,
);
#[cfg(not(feature = "scc_graphgen"))]
criterion::criterion_main!(scc);

#[cfg(feature = "scc_graphgen")]
fn main() {
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    use std::io::Write;

    let size = 200_000;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(892348932490);

    let mut rename: Vec<_> = (0..size).collect();
    rename.shuffle(&mut rng);

    let mut forw = AdjList::new();
    let mut back = AdjList::new();
    {
        let mut out =
            std::io::BufWriter::new(std::fs::File::create("benches/scc_edges.txt").unwrap());
        for a in 0_usize..size {
            for b in (a.saturating_sub(20))..(std::cmp::min(size, a + 20)) {
                if a == b {
                    continue;
                }
                let d = std::cmp::max(a, b) - std::cmp::min(a, b);
                let p = f64::max(
                    20.0 / ((size as f64) * (d as f64)),
                    1.0 / (1.0 + ((d as f64) - 1.8).exp()),
                );

                let a = rename[a];
                let b = rename[b];
                if rng.gen_bool(p) {
                    writeln!(out, "{} {}", a, b).unwrap();
                    forw.entry(a).or_default().push(b);
                    back.entry(b).or_default().push(a);
                }
            }
        }
        out.flush().unwrap();
    }
    println!("Done writing graph.");
    {
        let mut out =
            std::io::BufWriter::new(std::fs::File::create("benches/scc_labels.txt").unwrap());
        let labels = kosaraju(size, &forw, &back);

        let mut label_counts = HashMap::<_, usize>::new();
        for label in labels {
            writeln!(out, "{}", label).unwrap();
            *label_counts.entry(label).or_default() += 1;
        }
        let num_labels = label_counts.len();

        let mut distribution = BTreeMap::<_, usize>::new();
        for (_, count) in label_counts {
            *distribution.entry(count).or_default() += 1;
        }

        println!("{}\n{:#?}", num_labels, distribution);
    }
}

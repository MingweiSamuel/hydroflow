use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Cursor};
use std::time::Duration;

use criterion::Criterion;
use hydroflow::scheduled::input::Give;
use once_cell::sync::OnceCell;
use rand::Rng;

type AdjList = HashMap<usize, Vec<usize>>;
type GraphData = (usize, AdjList, AdjList);
fn graph_data() -> &'static GraphData {
    static GRAPH_DATA: OnceCell<GraphData> = OnceCell::new();
    GRAPH_DATA.get_or_init(|| {
        let cursor = Cursor::new(include_bytes!("scc_edges.txt"));
        let reader = BufReader::new(cursor);

        let mut n = 0;
        let mut forw = AdjList::new();
        let mut back = AdjList::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let mut nums = line.split_whitespace();
            let a = nums.next().unwrap().parse().unwrap();
            let b = nums.next().unwrap().parse().unwrap();
            assert!(nums.next().is_none());
            forw.entry(a).or_default().push(b);
            back.entry(b).or_default().push(a);
            n = std::cmp::max(n, std::cmp::max(a, b));
        }
        (n + 1, forw, back)
    })
}

fn scc_labels() -> &'static [(usize, usize)] {
    static SCC_LABELS: OnceCell<Vec<(usize, usize)>> = OnceCell::new();
    &*SCC_LABELS.get_or_init(|| {
        let cursor = Cursor::new(include_bytes!("scc_labels.txt"));
        let reader = BufReader::new(cursor);

        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut nums = line.split_whitespace();
                let a = nums.next().unwrap().parse().unwrap();
                let b = nums.next().unwrap().parse().unwrap();
                assert!(nums.next().is_none());
                (a, b)
            })
            .collect()
    })
}

pub fn dfs(
    adj_list: &HashMap<usize, Vec<usize>>,
    vertex: usize,
    preorder_visit: &mut impl FnMut(usize) -> bool,
    postorder_visit: &mut impl FnMut(usize),
) {
    if (preorder_visit)(vertex) {
        if let Some(nexts) = adj_list.get(&vertex) {
            nexts
                .iter()
                .for_each(|&next| dfs(adj_list, next, preorder_visit, postorder_visit));
        }
        (postorder_visit)(vertex);
    }
}

pub fn naive_n3(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<(usize, usize)> {
    let labels: Vec<_> = (0..size)
        .map(|v| {
            let mut visited_forw = HashSet::new();
            let mut visited_back = HashSet::new();
            let mut label = v;

            dfs(forw, v, &mut |v| visited_forw.insert(v), &mut |_| {});
            dfs(
                back,
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
        .collect();
    labels
}

pub fn kosaraju(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut seen = HashSet::new();
    let mut order = Vec::with_capacity(size);
    for v in 0..size {
        dfs(forw, v, &mut |v| seen.insert(v), &mut |x| order.push(x));
    }
    seen.clear();

    let mut roots = HashMap::new();
    let mut root_label = HashMap::new();
    for v in order.into_iter().rev() {
        let mut label = v;
        dfs(back, v, &mut |v| seen.insert(v), &mut |x| {
            roots.insert(x, v);
            label = std::cmp::max(label, x);
        });
        root_label.insert(v, label);
    }

    (0..size)
        .map(|v| {
            let root = roots[&v];
            let label = root_label[&root];
            (v, label)
        })
        .collect()
}

pub fn fleischer_subroutine(
    subgraph_vertices: Vec<usize>,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
    scc_found: &mut impl FnMut(usize, Vec<usize>),
) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    assert!(!subgraph_vertices.is_empty());
    let pivot = subgraph_vertices[rand::thread_rng().gen_range(0..subgraph_vertices.len())];

    let mut visited_forw = BTreeSet::new();
    dfs(
        forw,
        pivot,
        &mut |v| subgraph_vertices.binary_search(&v).is_ok() && visited_forw.insert(v),
        &mut |_| {},
    );

    let mut scc_max = 0;
    let mut scc_members = Vec::new();

    let mut subgraph_back = BinaryHeap::new();

    let mut visited_back = HashSet::new();
    dfs(
        back,
        pivot,
        &mut |v| {
            if subgraph_vertices.binary_search(&v).is_ok() && visited_back.insert(v) {
                if visited_forw.contains(&v) {
                    scc_max = std::cmp::max(scc_max, v);
                    scc_members.push(v);
                } else {
                    subgraph_back.push(v);
                }
                true
            } else {
                false
            }
        },
        &mut |_| {},
    );
    (scc_found)(scc_max, scc_members);

    let mut subgraph_othr = subgraph_vertices;
    subgraph_othr.retain(|v| !visited_forw.contains(v) && !visited_back.contains(v));
    let subgraph_forw: Vec<_> = visited_forw
        .into_iter()
        .filter(|v| !visited_back.contains(v))
        .collect();
    let subgraph_back = subgraph_back.into_sorted_vec();

    (subgraph_othr, subgraph_forw, subgraph_back)
}

pub fn fleischer_single(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<(usize, usize)> {
    fn fleischer_recursive(
        subgraph_vertices: Vec<usize>,
        forw: &HashMap<usize, Vec<usize>>,
        back: &HashMap<usize, Vec<usize>>,
        output: &mut BTreeMap<usize, usize>,
    ) {
        if subgraph_vertices.is_empty() {
            return;
        }
        let (subgraph_othr, subgraph_forw, subgraph_back) = fleischer_subroutine(
            subgraph_vertices,
            forw,
            back,
            &mut |scc_max, scc_members| {
                for v in scc_members {
                    output.insert(v, scc_max);
                }
            },
        );
        fleischer_recursive(subgraph_othr, forw, back, output);
        fleischer_recursive(subgraph_forw, forw, back, output);
        fleischer_recursive(subgraph_back, forw, back, output);
    }

    let mut output = Default::default();
    fleischer_recursive((0..size).collect(), forw, back, &mut output);
    output.into_iter().collect()
}

pub fn fleischer_multi(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
    num_threads: usize,
) -> Vec<(usize, usize)> {
    use rayon::{Scope, ThreadPoolBuilder};
    use std::sync::mpsc::{self, SyncSender};

    fn fleischer_rayon<'a>(
        scope: &Scope<'a>,
        subgraph_vertices: Vec<usize>,
        forw: &'a HashMap<usize, Vec<usize>>,
        back: &'a HashMap<usize, Vec<usize>>,
        output: &'a SyncSender<(usize, Vec<usize>)>,
    ) {
        if subgraph_vertices.is_empty() {
            return;
        }
        let (subgraph_othr, subgraph_forw, subgraph_back) = fleischer_subroutine(
            subgraph_vertices,
            forw,
            back,
            &mut |scc_max, scc_members| {
                output
                    .try_send((scc_max, scc_members))
                    .expect("Backpressure!")
            },
        );
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_othr, forw, back, output));
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_forw, forw, back, output));
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_back, forw, back, output));
    }

    let threadpool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to build threadpool.");

    let (output_sender, output_receiver) = mpsc::sync_channel(800);
    {
        threadpool.scope(move |scope| {
            let output_sender = output_sender;
            fleischer_rayon(scope, (0..size).collect(), forw, back, &output_sender);
        });
    }

    vec![]
}

pub fn crit_naive_n3(c: &mut Criterion) {
    let &(size, ref forw, ref back) = graph_data();
    let expected = scc_labels();

    c.bench_function("scc/naive_n3", |b| {
        b.iter(|| {
            let labels = naive_n3(size, forw, back);
            assert_eq!(expected, &*labels);
        });
    });
}

pub fn crit_kosaraju(c: &mut Criterion) {
    let &(size, ref forw, ref back) = graph_data();
    let expected = scc_labels();

    c.bench_function("scc/kosaraju", |b| {
        b.iter(|| {
            let labels = kosaraju(size, forw, back);
            let labels: Vec<_> = labels.into_iter().collect();
            expected
                .iter()
                .zip(labels.iter())
                .for_each(|(a, b)| assert_eq!(a, b));
            assert_eq!(expected, &*labels);
        });
    });
}

pub fn crit_fleischer_single(c: &mut Criterion) {
    let &(size, ref forw, ref back) = graph_data();
    let expected = scc_labels();

    c.bench_function("scc/fleischer_single", |b| {
        b.iter(|| {
            let labels = fleischer_single(size, forw, back);
            assert_eq!(expected, &*labels);
        });
    });
}

pub fn crit_fleischer_4(c: &mut Criterion) {
    let &(size, ref forw, ref back) = graph_data();
    let expected = scc_labels();

    c.bench_function("scc/fleischer_4", |b| {
        b.iter(|| {
            let labels = fleischer_multi(size, forw, back, 4);
            assert_eq!(expected, &*labels);
        });
    });
}

criterion::criterion_group!(
    name = scc;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(10));
    targets =
        crit_naive_n3,
        crit_kosaraju,
        crit_fleischer_single,
        crit_fleischer_4,
);
#[cfg(not(feature = "scc_graphgen"))]
criterion::criterion_main!(scc);

#[cfg(feature = "scc_graphgen")]
fn main() {
    use rand::{Rng, SeedableRng};
    use std::io::Write;

    let size = 2_000;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(892348932490);

    let mut forw = AdjList::new();
    let mut back = AdjList::new();
    {
        let mut out =
            std::io::BufWriter::new(std::fs::File::create("benches/scc_edges.txt").unwrap());
        for a in 0..size {
            for b in 0..size {
                if a == b {
                    continue;
                }
                let d = std::cmp::max(a, b) - std::cmp::min(a, b);
                let p = f64::max(0.02 / (d as f64), 1.0 / (1.0 + ((d as f64) - 1.0).exp()));
                if rng.gen_bool(p) {
                    writeln!(out, "{} {}", a, b).unwrap();
                    forw.entry(a).or_default().push(b);
                    back.entry(b).or_default().push(a);
                }
            }
        }
    }
    {
        let mut out =
            std::io::BufWriter::new(std::fs::File::create("benches/scc_labels.txt").unwrap());
        let labels = naive_n3(size, &forw, &back);
        for &(v, label) in &labels {
            writeln!(out, "{} {}", v, label).unwrap();
        }

        let label_set: HashSet<_> = labels.iter().map(|(_v, label)| label).collect();
        println!("Number of SCCs: {}", label_set.len());
    }
}

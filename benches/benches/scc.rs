use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Cursor};
use std::time::Duration;

use criterion::Criterion;
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
) -> Vec<usize> {
    (0..size)
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

            label
        })
        .collect()
}

pub fn kosaraju(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
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
            label
        })
        .collect()
}

pub fn fleischer_subroutine(
    forw: &AdjList,
    back: &AdjList,
) -> (
    (usize, Vec<usize>),
    HashSet<usize>,
    HashSet<usize>,
    HashSet<usize>,
) {
    let subgraph_vertices: Vec<_> = forw
        .keys()
        .filter(|&v| !back.contains_key(v))
        .chain(back.keys())
        .cloned()
        .collect();
    assert!(!subgraph_vertices.is_empty());
    let pivot = subgraph_vertices[rand::thread_rng().gen_range(0..subgraph_vertices.len())];

    let mut visited_forw = HashSet::new();
    dfs(forw, pivot, &mut |v| visited_forw.insert(v), &mut |_| {});

    let mut scc_max = 0;
    let mut scc_members = Vec::new();

    let mut subgraph_back = HashSet::new();

    let mut visited_back = HashSet::new();
    dfs(
        back,
        pivot,
        &mut |v| {
            if visited_back.insert(v) {
                if visited_forw.take(&v).is_some() {
                    scc_max = std::cmp::max(scc_max, v);
                    scc_members.push(v);
                } else {
                    subgraph_back.insert(v);
                }
                true
            } else {
                false
            }
        },
        &mut |_| {},
    );

    println!("A {}", subgraph_vertices.len());

    let subgraph_othr: HashSet<usize> = subgraph_vertices
        .into_iter()
        .filter(|v| !visited_forw.contains(v) && !visited_back.contains(v))
        .collect();
    let subgraph_forw = visited_forw;

    println!(
        "B {} {} {} {}",
        scc_members.len(),
        subgraph_othr.len(),
        subgraph_forw.len(),
        subgraph_back.len()
    );

    (
        (scc_max, scc_members),
        subgraph_othr,
        subgraph_forw,
        subgraph_back,
    )
}

pub fn filter_subgraph(verts: &HashSet<usize>, adj_list: &AdjList) -> AdjList {
    adj_list
        .iter()
        .filter(|&(k, _v)| verts.contains(k))
        .map(|(&k, val)| {
            (
                k,
                val.iter().filter(|&v| verts.contains(v)).copied().collect(),
            )
        })
        .collect()
}

pub fn fleischer_single(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    let mut output = vec![0; size];
    let mut subgraph_stack = vec![(0..size).collect::<HashSet<_>>()];
    while let Some(subgraph_vertices) = subgraph_stack.pop() {
        if subgraph_vertices.is_empty() {
            continue;
        }
        let curr_forw = filter_subgraph(&subgraph_vertices, forw);
        let curr_back = filter_subgraph(&subgraph_vertices, back);

        let ((scc_max, scc_members), subgraph_othr, subgraph_forw, subgraph_back) =
            fleischer_subroutine(&curr_forw, &curr_back);
        for v in scc_members {
            output[v] = scc_max;
        }
        subgraph_stack.extend([subgraph_forw, subgraph_back, subgraph_othr]);
    }
    output
}

pub fn fleischer_multi(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
    num_threads: usize,
) -> Vec<usize> {
    use rayon::{Scope, ThreadPoolBuilder};
    use std::sync::atomic::{AtomicUsize, Ordering};
    // use std::sync::mpsc::{self, Sender};

    fn fleischer_rayon<'a>(
        scope: &Scope<'a>,
        subgraph_vertices: HashSet<usize>,
        forw: &'a HashMap<usize, Vec<usize>>,
        back: &'a HashMap<usize, Vec<usize>>,
        output: &'a [AtomicUsize],
    ) {
        if subgraph_vertices.is_empty() {
            return;
        }

        let curr_forw = filter_subgraph(&subgraph_vertices, forw);
        let curr_back = filter_subgraph(&subgraph_vertices, back);

        let ((scc_max, scc_members), subgraph_othr, subgraph_forw, subgraph_back) =
            fleischer_subroutine(&curr_forw, &curr_back);
        for v in scc_members {
            output[v].store(scc_max, Ordering::Relaxed);
        }

        let (output_a, output_b, output_c) = (output.clone(), output.clone(), output);
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_othr, forw, back, output_a));
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_forw, forw, back, output_b));
        scope.spawn(|scope| fleischer_rayon(scope, subgraph_back, forw, back, output_c));
    }

    let threadpool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to build threadpool.");

    let mut output: Vec<AtomicUsize> = Vec::with_capacity(size);
    output.resize_with(size, Default::default);

    threadpool.scope(|scope| {
        let output_slice = &*output;
        scope.spawn(move |scope| {
            fleischer_rayon(scope, (0..size).collect(), forw, back, output_slice);
        });
    });
    output.iter().map(|x| x.load(Ordering::Relaxed)).collect()
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

pub fn crit_fleischer_8(c: &mut Criterion) {
    let &(size, ref forw, ref back) = graph_data();
    let expected = scc_labels();

    // println!("{:?}", std::thread::available_parallelism());

    c.bench_function("scc/fleischer_8", |b| {
        b.iter(|| {
            let labels = fleischer_multi(size, forw, back, 8);
            assert_eq!(expected, &*labels);
        });
    });
}

criterion::criterion_group!(
    name = scc;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(10));
    targets =
        // crit_kosaraju,
        // crit_fleischer_8,
        crit_fleischer_single,
        // crit_naive_n3, // TOO SLOW
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

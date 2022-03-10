use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Cursor};
use std::time::Duration;

use criterion::Criterion;
use graph_mesh::{GraphMesh, GraphMeshWriter};
use once_cell::sync::OnceCell;

fn graph_edges() -> &'static [(usize, usize)] {
    static GRAPH_DATA: OnceCell<Vec<(usize, usize)>> = OnceCell::new();
    GRAPH_DATA.get_or_init(|| {
        let cursor = Cursor::new(include_bytes!("scc_edges.txt"));
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

fn edges_to_graph(edges: impl IntoIterator<Item = (usize, usize)>) -> GraphMeshWriter {
    let mut graph = GraphMeshWriter::new();
    edges.into_iter().for_each(|(a, b)| graph.insert_edge(a, b));
    graph
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
    graph: &GraphMesh<'_>,
    forw: bool,
    from: usize,
    seen: &mut HashSet<usize>,
    mut visit: impl FnMut(usize),
) {
    let mut stack = vec![from];
    while let Some(v) = stack.pop() {
        if !seen.contains(&v) {
            (visit)(v);
            seen.insert(v);
            let nexts = if forw { graph.succs(v) } else { graph.preds(v) };
            stack.extend(nexts.iter().filter(|&next| !seen.contains(next)));
        }
    }
}

pub fn naive_n3(graph: &GraphMesh<'_>) -> Vec<(usize, usize)> {
    let labels: Vec<_> = graph
        .vertices()
        .map(|v| {
            let mut visited = HashSet::new();
            let mut label = v;

            dfs(graph, true, v, &mut Default::default(), |w| {
                visited.insert(w);
            });
            dfs(graph, false, v, &mut Default::default(), |w| {
                if visited.contains(&w) {
                    label = std::cmp::max(label, w);
                }
            });

            (v, label)
        })
        .collect();
    labels
}

pub fn kosaraju(graph: &GraphMesh<'_>) -> Vec<(usize, usize)> {
    let mut seen = Default::default();
    let mut order = std::collections::VecDeque::new();
    for v in graph.vertices() {
        dfs(graph, true, v, &mut seen, |x| order.push_front(x));
    }
    seen.clear();

    let mut roots = HashMap::new();
    let mut root_label = HashMap::new();
    for v in order.into_iter() {
        let mut label = v;
        dfs(graph, false, v, &mut seen, |x| {
            roots.insert(x, v);
            label = std::cmp::max(label, x);
        });
        root_label.insert(v, label);
    }

    println!("a {}", roots[&0]);
    println!("b {}", root_label[&roots[&0]]);

    graph
        .vertices()
        .map(|v| {
            let root = roots[&v];
            let label = root_label[&root];
            (v, label)
        })
        .collect()
}

pub fn crit_naive_n3(c: &mut Criterion) {
    let edges = graph_edges();
    let expected = scc_labels();

    c.bench_function("scc/naive_n3", |b| {
        b.iter(|| {
            let graph = edges_to_graph(edges.iter().copied());
            let graph = graph.finish();

            let labels = naive_n3(&graph);
            assert_eq!(expected, &*labels);
        });
    });
}

pub fn crit_kosaraju(c: &mut Criterion) {
    let edges = graph_edges();
    let expected = scc_labels();

    c.bench_function("scc/kosaraju", |b| {
        b.iter(|| {
            let graph = edges_to_graph(edges.iter().copied());
            let graph = graph.finish();

            let labels = kosaraju(&graph);
            let labels: Vec<_> = labels.into_iter().collect();
            expected
                .iter()
                .zip(labels.iter())
                .for_each(|(a, b)| assert_eq!(a, b));
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

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Cursor};
use std::time::Duration;

use criterion::Criterion;
use once_cell::sync::OnceCell;

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
    from: usize,
    seen: &mut HashSet<usize>,
    mut visit: impl FnMut(usize),
) {
    let mut stack = vec![from];
    while let Some(v) = stack.pop() {
        if !seen.contains(&v) {
            (visit)(v);
            seen.insert(v);
            if let Some(nexts) = adj_list.get(&v) {
                stack.extend(nexts.iter().filter(|&next| !seen.contains(next)));
            }
        }
    }
}

pub fn naive_n3(
    size: usize,
    forw: &HashMap<usize, Vec<usize>>,
    back: &HashMap<usize, Vec<usize>>,
) -> Vec<(usize, usize)> {
    let labels: Vec<_> = (0..size)
        .map(|v| {
            let mut visited = HashSet::new();
            let mut label = v;

            dfs(forw, v, &mut Default::default(), |w| {
                visited.insert(w);
            });
            dfs(back, v, &mut Default::default(), |w| {
                if visited.contains(&w) {
                    label = std::cmp::max(label, w);
                }
            });

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
    let mut seen = Default::default();
    let mut order = std::collections::VecDeque::new();
    for v in 0..size {
        dfs(forw, v, &mut seen, |x| order.push_front(x));
    }
    seen.clear();

    let mut roots = HashMap::new();
    let mut root_label = HashMap::new();
    for v in order.into_iter() {
        let mut label = v;
        dfs(back, v, &mut seen, |x| {
            roots.insert(x, v);
            label = std::cmp::max(label, x);
        });
        root_label.insert(v, label);
    }

    println!("a {}", roots[&0]);
    println!("b {}", root_label[&roots[&0]]);

    (0..size)
        .map(|v| {
            let root = roots[&v];
            let label = root_label[&root];
            (v, label)
        })
        .collect()
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

criterion::criterion_group!(
    name = scc;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(10));
    targets =
        // crit_naive_n3,
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

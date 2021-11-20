use criterion::{criterion_group, criterion_main, Criterion};

const NUM_OPS: usize = 100;
const NUM_INTS: usize = 50_000;

fn benchmark_vec_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/vec_vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut stuff = Vec::new();
            {
                let handoff_a = (0..NUM_INTS).collect::<Buf>();
                stuff.push(handoff_a);
                let handoff_b = Buf::new();
                stuff.push(handoff_b);
            }

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut stuff[0]) {
                    stuff[1].push(x);
                }
                for x in std::mem::take(&mut stuff[1]) {
                    stuff[0].push(x);
                }
            }
        });
    });
}

fn benchmark_vec_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/vec_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut stuff = Vec::new();
            {
                let handoff_a = (0..NUM_INTS).collect::<Buf>();
                stuff.push(handoff_a);
                let handoff_b = Buf::new();
                stuff.push(handoff_b);
            }

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut stuff[0]) {
                    stuff[1].push_back(x);
                }
                for x in std::mem::take(&mut stuff[1]) {
                    stuff[0].push_back(x);
                }
            }
        });
    });
}

fn benchmark_vecdeque_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/vecdeque_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut stuff = VecDeque::new();
            {
                let handoff_a = (0..NUM_INTS).collect::<Buf>();
                stuff.push_back(handoff_a);
                let handoff_b = Buf::new();
                stuff.push_back(handoff_b);
            }

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut stuff[0]) {
                    stuff[1].push_back(x);
                }
                for x in std::mem::take(&mut stuff[1]) {
                    stuff[0].push_back(x);
                }
            }
        });
    });
}

criterion_group!(vec_deque,
    benchmark_vec_vec,
    benchmark_vec_vecdeque,
    benchmark_vecdeque_vecdeque,
);
criterion_main!(vec_deque);

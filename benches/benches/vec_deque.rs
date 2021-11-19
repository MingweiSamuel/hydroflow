use std::any::Any;
use std::cell::RefCell;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

const NUM_OPS: usize = 20;
const NUM_INTS: usize = 100_000;

fn benchmark_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/vec", |b| {
        b.iter(|| {
            type Buf = RefCell<Vec<usize>>;

            let handoffs: Vec<Box<dyn Any>> = (0..=NUM_OPS)
                .map(|_| Box::new(Buf::default()) as Box<dyn Any>)
                .collect();

            *handoffs[0].downcast_ref::<Buf>().unwrap().borrow_mut() = (0..NUM_INTS).collect();

            for i in 0..NUM_OPS {
                let mut handoff_prev = handoffs[i].downcast_ref::<Buf>().unwrap().borrow_mut();
                let mut handoff_next = handoffs[i + 1].downcast_ref::<Buf>().unwrap().borrow_mut();
                for x in std::mem::take(&mut *handoff_prev) {
                    handoff_next.push(x);
                }
            }

            for x in std::mem::take(&mut *handoffs[NUM_OPS].downcast_ref::<Buf>().unwrap().borrow_mut()) {
                black_box(x);
            }
        });
    });
}

fn benchmark_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/vecdeque", |b| {
        b.iter(|| {
            type Buf = RefCell<VecDeque<usize>>;

            let handoffs: Vec<Box<dyn Any>> = (0..=NUM_OPS)
                .map(|_| Box::new(Buf::default()) as Box<dyn Any>)
                .collect();

            *handoffs[0].downcast_ref::<Buf>().unwrap().borrow_mut() = (0..NUM_INTS).collect();

            for i in 0..NUM_OPS {
                let mut handoff_prev = handoffs[i].downcast_ref::<Buf>().unwrap().borrow_mut();
                let mut handoff_next = handoffs[i + 1].downcast_ref::<Buf>().unwrap().borrow_mut();
                for x in std::mem::take(&mut *handoff_prev) {
                    handoff_next.push_back(x);
                }
            }

            for x in std::mem::take(&mut *handoffs[NUM_OPS].downcast_ref::<Buf>().unwrap().borrow_mut()) {
                black_box(x);
            }
        });
    });
}

criterion_group!(vec_deque, benchmark_vec, benchmark_vecdeque,);
criterion_main!(vec_deque);

use std::any::Any;
use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main, Criterion};

const NUM_OPS: usize = 20;
const NUM_INTS: usize = 100_000;

#[derive(Default)]
pub struct StateManager {
    states: Vec<Box<dyn Any>>,
}
impl StateManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_state<T: 'static>(&mut self, val: T) -> StateHandle<T> {
        let i = self.states.len();
        self.states.push(Box::new(val));
        StateHandle {
            i,
            _phantom: PhantomData,
        }
    }

    pub fn get_ref<T: 'static>(&self, handle: StateHandle<T>) -> &T {
        self.states[handle.i].downcast_ref().unwrap()
    }

    pub fn get_mut<T: 'static>(&mut self, handle: StateHandle<T>) -> &mut T {
        self.states[handle.i].downcast_mut().unwrap()
    }
}

pub struct StateHandle<T> {
    i: usize,
    _phantom: PhantomData<*mut T>,
}
impl<T> Clone for StateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            i: self.i,
            _phantom: PhantomData,
        }
    }
}
impl<T> Copy for StateHandle<T> {}

fn benchmark_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut state_mgr = StateManager::new();

            let handoff = state_mgr.new_state((0..NUM_INTS).collect::<Buf>());
            let evns = state_mgr.new_state(Buf::new());
            let odds = state_mgr.new_state(Buf::new());

            for _ in 0..NUM_OPS {
                for x in std::mem::take(state_mgr.get_mut(handoff)) {
                    if x % 2 == 0 {
                        state_mgr.get_mut(evns).push(x);
                    } else {
                        state_mgr.get_mut(odds).push(x);
                    }
                }

                let evns_iter = std::mem::take(state_mgr.get_mut(evns)).into_iter();
                let odds_iter = std::mem::take(state_mgr.get_mut(odds)).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    state_mgr.get_mut(handoff).push(x);
                }
            }
        });
    });
}

fn benchmark_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut state_mgr = StateManager::new();

            let handoff = state_mgr.new_state((0..NUM_INTS).collect::<Buf>());
            let evns = state_mgr.new_state(Buf::new());
            let odds = state_mgr.new_state(Buf::new());

            for _ in 0..NUM_OPS {
                for x in std::mem::take(state_mgr.get_mut(handoff)) {
                    if x % 2 == 0 {
                        state_mgr.get_mut(evns).push_back(x);
                    } else {
                        state_mgr.get_mut(odds).push_back(x);
                    }
                }

                let evns_iter = std::mem::take(state_mgr.get_mut(evns)).into_iter();
                let odds_iter = std::mem::take(state_mgr.get_mut(odds)).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    state_mgr.get_mut(handoff).push_back(x);
                }
            }
        });
    });
}

criterion_group!(vec_deque, benchmark_vec, benchmark_vecdeque,);
criterion_main!(vec_deque);

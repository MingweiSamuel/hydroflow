use std::any::Any;
use std::marker::PhantomData;

use criterion::{criterion_group, criterion_main, Criterion};

const NUM_OPS: usize = 100;
const NUM_INTS: usize = 50_000;

#[derive(Default)]
pub struct AnyStateManager {
    states: Vec<Box<dyn Any>>,
}
impl AnyStateManager {
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

pub struct StaticStateManager<T> {
    states: Vec<Box<T>>,
}
impl<T> Default for StaticStateManager<T> {
    fn default() -> Self {
        Self {
            states: Default::default(),
        }
    }
}
impl<T> StaticStateManager<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_state(&mut self, val: T) -> StateHandle<T> {
        let i = self.states.len();
        self.states.push(Box::new(val));
        StateHandle {
            i,
            _phantom: PhantomData,
        }
    }

    pub fn get_ref(&self, handle: StateHandle<T>) -> &T {
        &self.states[handle.i]
    }

    pub fn get_mut(&mut self, handle: StateHandle<T>) -> &mut T {
        &mut self.states[handle.i]
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


fn benchmark_dyn_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/dyn_vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut state_mgr = AnyStateManager::new();

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

fn benchmark_dyn_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/dyn_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut state_mgr = AnyStateManager::new();

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

fn benchmark_static_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/static_vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut state_mgr = StaticStateManager::new();

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

fn benchmark_static_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/static_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut state_mgr = StaticStateManager::new();

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

fn benchmark_vecbox_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/vecbox_vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut stuff = Vec::new();
            {
                let handoff = Box::new((0..NUM_INTS).collect::<Buf>());
                stuff.push(handoff);
                let evns = Box::new(Buf::new());
                stuff.push(evns);
                let odds = Box::new(Buf::new());
                stuff.push(odds);
            }

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut *stuff[0]) {
                    if x % 2 == 0 {
                        stuff[1].push(x);
                    } else {
                        stuff[2].push(x);
                    }
                }

                let evns_iter = std::mem::take(&mut *stuff[1]).into_iter();
                let odds_iter = std::mem::take(&mut *stuff[2]).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    stuff[0].push(x);
                }
            }
        });
    });
}

fn benchmark_vecbox_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/vecvbox_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut stuff = Vec::new();
            {
                let handoff = Box::new((0..NUM_INTS).collect::<Buf>());
                stuff.push(handoff);
                let evns = Box::new(Buf::new());
                stuff.push(evns);
                let odds = Box::new(Buf::new());
                stuff.push(odds);
            }

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut *stuff[0]) {
                    if x % 2 == 0 {
                        stuff[1].push_back(x);
                    } else {
                        stuff[2].push_back(x);
                    }
                }

                let evns_iter = std::mem::take(&mut *stuff[1]).into_iter();
                let odds_iter = std::mem::take(&mut *stuff[2]).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    stuff[0].push_back(x);
                }
            }
        });
    });
}

fn benchmark_box_vec(c: &mut Criterion) {
    c.bench_function("vec_deque/box_vec", |b| {
        b.iter(|| {
            type Buf = Vec<usize>;

            let mut handoff = Box::new((0..NUM_INTS).collect::<Buf>());
            let mut evns = Box::new(Buf::new());
            let mut odds = Box::new(Buf::new());

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut *handoff) {
                    if x % 2 == 0 {
                        evns.push(x);
                    } else {
                        odds.push(x);
                    }
                }

                let evns_iter = std::mem::take(&mut *evns).into_iter();
                let odds_iter = std::mem::take(&mut *odds).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    handoff.push(x);
                }
            }
        });
    });
}

fn benchmark_box_vecdeque(c: &mut Criterion) {
    use std::collections::VecDeque;

    c.bench_function("vec_deque/box_vecdeque", |b| {
        b.iter(|| {
            type Buf = VecDeque<usize>;

            let mut handoff = Box::new((0..NUM_INTS).collect::<Buf>());
            let mut evns = Box::new(Buf::new());
            let mut odds = Box::new(Buf::new());

            for _ in 0..NUM_OPS {
                for x in std::mem::take(&mut *handoff) {
                    if x % 2 == 0 {
                        evns.push_back(x);
                    } else {
                        odds.push_back(x);
                    }
                }

                let evns_iter = std::mem::take(&mut *evns).into_iter();
                let odds_iter = std::mem::take(&mut *odds).into_iter();
                for x in evns_iter.chain(odds_iter) {
                    handoff.push_back(x);
                }
            }
        });
    });
}

criterion_group!(vec_deque,
    benchmark_dyn_vec,
    benchmark_dyn_vecdeque,
    benchmark_static_vec,
    benchmark_static_vecdeque,
    benchmark_vecbox_vec,
    benchmark_vecbox_vecdeque,
    benchmark_box_vec,
    benchmark_box_vecdeque,
);
criterion_main!(vec_deque);

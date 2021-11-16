use std::any::Any;
use std::collections::VecDeque;
use std::marker::PhantomData;

use slotmap::{DefaultKey, Key, SlotMap};

pub struct Pyro<Sid = DefaultKey, Tid = DefaultKey>
where
    Sid: Key,
    Tid: Key,
{
    state: SlotMap<Sid, Box<dyn Any>>,
    tasks: SlotMap<Tid, Box<dyn Task<Sid, Tid>>>,

    ready_queue: VecDeque<Tid>,
}
impl<Sid, Tid> Default for Pyro<Sid, Tid>
where
    Sid: Key,
    Tid: Key,
{
    fn default() -> Self {
        let (state, tasks, ready_queue) = Default::default();
        Self {
            state,
            tasks,
            ready_queue,
        }
    }
}
impl Pyro<DefaultKey, DefaultKey> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<Sid, Tid> Pyro<Sid, Tid>
where
    Sid: Key,
    Tid: Key,
{
    pub fn with_key() -> Self {
        Default::default()
    }
    pub fn tick(&mut self) {
        while let Some(tid) = self.ready_queue.pop_front() {
            let context = Context {
                tid,
                state: &mut self.state,
                ready_queue: &mut self.ready_queue,
            };
            let task = self.tasks.get_mut(tid).expect("Task not found");
            task.run(context);
        }
    }
    pub fn schedule(&mut self, tid: Tid) {
        self.ready_queue.push_back(tid);
    }

    pub fn new_state<T>(&mut self, state: T) -> StateHandle<T, Sid>
    where
        T: Any,
    {
        let sid = self.state.insert(Box::new(state));
        StateHandle {
            sid,
            _phantom: PhantomData,
        }
    }
    pub fn default_state<T>(&mut self) -> StateHandle<T, Sid>
    where
        T: Any + Default,
    {
        let sid = self.state.insert(Box::new(T::default()));
        StateHandle {
            sid,
            _phantom: PhantomData,
        }
    }

    pub fn new_task<F>(&mut self, f: F) -> Tid
    where
        F: 'static + Task<Sid, Tid>,
    {
        self.tasks.insert(Box::new(f))
    }
}

pub struct Context<'a, Sid = DefaultKey, Tid = DefaultKey>
where
    Sid: Key,
    Tid: Key,
{
    tid: Tid,
    state: &'a mut SlotMap<Sid, Box<dyn Any>>,
    ready_queue: &'a mut VecDeque<Tid>,
}
impl<'a, Sid, Tid> Context<'a, Sid, Tid>
where
    Sid: Key,
    Tid: Key,
{
    pub fn current_tid(&self) -> Tid {
        self.tid
    }

    pub fn get_state_ref<T>(&self, state_handle: StateHandle<T, Sid>) -> &T
    where
        T: 'static,
    {
        self.state
            .get(state_handle.sid)
            .expect("Failed to find state for Sid.")
            .downcast_ref()
            .expect("StateHandle wrong type T, cannot cast.")
    }
    pub fn get_state_mut<T>(&mut self, state_handle: StateHandle<T, Sid>) -> &mut T
    where
        T: 'static,
    {
        self.state
            .get_mut(state_handle.sid)
            .expect("Failed to find state for Sid.")
            .downcast_mut()
            .expect("StateHandle wrong type T, cannot cast.")
    }

    pub fn schedule(&mut self, tid: Tid) {
        if !self.ready_queue.contains(&tid) {
            self.ready_queue.push_back(tid);
        }
    }
}

pub struct StateHandle<T, Sid = DefaultKey>
where
    Sid: Key,
{
    sid: Sid,
    _phantom: PhantomData<fn() -> T>,
}
impl<T, Sid> Clone for StateHandle<T, Sid>
where
    Sid: Key,
{
    fn clone(&self) -> Self {
        Self {
            sid: self.sid,
            _phantom: PhantomData,
        }
    }
}

impl<T, Sid> Copy for StateHandle<T, Sid> where Sid: Key {}

pub trait Task<Sid = DefaultKey, Tid = DefaultKey>
where
    Sid: Key,
    Tid: Key,
{
    fn run(&mut self, context: Context<'_, Sid, Tid>);
}
impl<F, Sid, Tid> Task<Sid, Tid> for F
where
    Sid: Key,
    Tid: Key,
    F: FnMut(Context<'_, Sid, Tid>),
{
    fn run(&mut self, context: Context<'_, Sid, Tid>) {
        (self)(context);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn test_basic() {
        let mut pyro = Pyro::new();

        let handoff_handle: StateHandle<VecDeque<usize>> = pyro.default_state();

        let output: Rc<RefCell<Vec<usize>>> = Default::default();
        let output_send = output.clone();

        // Sink
        let sink_tid = pyro.new_task(move |mut ctx: Context<'_>| {
            for x in ctx.get_state_mut(handoff_handle).drain(..) {
                output_send.borrow_mut().push(x);
            }
        });

        let source_tid = pyro.new_task(move |mut ctx: Context<'_>| {
            let handoff = ctx.get_state_mut(handoff_handle);
            for x in 0..100 {
                handoff.push_back(x);
            }
            ctx.schedule(sink_tid);
        });

        pyro.schedule(source_tid);
        pyro.tick();

        assert_eq!(&(0..100).collect::<Vec<_>>(), &*output.borrow());
    }
}

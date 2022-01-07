//! What the builder needs to do:
//! 1. Represent the structure of the graph (chain together exactly like iterators).
//! 2. Construct input and output `TypeList`s and create the subgraph.
//! 3. Connect up handoffs after subgraphs have been created.
//! 4. RUN CODE. (or produce something which runs).

mod chain_pull;
mod filter_pull;
mod flat_map_pull;
mod flat_map_push;
mod for_each_push;
mod hydroflow_builder;
mod identity_push;
mod join_pull;
mod map_pull;
mod map_push;
mod pivot;
mod tee_push;

pub use chain_pull::ChainPull;
pub use filter_pull::FilterPull;
pub use flat_map_pull::FlatMapPull;
pub use flat_map_push::FlatMapPush;
pub use for_each_push::ForEachPush;
pub use hydroflow_builder::HydroflowBuilder;
pub use identity_push::IdentityPushBuild;
pub use join_pull::JoinPull;
pub use map_pull::MapPull;
pub use map_push::MapPush;
pub use pivot::{Pivot, PivotBuild};
pub use tee_push::TeePush;

use std::hash::Hash;

use crate::compiled::Pusherator;
use crate::scheduled::handoff::{CanReceive, Handoff, HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

/// If this was directly on [`Pull`], the `Build<'i>` GAT would need an extra
/// `where Self: 'i` bound to prevent a funny edge case when returning `Build<'i> = Self`.
/// This avoids that: https://github.com/rust-lang/rust/issues/87479
pub trait PullBase {
    type Item;
    type Build<'i>: Iterator<Item = Self::Item>;
}
/// Surface API trait for building the pull half of a subgraph.
///
/// This trait is used to:
/// 1. Represent the structure of the graph.
/// 2. Construct the `InputHandoffs` `HandoffList` type for the subgraph.
/// 3. Produce the `Build<'i>` iterator each time the subgraph runs.
/// 4. Provide the surface chaining API!
pub trait Pull: PullBase {
    type InputHandoffs: HandoffList;

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort);

    /// Builds the iterator for a single run of the subgraph.
    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_>;

    fn chain<I>(self, pull: I) -> ChainPull<Self, I>
    where
        Self: Sized,
        I: Pull<Item = Self::Item>,
    {
        ChainPull::new(self, pull)
    }

    fn join<I, K, VSelf, VI>(self, pull: I) -> JoinPull<Self, I, K, VSelf, VI>
    where
        Self: Sized + PullBase<Item = (K, VSelf)>,
        I: Pull<Item = (K, VI)>,
        K: 'static + Eq + Hash + Clone,
        VSelf: 'static + Eq + Clone,
        VI: 'static + Eq + Clone,
    {
        JoinPull::new(self, pull)
    }

    fn map<F, B>(self, func: F) -> MapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        MapPull::new(self, func)
    }

    fn filter<F>(self, func: F) -> FilterPull<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
    {
        FilterPull::new(self, func)
    }

    fn flat_map<F, U>(self, func: F) -> FlatMapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        FlatMapPull::new(self, func)
    }

    // TODO(mingwei): Dedicated FilterMap impl struct.
    fn filter_map<F, B>(self, func: F) -> FlatMapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        FlatMapPull::new(self, func)
    }

    // fn flatten(self) -> FlatMapPull<Self, std::convert::identity> // expected type, found function `std::convert::identity`
    // where
    //     Self: Sized,
    //     Self::Item: IntoIterator,
    // {
    //     FlatMapPull::new(self, std::convert::identity)
    // }

    fn pivot(self) -> PivotBuild<Self>
    where
        Self: Sized,
    {
        PivotBuild::new(self)
    }
}

/// Helper, see [`PullBase`] for why this exists, and [`Push`] for where this is used.
pub trait PushBase {
    type Item;
    type Build<'a, 'i>: Pusherator<Item = Self::Item>;
}
/// Helper trait for building the push half of a subgraph.
///
/// Unlike [`Push`], this is not the surface API. One more layer is required on
/// the push half in order to reverse the chaining order (push ownership is
/// forward, but chaining naturally builds backwards ownership).
///
/// This trait is used to:
/// 1. Represent the structure of the graph.
/// 2. Construct the `OutputHandoffs` `HandoffList` type for the subgraph.
/// 3. Produce the `Build<'i>` pusherator each time the subgraph runs.
pub trait Push: PushBase {
    type OutputHandoffs: HandoffList;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort);

    fn build<'a, 'i>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i>;
}

/// The surface API for the push half of the subgraph.
pub trait PushBuild {
    type Item;

    type Output<O>
    where
        O: Push<Item = Self::Item>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>;

    fn map<F, C>(self, f: F) -> map_push::MapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> C,
    {
        map_push::MapPushBuild::new(self, f)
    }

    fn flat_map<F, U>(self, f: F) -> flat_map_push::FlatMapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        flat_map_push::FlatMapPushBuild::new(self, f)
    }

    // TODO(mingwei): Dedicated FilterMap impl struct.
    fn filter_map<F, C>(self, f: F) -> flat_map_push::FlatMapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<C>,
    {
        flat_map_push::FlatMapPushBuild::new(self, f)
    }

    fn tee<A, B>(self, a: A, b: B) -> Self::Output<tee_push::TeePush<A, B>>
    where
        Self: Sized,
        A: Push<Item = Self::Item>,
        B: Push<Item = Self::Item>,
        A::Item: Clone,
        A::OutputHandoffs: Extend<B::OutputHandoffs>,
        // Needed to un-concat the handoff lists.
        <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Extended:
            HandoffList + HandoffListSplit<A::OutputHandoffs, Suffix = B::OutputHandoffs>,
    {
        self.build(tee_push::TeePush::new(a, b))
    }

    fn handoff<H>(
        self,
        output_port: hydroflow_builder::BuilderHandoffPush<H, Self::Item>,
    ) -> Self::Output<hydroflow_builder::BuilderHandoffPush<H, Self::Item>>
    where
        Self: Sized,
        H: Handoff + CanReceive<Self::Item>,
    {
        self.build(output_port)
    }

    fn for_each<F>(self, func: F) -> Self::Output<ForEachPush<F, Self::Item>>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.build(ForEachPush::new(func))
    }
}

#[test]
fn test_covid() {
    use crate::scheduled::handoff::VecHandoff;

    type Pid = usize;
    type Name = &'static str;
    type Phone = &'static str;
    type DateTime = usize;

    const TRANSMISSIBLE_DURATION: usize = 14;

    let mut build_ctx = HydroflowBuilder::default();

    let (loop_send, loop_recv) = build_ctx.make_handoff::<VecHandoff<(Pid, DateTime)>, _>();
    let (notifs_send, notifs_recv) = build_ctx.make_handoff::<VecHandoff<(Pid, DateTime)>, _>();

    let (diagnosed_send, diagnosed) =
        build_ctx.add_channel_input::<Option<(Pid, (DateTime, DateTime))>, VecHandoff<_>>();
    let (contacts_send, contacts) =
        build_ctx.add_channel_input::<Option<(Pid, Pid, DateTime)>, VecHandoff<_>>();
    let (peoples_send, peoples) =
        build_ctx.add_channel_input::<Option<(Pid, (Name, Phone))>, VecHandoff<_>>();

    let exposed = loop_recv
        .flat_map(std::convert::identity)
        .map(|(pid, t)| (pid, (t, t + TRANSMISSIBLE_DURATION)))
        .chain(diagnosed.flat_map(std::convert::identity));

    build_ctx.add_subgraph(
        contacts
            .flat_map(std::convert::identity)
            .flat_map(|(pid_a, pid_b, t)| [(pid_a, (pid_b, t)), (pid_b, (pid_a, t))])
            .join(exposed)
            .filter(|(_pid_a, (_pid_b, t_contact), (t_from, t_to))| {
                (t_from..=t_to).contains(&t_contact)
            })
            .map(|(_pid_a, pid_b_t_contact, _t_from_to)| pid_b_t_contact)
            .pivot()
            .map(Some) // For handoff CanReceive.
            .tee(notifs_send, loop_send),
    );

    build_ctx.add_subgraph(
        notifs_recv
            .flat_map(std::convert::identity)
            .join(peoples.flat_map(std::convert::identity))
            .pivot()
            .for_each(|(_pid, exposure_time, (name, phone))| {
                println!(
                    "[{}] To {}: Possible Exposure at t = {}",
                    name, phone, exposure_time
                );
            }),
    );

    let mut hydroflow = build_ctx.build();

    {
        peoples_send.give(Some((101, ("Mingwei S", "+1 650 555 7283"))));
        peoples_send.give(Some((102, ("Justin J", "+1 519 555 3458"))));
        peoples_send.give(Some((103, ("Mae M", "+1 912 555 9129"))));
        peoples_send.flush();

        contacts_send.give(Some((101, 102, 1031))); // Mingwei + Justin
        contacts_send.give(Some((101, 201, 1027))); // Mingwei + Joe
        contacts_send.flush();

        let mae_diag_datetime = 1022;

        diagnosed_send.give(Some((
            103, // Mae
            (
                mae_diag_datetime,
                mae_diag_datetime + TRANSMISSIBLE_DURATION,
            ),
        )));
        diagnosed_send.flush();

        hydroflow.tick();

        contacts_send.give(Some((101, 103, mae_diag_datetime + 6))); // Mingwei + Mae
        contacts_send.flush();

        hydroflow.tick();

        peoples_send.give(Some((103, ("Joe H", "+1 510 555 9999"))));
        peoples_send.flush();

        hydroflow.tick();
    }
}

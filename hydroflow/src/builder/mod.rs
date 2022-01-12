//! Hydroflow's Surface API.
//!
//! ## Internal Documentation
//!
//! Due to the limitations of type-level programming in Rust, this code is
//! "baklava" code containing lot of layers. Each layer does one thing, then
//! constructs the next layer(s) down. This table describes what each layer
//! does and is named. Layers are listed starting from the highest
//! (user-facing API) layer and ending with the lowest (code-running) layer.
//!
//! ### Layer Structure
//! ```txt
//!               (A) Surface API
//!            (B) (Surface-Reversed*)
//!                  /            \
//!                 /              \
//!        (C) Connector   (D) Subgraph Builder
//!                                  |
//!                       (E) Iterator/Pusherator
//! ```
//! <sup>*Only used with `Push` to reverse the ownership direction.</sup>
//!
//! ### Layer Descriptions
//! <table>
//! <tr>
//!     <th rowspan="2">Layer</th>
//!     <th rowspan="2">Purpose</th>
//!     <th colspan="2">Naming</th>
//! </tr>
//! <tr>
//!    <th>Pull</th>
//!    <th>Push</th>
//! </tr>
//! <tr>
//!     <td>(A) The Surface API</td>
//!     <td rowspan="2">
//!         &bull; Presents a clean functional-chaining API for users.<br>
//!         &bull; Consumed to simultaneously create a (C) connector and (D) builder.<br>
//!         &bull; <strong>Push Only</strong>: Extra layer needed to reverse ownership order.
//!     </td>
//!     <td><code>PullSurface</code></td>
//!     <td><code>PushSurface</code></td>
//! </tr>
//! <tr>
//!     <td>(B) Surface-Reversed</td>
//!     <td>N/A</td>
//!     <td><code>PushSurfaceReversed</code></td>
//! </tr>
//! <tr>
//!     <td>(C) Connectors</td>
//!     <td>
//!         &bull; Connects <code>OutputPort</code>s and <code>InputPort</code>s, splits type lists in order to do so.<br>
//!         &bull; Does not go to any lower layers.<br>
//!         &bull; Uses the input/output <code>HandoffList</code> variadic type.
//!     </td>
//!     <td><code>PullConnect</code></td>
//!     <td><code>PushConnect</code></td>
//! </tr>
//! <tr>
//!     <td>(D) Subgraph Builders</td>
//!     <td>
//!         &bull; On each subgraph invocation, constructs the (E) iterators and pivot which will be run.<br>
//!         &bull; Is owned by the subgraph task, lends closures to (E) iterators.<br>
//!         &bull; Uses the input/output <code>HandoffList</code> variadic type.
//!     </td>
//!     <td><code>PullBuild</code></td>
//!     <td><code>PushBuild</code></td>
//! </tr>
//! <tr>
//!     <td>(E) Iterators</td>
//!     <td>
//!         &bull; Runs code on individual dataflow elements, in the case of dataflow.<br>
//!         &bull; In the future, will correspond to semilattice morphisms alternatively.
//!     </td>
//!     <td><code>std::iter::Iterator</code></td>
//!     <td><code>Pusherator</code></td>
//! </tr>
//! </table>

pub mod build;
pub mod connect;
pub mod surface;

mod build_pivot;
mod hydroflow_builder;

pub use build_pivot::PivotBuild;
pub use hydroflow_builder::HydroflowBuilder;

#[test]
fn test_covid() {
    use crate::scheduled::handoff::VecHandoff;
    use surface::{BaseSurface, PullSurface, PushSurface};

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

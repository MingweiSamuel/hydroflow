# Graph Reachability

```rust
use hydroflow::builder::prelude::*;

pub fn main() {
    let mut builder = HydroflowBuilder::default();

    let (send_edges, recv_edges) =
        builder.add_channel_input::<_, _, VecHandoff<(usize, usize)>>("edge input");
    let (send_loop, recv_loop) = builder.make_edge::<_, VecHandoff<usize>, _>("loop");

    builder.add_subgraph(
        "main",
        [0].into_hydroflow()
            .chain(recv_loop.flatten())
            .map(|v| (v, ()))
            .join(recv_edges.flatten())
            .pull_to_push()
            .map(|(_old_v, (), new_v)| new_v)
            .inspect(|&v| println!("Reached: {}", v))
            .map(Some)
            .push_to(send_loop),
    );

    let mut hf = builder.build();
    println!("{}", hf.generate_mermaid());

    println!("A");

    send_edges.give(Some((5, 10)));
    send_edges.give(Some((0, 3)));
    send_edges.give(Some((3, 6)));
    send_edges.flush();
    hf.tick();

    println!("B");

    send_edges.give(Some((6, 5)));
    send_edges.flush();
    hf.tick();
}
```

```mermaid
graph TD
  subgraph stratum0
    subgraph main1
      Handoff_0[\edge input handoff/] --> 1.6[Flatten]
      Handoff_1[\loop/] --> 1.4[Flatten]
      1.3[Iter] --> 1.2[Chain]
      1.9[Map] --> 1.10[Map]
      1.12[/PullToPush\] --> 1.8[Map]
      1.6[Flatten] --> 1.0[Join]
      1.0[Join] --> 1.12[/PullToPush\]
      1.8[Map] --> 1.9[Map]
      1.1[Map] --> 1.0[Join]
      1.2[Chain] --> 1.1[Map]
      1.10[Map] --> Handoff_1[\loop/]
      1.4[Flatten] --> 1.2[Chain]
    end
  end
```
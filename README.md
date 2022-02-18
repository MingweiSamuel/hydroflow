# Hydroflow

Hydroflow is a runtime for low-level dataflow and flow-like systems, written
in Rust. As the exeuction level of the [Hydro Project](https://hydro-project.github.io/),
Hydroflow aims to be fast and flexible enough to represent not just batch/stream
processing jobs, but all kinds of distributed systems and protocols, creating a
new way to program the cloud.

## Usage/Development

Hydroflow is evolving as we develop it, so expect some rough edges and feel
free to [ask any questions](https://github.com/hydro-project/hydroflow/issues/new).
The best way to try it out is to clone and run this repo.
Currently Hydroflow depends on some nightly Rust features, and the repo comes
with the right versions pinned.

If Rust is not installed you will need to [install it here](https://www.rust-lang.org/tools/install).

To run tests:
```
cargo test
```

Examples are in [`hydroflow/examples`](https://github.com/hydro-project/hydroflow/tree/main/hydroflow/examples).
`graph_reachability` is the the most approachable as it uses the dataflow surface API.

To test your own example, add an `[[example]]` field to [`hydroflow/Cargo.toml`](https://github.com/hydro-project/hydroflow/blob/main/hydroflow/Cargo.toml)
and creat the corresponding `hydroflow/examples/EXAMPLE_NAME` folder and `main.rs`.

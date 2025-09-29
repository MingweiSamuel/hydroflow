Extra [`Sink`] adaptors and functions.

## Forward Building with `SinkBuilder`

For a more intuitive API that matches the data flow direction, use [`SinkBuilder`] to chain adaptors in forward order:

```rust
use sinktools::{SinkBuilder, SinkBuild};
use sinktools::sink::SinkExt; // for `.send(_).await`

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Forward chain: flatten -> filter_map -> map -> filter -> for_each
let mut pipeline = SinkBuilder::<Vec<i32>>::new()
    .flatten::<Vec<i32>>()          // Flatten input vectors
    .filter_map(|x: i32| {          // Double evens, filter odds
        if x % 2 == 0 {
            Some(x * 2)
        } else {
            None
        }
    })
    .map(|x| x + 1)                 // Add 1
    .filter(|x| *x < 100)           // Only values < 100
    .for_each(|x: i32| {            // Terminal operation
        println!("Received: {}", x);
    });

// Send nested data
pipeline.send(vec![1, 2, 3, 4]).await.unwrap();
pipeline.send(vec![5, 6]).await.unwrap();
pipeline.send(vec![]).await.unwrap();
pipeline.send(vec![7, 8, 9]).await.unwrap();
# }
```

## Backward Building with `Sinktools`

Alternatively, you can extend existing `Sink`s with methods by importing the [`Sinktools`] trait:

```rust
use sinktools::Sinktools;
use sinktools::sink::SinkExt; // for `.send(_).await`

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Complex chain: flatten -> filter_map -> map -> filter
let mut complex_sink = sinktools::ForEach::new(|x: i32| {
        println!("Received: {}", x);
    })
    .un_filter(|x: &i32| *x < 100)  // Only values < 100
    .un_map(|x: i32| x + 1)         // Add 1
    .un_filter_map(|x: i32| {       // Double evens, filter odds
        if x % 2 == 0 {
            Some(x * 2)
        } else {
            None
        }
    })
    .un_flatten::<Vec<i32>>(); // Flatten input vectors

// Send nested data
complex_sink.send(vec![1, 2, 3, 4]).await.unwrap();
complex_sink.send(vec![5, 6]).await.unwrap();
complex_sink.send(vec![]).await.unwrap();
complex_sink.send(vec![7, 8, 9]).await.unwrap();
# }
```

Note that with the backward API, each adaptor will be placed _in front_ of the existing sink, the reverse of [`Iterator`]. All adaptor methods are prefixed with `un_` to indicate this backwards operation.

## Choosing Between Forward and Backward APIs

- **Forward API (`SinkBuilder`)**: More intuitive as it matches the data flow direction. Start with `SinkBuilder::new()` and chain adaptors in the order data flows through them.
- **Backward API (`Sinktools`)**: Useful when you already have a sink and want to add processing stages in front of it. Adaptors are applied in reverse order.

Both APIs are functionally equivalent and produce the same results.

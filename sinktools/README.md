Extra [`Sink`] adaptors and functions.

To extend `Sink` with methods in this crate, import the [`Sinktools`] trait. You can now use the methods provided by
`Sinktools`:

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

Note that each adaptor will be placed _in front_ of the existing sink, the reverse of [`Iterator`]. All adaptor methods
are prefixed with `un_` to indicate this backwards operation.

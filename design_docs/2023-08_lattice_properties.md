# Lattice Properties

## Goals

1. Make monotonicity the easy and default option, make non-monotonic operations the special case.
2. Reject operations that are incorrect (would violate monotonicity).  
   E.g. can't use a order-dependent fold on an arbitrarily ordered stream.
3. Reason about and optimize Hydroflow graphs at proc-macro time.  
   What portions can be parallelized, partitioned, etc.

## Design

Introduce _stream types_, as a layer on top of lattice types. The stream type represents sequential
information about the lattice instances, such as ordering, sorting, monotonicity, or atomization.

* `SeqFlow<*, T>`
* `DiffLatticeFlow<Lat>`
* `CumuLatticeFlow<Lat>`

`SeqFlow<T>` is a special per-element representation of the `Seq<*, T>` lattice type.

Stream types are **NOT** automatically infered. It will be up to the user to explicitly switch
between different stream types.
An alternative, using Rust's type system to infer stream types, is too fragile and more importantly
cannot be used a proc-macro time which prevents goal #3. Having the user manually specify stream
types ensures the scaling and monotonicity of the system will be top-of-mind, and avoids the
complexity of implementing our own type inference system.

Stream type topology. Stream types can be cast upwards:
```mermaid
flowchart BT
seq["SeqFlow&lt;*, T&gt;"]
dif["DiffLatticeFlow&lt;Lat&gt;"]
cum["CumuLatticeFlow&lt;Lat&gt;"]

cum --> seq
dif --> seq
```

Monotonic function topology:
```mermaid
flowchart BT
any["any &quot;function&quot;"]
fun["deterministic function"]
mono["monotonic function"]
morph["morphism"]

morph --> mono
mono --> fun
fun --> any
```

---

Sending bottom $\bot$ through a [lattice flow] stream should have the exact same behavior as sending nothing
through.

<details>
   <summary>Note: bottom in a SeqStream is not SeqStream's bottom</summary>

```rust
Seq = VecUnion<Point<*, T>>
Seq bottom = vec![]
vec![bottom, bottom, bottom] is not Seq's bottom
```
</details>

## Operators

```rust
// input: set {1, 2, 3, 4}

// map stream
input -> random_batches()
   // input: { 1 }, { 2 }, { 3 }, { 4 }
   // NOT A MORPHISM ILLEGAL
   // the map function is a set union morphism if it acts on the atoms.
   -> map(|x: Set| if x.all(is_even) { OptionSet(x) } else { OptionSet(None) }) -> output
   // { 2 }, { 4 }

// filter stream
input -> atomize()
   // input: { 1 }, { 2 }, { 3 }, { 4 }
   -> filter(|x: Set| if x.all(is_even)) -> output
   // { 2 }, { 4 }
```

## TODO: start with cumul thing

```rust
// input: set {1, 2, 3, 4}

// map stream
input
   -> map(|x: Set| if x.all(is_even) { OptionSet(x) } else { OptionSet(None) }) -> output

// filter stream
input
   -> filter(|x: (x)| 0 == x % 2) -> output
   // { 2 }, { 4 }
```

| Input(s) | Operator | Output(s) | Condition |
| --- | --- | --- | --- |
| `SeqFlow<*1, T>` | `map(f)` | `SeqFlow<*2, U>` | `f: Fn(T) -> U` |
| `CumuLatticeFlow<Lat1>` | `map(f)` | `CumuLatticeFlow<Lat2>` | `f: MonotonicFn(Lat1) -> Lat2` |
| `DiffLatticeFlow<Lat1>` | `map(f)` | `DiffLatticeFlow<Lat2>` | `f: Morphism(Lat1) -> Lat2` |

| Input(s) | Operator | Output(s) | Condition |
| --- | --- | --- | --- |
| `SeqFlow<*1, T>` | `filter(p)` | `SeqFlow<*2, T>` | `p: Fn(&T) -> bool` |
| `CumuLatticeFlow<Lat>` | `filter(p)` | `CumuLatticeFlow<Lat>` | `p: MonotonicFn(&Lat) -> Max<bool>` |
| `DiffLatticeFlow<Lat>` | `filter(p)` | ILLEGAL | |

`filter()` doesn't make sense on difflattice flows, even if the predicate is a morphism

Filter doesn't make sense!!!! No more filter.



| Input(s) | Operator | Output(s) | Condition |
| --- | --- | --- | --- |
| `SeqFlow<*1, (K, V1)>`, `SeqFlow<*2, (K, V2)>` | `join()` | `SeqFlow<*3, (K, (V1, V2))>` | |
| | | |
| `LatticeFlow<Lat>` | `unmerge()` | `DiffLatticeFlow<Lat>` | |
| `LatticeFlow<Lat>` | `merge()` | `CumuLatticeFlow<Lat>` | |
| any, $N$ times | `union()` | same out | |
| any | `tee()` | same out, $N$ times | |
| | | |
| `SeqFlow<*1, T>` | `sort()` | `SeqFlow<*SORT, T>` | |
| `LatticeFlow<Lat>` | `sort()` | `SeqFlow<*SORT, Lat>` | |
| | | |

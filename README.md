# array_iter_tools

[![Rust Documentation](https://img.shields.io/crates/v/array_iter_tools?color=blue&label=docs&style=flat-square)][docs.rs]
[![Latest Version](https://img.shields.io/crates/d/array_iter_tools?style=flat-square)][crates.io]

[crates.io]: https://crates.io/crates/array_iter_tools
[docs.rs]: https://docs.rs/array_iter_tools

Modify simple arrays

```rust
use array_iter_tools::{ArrayIterator, IntoArrayIterator};
let a = [1, 2, 3, 4];
let b = [5, 6, 7, 8];
let c = a.into_array_iter().zip(b).map(|(a, b)| a + b).collect();
assert_eq!(c, [6, 8, 10, 12]);
```

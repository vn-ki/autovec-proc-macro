# autovec

`autovec` auto vectorizes normal functions. Code speaks more than words, so here are some examples.

### Example 1: With generics

```rust
#[autovec]
fn fn_3<X: Into<usize>, Y: Into<usize>>( a: X, b: Y, c: String) -> usize {
    return a.into() + b.into();
}
```

turns into

```rust
fn fn_3<X: Into<usize>, Y: Into<usize>>(
    a_vec: Vec<X>,
    b_vec: Vec<Y>,
    c_vec: Vec<String>,
) -> Vec<usize> {
    let n = a_vec.len();
    if n != b_vec.len() {
        panic!("b_vec len does not match with other vectors")
    }
    if n != c_vec.len() {
        panic!("c_vec len does not match with other vectors")
    }
    let ret = a_vec
        .into_iter()
        .zip(b_vec.into_iter())
        .zip(c_vec.into_iter())
        .map(|((a, b), c)| -> usize {
            return a.into() + b.into();
        })
        .collect();
    return ret;
}
```

### Example 2: Tuple syntax

```rust
struct Location {
   x: i64,
   y: i64,
}

#[autovec]
fn fn_4((Location { x, .. }, Location { y, .. }): (Location, Location)) -> i64 {
   x * y
}
```

turns into

```rust
struct Location {
    x: i64,
    y: i64,
}
fn fn_4(arg1: Vec<(Location, Location)>) -> Vec<i64> {
    let n = arg1.len();
    let ret = arg1
        .into_iter()
        .map(|(Location { x, .. }, Location { y, .. })| -> i64 { x * y })
        .collect();
    return ret;
}
```

## Tests

- Tests which check compilation are [tests](tests/)
- Tests which check correctness are [example/tests](example/tests/)

```bash
cargo test && cargo test example
```

## TODO

- [ ] Support more argument patterns (ref: [syn::Pat](https://docs.rs/syn/1.0.41/syn/enum.Pat.html), [rust patterns](https://doc.rust-lang.org/reference/patterns.html))

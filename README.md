# Helper

[![Rust](https://github.com/jmjoy/helper/actions/workflows/rust.yml/badge.svg)](https://github.com/jmjoy/helper/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/helper.svg)](https://crates.io/crates/helper)
[![API](https://docs.rs/helper/badge.svg)](https://docs.rs/helper)

A library provided some useful proc macros for Rust.

## Macros

- **control flow**

  - [**either**](https://docs.rs/helper/latest/helper/macro.either.html): Ternary operator in many C-like languages.

- **collections**

  - [**btmap**](https://docs.rs/helper/latest/helper/macro.btmap.html): Create `std::collections::BTreeMap` from list of key-value pairs.
  - [**btset**](https://docs.rs/helper/latest/helper/macro.btset.html): Create a `std::collections::BTreeSet` from a list of elements.
  - [**hmap**](https://docs.rs/helper/latest/helper/macro.hmap.html): Create `std::collections::HashMap` from list of key-value pairs.
  - [**hset**](https://docs.rs/helper/latest/helper/macro.hset.html): Create a `std::collections::HashSet` from a list of elements.

- **type conversion**

  - [**u8**](https://docs.rs/helper/latest/helper/macro.u8.html): Convert expression to `u8` type.
  - [**u16**](https://docs.rs/helper/latest/helper/macro.u16.html): Convert expression to `u16` type.
  - [**u32**](https://docs.rs/helper/latest/helper/macro.u32.html): Convert expression to `u32` type.
  - [**u64**](https://docs.rs/helper/latest/helper/macro.u64.html): Convert expression to `u64` type.
  - [**u128**](https://docs.rs/helper/latest/helper/macro.u128.html): Convert expression to `u128` type.
  - [**usize**](https://docs.rs/helper/latest/helper/macro.usize.html): Convert expression to `usize` type.
  - [**i8**](https://docs.rs/helper/latest/helper/macro.i8.html): Convert expression to `i8` type.
  - [**i16**](https://docs.rs/helper/latest/helper/macro.i16.html): Convert expression to `i16` type.
  - [**i32**](https://docs.rs/helper/latest/helper/macro.i32.html): Convert expression to `i32` type.
  - [**i64**](https://docs.rs/helper/latest/helper/macro.i64.html): Convert expression to `i64` type.
  - [**i128**](https://docs.rs/helper/latest/helper/macro.i128.html): Convert expression to `i128` type.
  - [**isize**](https://docs.rs/helper/latest/helper/macro.isize.html): Convert expression to `isize` type.
  - [**f32**](https://docs.rs/helper/latest/helper/macro.f32.html): Convert expression to `f32` type.
  - [**f64**](https://docs.rs/helper/latest/helper/macro.f64.html): Convert expression to `f64` type.
  - [**char**](https://docs.rs/helper/latest/helper/macro.char.html): Convert expression to `char` type.
  - [**bool**](https://docs.rs/helper/latest/helper/macro.bool.html): Convert expression to `bool` type.

## Examples

### Type Conversion

```rust
use helper::*;

let x = 42i32;
let y = u8!(x);      // Convert to u8
let z = f64!(x);     // Convert to f64

// Support complex expressions
let result = u64!(x + 10 * 2);

// Support chained conversions
let chained = f64!(i64!(u32!(x)));

// Character conversion
let ascii_value = 65u8;
let character = char!(ascii_value);  // 'A'
```

### Collections

```rust
use helper::*;

// HashMap
let map = hmap! {
    "key1": "value1",
    "key2": "value2",
};

// HashSet
let set = hset! { 1, 2, 3 };

// BTreeMap
let btree_map = btmap! {
    "a": 1,
    "b": 2,
};

// BTreeSet
let btree_set = btset! { 1, 2, 3 };
```

## License

MulanPSL-2.0

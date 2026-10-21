# Helper

[![Rust](https://github.com/jmjoy/helper/actions/workflows/rust.yml/badge.svg)](https://github.com/jmjoy/helper/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/helper.svg)](https://crates.io/crates/helper)
[![API](https://docs.rs/helper/badge.svg)](https://docs.rs/helper)

A library provided some useful proc macros for Rust.

## Macros

- **control flow**

  - [**either**](https://docs.rs/helper/latest/helper/macro.either.html): Select between two expressions with `either!(condition, left, right)`.

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

### Conditional Selection

```rust
use helper::either;

let condition = true;
let value = either!(condition, std::cmp::max(1, 2), 0,);
assert_eq!(value, 2);
```

The condition is evaluated once, and only the selected branch is evaluated.
All three arguments are Rust expressions and support the `?` operator.
A trailing comma is optional.

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

Each conversion accepts one Rust expression and follows Rust's `as` cast rules,
including truncation when narrowing integers. The target is always the named
primitive type, even if a type alias with that name is in scope.

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

Map keys and values and set elements are Rust expressions. Paths (`u8::MAX`),
generic arguments (`Result::<u16, u32>::Ok(1)`), closures, and `?` are parsed as
part of the expression. Maps keep the `key: value` syntax:

```rust
let limits = helper::hmap! {
    u8::MAX: Result::<u16, u32>::Ok(u16::MAX),
};
assert_eq!(limits[&u8::MAX], Ok(u16::MAX));

let operations = helper::btmap! {"add": |a: i32, b: i32| a + b};
assert_eq!(operations["add"](2, 3), 5);
```

Expressions are evaluated once in input order. Later map entries with duplicate
keys replace earlier values, and sets deduplicate elements. Empty collections
and trailing commas are supported; missing keys, values, or elements produce
compile errors. All macros preserve variable bindings and expression precedence
when called from `macro_rules!` wrappers.

## License

MulanPSL-2.0

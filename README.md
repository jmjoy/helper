# Helper

[![Rust](https://github.com/jmjoy/helper/actions/workflows/rust.yml/badge.svg)](https://github.com/jmjoy/helper/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/helper.svg)](https://crates.io/crates/helper)
[![API](https://docs.rs/helper/badge.svg)](https://docs.rs/helper)

A library provided some useful proc macros for Rust.

## Macros

- **compile-time type assertions**

  - [**assert_impl_all**](https://docs.rs/helper/latest/helper/macro.assert_impl_all.html): Assert that a type implements every listed trait.
  - [**assert_not_impl_any**](https://docs.rs/helper/latest/helper/macro.assert_not_impl_any.html): Assert that a type implements none of the listed traits.

- **control flow**

  - [**either**](https://docs.rs/helper/latest/helper/macro.either.html): Select between two expressions with `either!(condition, left, right)`.
  - [**defer**](https://docs.rs/helper/latest/helper/macro.defer.html): Run cleanup when the enclosing scope exits.

- **closure and async captures**

  - [**capture**](https://docs.rs/helper/latest/helper/macro.capture.html): Clone named variables before creating a `move` closure or `async move` block.

- **string literals**

  - [**indoc**](https://docs.rs/helper/latest/helper/macro.indoc.html): Remove common indentation from string and byte string literals at compile time.

- **byte literals**

  - [**concat_bytes**](https://docs.rs/helper/latest/helper/macro.concat_bytes.html): Concatenate byte literals, byte strings, and byte arrays at compile time.
  - [**hex**](https://docs.rs/helper/latest/helper/macro.hex.html): Decode a hexadecimal string literal into a byte array at compile time.

- **file inclusion**

  - [**include_c_str**](https://docs.rs/helper/latest/helper/macro.include_c_str.html): Include a file as a `&'static core::ffi::CStr`, validating NUL bytes at compile time.
  - [**include_str_trimmed**](https://docs.rs/helper/latest/helper/macro.include_str_trimmed.html): Include a UTF-8 file as a `&'static str`, trimming leading and trailing ASCII whitespace at compile time.

- **collections**

  - [**btmap**](https://docs.rs/helper/latest/helper/macro.btmap.html): Create `std::collections::BTreeMap` from list of key-value pairs.
  - [**btmap_into**](https://docs.rs/helper/latest/helper/macro.btmap_into.html): Create a `std::collections::BTreeMap`, converting every key and value with `Into::into`.
  - [**btset**](https://docs.rs/helper/latest/helper/macro.btset.html): Create a `std::collections::BTreeSet` from a list of elements.
  - [**btset_into**](https://docs.rs/helper/latest/helper/macro.btset_into.html): Create a `std::collections::BTreeSet`, converting every element with `Into::into`.
  - [**hmap**](https://docs.rs/helper/latest/helper/macro.hmap.html): Create `std::collections::HashMap` from list of key-value pairs.
  - [**hmap_into**](https://docs.rs/helper/latest/helper/macro.hmap_into.html): Create a `std::collections::HashMap`, converting every key and value with `Into::into`.
  - [**hset**](https://docs.rs/helper/latest/helper/macro.hset.html): Create a `std::collections::HashSet` from a list of elements.
  - [**hset_into**](https://docs.rs/helper/latest/helper/macro.hset_into.html): Create a `std::collections::HashSet`, converting every element with `Into::into`.

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

### Compile-Time Type Assertions

```rust
use helper::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(String: Send, Sync, Clone);
assert_impl_all!(u8: Into<u16>, Into<u32>,);
assert_not_impl_any!(std::rc::Rc<u8>: Send, Sync);
```

Assertions work at module scope or inside blocks for concrete types, and are
checked even if the surrounding code is never executed. Generic trait arguments,
associated type bindings, higher-ranked lifetimes, and unsized types are
supported. At least one comma-separated trait bound is required, with an optional
trailing comma. `assert_not_impl_any!` checks each trait independently: implementing
even one listed trait causes a compile error. Both macros use only `core` and
generate no runtime work.

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

### Cloned Captures

```rust
use helper::capture;
use std::sync::Arc;

let message = Arc::new(String::from("hello"));
let callback = capture!([message], move || message.len());
let task = capture!([message], async move { message.len() });

assert_eq!(callback(), 5);
assert_eq!(message.as_str(), "hello");
drop(task);
```

Each listed variable is cloned once, in list order, before the closure or future
is created. The original bindings remain available, and cloned bindings can be
mutated inside the closure or future. Unlisted variables follow ordinary capture
rules. The list accepts distinct variable names, including raw identifiers, and
may be empty. A trailing comma is allowed in the list and after the expression.
A `move` closure or `async move` block is required.

### Scope Cleanup

```rust
use helper::defer;
use std::cell::RefCell;

let events = RefCell::new(Vec::new());
{
    defer! { events.borrow_mut().push("first"); }
    defer! { events.borrow_mut().push("second"); }
    events.borrow_mut().push("body");
}
assert_eq!(*events.borrow(), ["body", "second", "first"]);
```

Invoke `defer!` as a statement. Its body runs once when that scope exits, including
early `return`, `?`, loop exits, and panic unwinding. Multiple defers run in
reverse registration order. Cleanup does not run when the process aborts or
exits without unwinding. The body follows ordinary closure capture rules and
must return `()`; any borrows last until cleanup runs.

### Indented Text

```rust
use helper::indoc;

const SQL: &str = indoc! {r#"
    SELECT id, name
    FROM users
    WHERE active = true
"#};
assert_eq!(SQL, "SELECT id, name\nFROM users\nWHERE active = true\n");
```

`indoc!` accepts one string or byte string literal, including raw literals, and
an optional trailing comma. It removes an initial line ending and the common
indentation of the remaining nonblank lines. A first line containing text is
preserved unchanged and excluded when computing indentation. Spaces and tabs
each count as one character. Blank lines do not affect the minimum indentation;
when no content lines remain, the minimum is zero. Relative indentation, other
whitespace, line endings, and trailing whitespace are preserved.

The result is a string or byte string literal, so it works in `const` and
`static` initializers and as input to formatting macros with explicit format
arguments. Processing happens at compile time, with no runtime work, allocation,
unsafe code, or dependency on `std`.

### Hexadecimal Bytes

```rust
use helper::hex;

const MAGIC: [u8; 4] = hex!("DE AD be ef");
assert_eq!(MAGIC, [0xde, 0xad, 0xbe, 0xef]);
```

`hex!` accepts one string literal and an optional trailing comma. Uppercase and
lowercase digits are accepted, and all ASCII whitespace is ignored. Empty input
produces `[u8; 0]`. Invalid characters and odd numbers of digits cause compile
errors. Decoding happens at compile time, and the result works in `const` and
`static` initializers.

`capture!`, `defer!`, and `hex!` work in `no_std` crates without an allocator and
generate no unsafe code.

### Byte Concatenation

```rust
use helper::concat_bytes;

const HEADER: &[u8; 4] = concat_bytes!(b"PK", [3, 4]);
assert_eq!(HEADER, b"PK\x03\x04");
assert_eq!(concat_bytes!(b'A', br"BC", [68u8, b'E'], [b'!'; 2],), b"ABCDE!!");
```

Arguments are byte literals, byte string literals, or arrays of byte and integer
literals. Array integers must be in `0..=255` and have no suffix or the `u8`
suffix. Repeated arrays accept an integer literal count with no suffix or the
`usize` suffix. Empty input and trailing commas are supported. The result is a
`&'static [u8; N]` byte string literal, usable in `const`, `static`, and `no_std`
code without runtime copying or allocation.

### Trimmed Text Inclusion

```rust
use helper::include_str_trimmed;

const MANIFEST: &str = include_str_trimmed!(concat!(
    env!("CARGO_MANIFEST_DIR"), "/Cargo.toml",
));
assert!(MANIFEST.starts_with("[package]"));
assert!(!MANIFEST.ends_with('\n'));
```

Leading and trailing ASCII whitespace (space, tab, line feed, vertical tab, form
feed, and carriage return) is removed at compile time. Interior whitespace and
non-ASCII whitespace are preserved; empty and all-whitespace files produce `""`.
The file must contain valid UTF-8. Paths follow `include_str!` resolution and
accept string literals, path macros, and a trailing comma. File changes trigger
recompilation. The result is a `&'static str`, usable in `const`, `static`, and
`no_std` code without runtime allocation or unsafe code, including on Rust 1.72.

### C String Inclusion

```rust
use helper::include_c_str;

const MANIFEST: &core::ffi::CStr = include_c_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Cargo.toml",
));
assert!(MANIFEST.to_bytes().starts_with(b"[package]"));
```

Paths are relative to the file containing the invocation, just like
`include_bytes!`. String literals, macros such as `concat!` and `env!`, and an
optional trailing comma are supported. File changes trigger recompilation.
A missing NUL terminator is appended, and an existing single terminator is
preserved. Interior NUL bytes, including repeated trailing NULs, cause a compile
error. Empty files and non-UTF-8 bytes are supported.

The result works in `const` and `static` initializers and in `no_std` crates.
Copying and validation happen at compile time, with no runtime allocation or
unsafe code. Requires Rust 1.72 or later.

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

Use `hmap_into!`, `hset_into!`, `btmap_into!`, and `btset_into!` to convert inputs
to the target types:

```rust
use helper::{btmap_into, btset_into, hmap_into, hset_into};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

let headers: HashMap<String, String> = hmap_into! {
    "Accept": "application/json",
    "User-Agent": String::from("helper"),
};
let names: HashSet<String> = hset_into! {"alice", String::from("bob"), "alice"};
let sorted: BTreeMap<String, String> = btmap_into! {"z": "last", "a": "first"};
let sorted_names: BTreeSet<String> = btset_into! {"bob", "alice", "bob"};

assert_eq!(headers["Accept"], "application/json");
assert_eq!(names.len(), 2);
assert_eq!(sorted.keys().map(String::as_str).collect::<Vec<_>>(), ["a", "z"]);
assert_eq!(sorted_names.into_iter().collect::<Vec<_>>(), ["alice", "bob"]);
```

The target types must be determined by the surrounding context, typically a type
annotation. Each input is evaluated and converted with `Into::into` once in
input order. Map keys are converted before their values; duplicate converted
keys keep the last value, and sets deduplicate converted elements. Empty
collections, trailing commas, complex expressions, and `?` are supported.

Expressions are evaluated once in input order. Later map entries with duplicate
keys replace earlier values, and sets deduplicate elements. Empty collections
and trailing commas are supported; missing keys, values, or elements produce
compile errors. All macros preserve variable bindings and expression precedence
when called from `macro_rules!` wrappers.

## License

MulanPSL-2.0

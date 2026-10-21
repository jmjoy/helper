// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, non_ascii_idents)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

pub(crate) mod assertions;
pub(crate) mod capture;
pub(crate) mod collections;
pub(crate) mod control;
pub(crate) mod strings;
pub(crate) mod types;
pub(crate) mod utils;

use proc_macro::TokenStream;

/// Assert at compile time that a type implements every listed trait.
///
/// Accepts `Type: Trait, OtherTrait, ...`, with an optional trailing comma.
/// Bounds can contain generic arguments, associated type bindings, and
/// higher-ranked lifetimes. Unsized types are supported. At least one trait is
/// required; bounds with `?` are rejected.
///
/// Assertions are anonymous const items, usable at module scope or inside a
/// block for concrete types. They generate no runtime work and use only `core`.
///
/// # Examples
/// ```
/// use helper::assert_impl_all;
///
/// assert_impl_all!(String: Send, Sync, Clone);
/// assert_impl_all!(u8: Into<u16>, Into<u32>,);
/// assert_impl_all!(str: Send, Sync);
/// ```
///
/// Missing implementations cause a compile error:
/// ```compile_fail
/// helper::assert_impl_all!(std::rc::Rc<u8>: Send, Sync);
/// ```
/// A nonempty comma-separated trait list is required:
/// ```compile_fail
/// helper::assert_impl_all!(u8:);
/// ```
#[proc_macro]
pub fn assert_impl_all(items: TokenStream) -> TokenStream {
    assertions::assert_impl_all(items.into()).into()
}

/// Assert at compile time that a type implements none of the listed traits.
///
/// Each trait is checked independently: implementing even one listed trait
/// causes a compile error. Accepts `Type: Trait, OtherTrait, ...`, with an
/// optional trailing comma. Generic arguments, associated type bindings,
/// higher-ranked lifetimes, and unsized target types are supported.
/// At least one trait is required; bounds with `?` are rejected.
///
/// Assertions are anonymous const items, usable at module scope or inside a
/// block for concrete types. They generate no runtime work and use only `core`.
///
/// # Examples
/// ```
/// use helper::assert_not_impl_any;
///
/// assert_not_impl_any!(std::rc::Rc<u8>: Send, Sync);
/// assert_not_impl_any!(str: Sized, Copy,);
/// ```
///
/// Implementing just one of the listed traits is sufficient to fail:
/// ```compile_fail
/// #[derive(Clone)]
/// struct Value;
/// helper::assert_not_impl_any!(Value: Copy, Clone);
/// ```
/// ```compile_fail
/// helper::assert_not_impl_any!(u8: Copy);
/// ```
/// A nonempty trait list is required:
/// ```compile_fail
/// helper::assert_not_impl_any!(u8:);
/// ```
#[proc_macro]
pub fn assert_not_impl_any(items: TokenStream) -> TokenStream {
    assertions::assert_not_impl_any(items.into()).into()
}

/// Select between two expressions using a boolean condition.
///
/// `either!(condition, left, right)` evaluates the condition once and evaluates
/// only the selected branch, just like `if condition { left } else { right }`.
/// All three arguments are Rust expressions, including expressions using `?`.
/// A trailing comma is allowed.
///
/// # Example
/// ```
/// use helper::either;
///
/// assert_eq!(either!(1 > 2, "1", "2"), "2");
/// assert_eq!(either!(true, std::cmp::max(1, 2), 0,), 2);
/// ```
///
/// Error propagation works in the condition and both branches:
/// ```
/// use helper::either;
///
/// fn select(
///     condition: Result<bool, &'static str>, left: Result<i32, &'static str>,
///     right: Result<i32, &'static str>,
/// ) -> Result<i32, &'static str> {
///     Ok(either!(condition?, left?, right?))
/// }
///
/// assert_eq!(select(Ok(true), Ok(7), Err("unselected")), Ok(7));
/// assert_eq!(select(Ok(false), Ok(7), Err("selected")), Err("selected"));
/// ```
///
/// Exactly three comma-separated expressions are required:
/// ```compile_fail
/// helper::either!();
/// ```
/// ```compile_fail
/// helper::either!(true, 1);
/// ```
/// ```compile_fail
/// helper::either!(true, 1, 2, 3);
/// ```
/// ```compile_fail
/// helper::either!(true, 1 +, 2);
/// ```
/// ```compile_fail
/// helper::either!(true ? 1 : 2);
/// ```
#[proc_macro]
pub fn either(items: TokenStream) -> TokenStream {
    control::either(items.into()).into()
}

/// Clone named variables before creating a `move` closure or `async move`
/// block.
///
/// Each listed variable is cloned exactly once, in list order, using
/// [`Clone::clone`]. The clones are scoped to the returned closure or future;
/// the original bindings remain available. Other variables follow the normal
/// capture rules of the closure or async block. Cloning happens immediately,
/// even if the returned closure or future is never run.
/// Cloned bindings can be mutated inside the closure or future.
///
/// The capture list contains distinct variable names, with an optional trailing
/// comma. The whole invocation also accepts a trailing comma. An empty list is
/// allowed. The expansion uses only `core`.
///
/// # Examples
/// ```
/// use helper::capture;
/// use std::sync::Arc;
///
/// let message = Arc::new(String::from("hello"));
/// let callback = capture!([message], move || message.len());
/// assert_eq!(callback(), 5);
/// assert_eq!(message.as_str(), "hello");
///
/// let task = capture!([message,], async move { message.len() },);
/// assert_eq!(Arc::strong_count(&message), 3);
/// drop(task);
/// ```
///
/// A `move` closure or `async move` block is required:
/// ```compile_fail
/// let callback = helper::capture!([], || 42);
/// ```
/// ```compile_fail
/// let task = helper::capture!([], async { 42 });
/// ```
/// Duplicate captures are rejected:
/// ```compile_fail
/// let value = String::from("hello");
/// let callback = helper::capture!([value, value], move || value);
/// ```
/// Each capture must be a variable name:
/// ```compile_fail
/// let values = [String::from("hello")];
/// let callback = helper::capture!([values[0]], move || 42);
/// ```
#[proc_macro]
pub fn capture(items: TokenStream) -> TokenStream {
    capture::capture(items.into()).into()
}

/// Execute statements when the enclosing scope is exited.
///
/// Cleanup runs once on normal scope exit, early `return`, `?`, `break`,
/// `continue`, or panic unwinding. Multiple defers run in reverse registration
/// order. Cleanup does not run when the process aborts or exits without
/// unwinding.
///
/// The body behaves like a zero-argument closure returning `()`: it borrows
/// values when possible and moves values that it consumes. Borrows last until
/// cleanup runs. A `return` in the body returns from the cleanup closure.
///
/// Invoke this macro as a statement in the scope to clean up. It generates a
/// local drop guard, uses only `core`, and requires no allocation or unsafe
/// code.
///
/// # Example
/// ```
/// use helper::defer;
/// use std::cell::RefCell;
///
/// let events = RefCell::new(Vec::new());
/// {
///     defer! { events.borrow_mut().push("first"); }
///     defer! { events.borrow_mut().push("second"); }
///     events.borrow_mut().push("body");
/// }
/// assert_eq!(*events.borrow(), ["body", "second", "first"]);
/// ```
#[proc_macro]
pub fn defer(items: TokenStream) -> TokenStream {
    control::defer(items.into()).into()
}

/// Remove common indentation from a string or byte string literal at compile
/// time.
///
/// Accepts one ordinary or raw string literal, including byte strings, and an
/// optional trailing comma. The output is a string literal (`&'static str`) or
/// byte string literal (`&'static [u8; N]`), suitable for `const`, `static`,
/// `concat!`, and formatting macros. No runtime work, allocation, unsafe code,
/// or dependency on `std` is generated.
///
/// The first line is excluded when computing the minimum indentation. If it
/// consists of only a line ending (`\n` or `\r\n`), it is removed; otherwise
/// it is preserved unchanged. Remaining lines containing only spaces or tabs
/// are also excluded when computing the minimum. With no remaining content
/// lines, the minimum is zero. Up to that many leading spaces or tabs are
/// removed from each line after the first. Each space or tab counts as one
/// character, without expanding tabs. Other whitespace, relative indentation,
/// line endings, and trailing whitespace are preserved.
///
/// # Examples
/// ```
/// use helper::indoc;
///
/// const SQL: &str = indoc! {r#"
///     SELECT id, name
///     FROM users
///     WHERE active = true
/// "#};
/// assert_eq!(SQL, "SELECT id, name\nFROM users\nWHERE active = true\n");
///
/// const BYTES: &[u8; 3] = indoc!(b"\n    \xff\0\n",);
/// assert_eq!(BYTES, b"\xff\0\n");
/// ```
///
/// The result can be used as a format string, with explicit format arguments:
/// ```
/// let message = format!(helper::indoc!("\n    Hello, {name}!\n"), name = "Rust");
/// assert_eq!(message, "Hello, Rust!\n");
/// ```
/// Format arguments must be passed explicitly because the format string comes
/// from a macro.
///
/// Exactly one string or byte string literal is required:
/// ```compile_fail
/// let _ = helper::indoc!();
/// ```
/// ```compile_fail
/// let _ = helper::indoc!("first", "second");
/// ```
/// ```compile_fail
/// let _ = helper::indoc!(42);
/// ```
/// ```compile_fail
/// let _ = helper::indoc!('x');
/// ```
/// ```compile_fail
/// let _ = helper::indoc!(concat!("first", "second"));
/// ```
#[proc_macro]
pub fn indoc(items: TokenStream) -> TokenStream {
    strings::indoc(items.into()).into()
}

/// Decode a hexadecimal string literal into a `[u8; N]` at compile time.
///
/// Uppercase and lowercase digits are accepted. ASCII whitespace is ignored,
/// including between the two digits of a byte. Empty input produces `[u8; 0]`.
/// Any other character or an odd number of digits causes a compile error.
/// One string literal is required; a trailing comma is allowed.
///
/// The expansion works in `const` and `static` initializers, uses only `core`,
/// and performs no runtime decoding or allocation.
///
/// # Example
/// ```
/// use helper::hex;
///
/// const MAGIC: [u8; 4] = hex!("DE AD be ef");
/// assert_eq!(MAGIC, [0xde, 0xad, 0xbe, 0xef]);
/// assert_eq!(hex!(r#"00 ff"#,), [0, 255]);
/// ```
///
/// Invalid characters and incomplete bytes are rejected:
/// ```compile_fail
/// let _ = helper::hex!("0g");
/// ```
/// ```compile_fail
/// let _ = helper::hex!("abc");
/// ```
/// The input must be one string literal:
/// ```compile_fail
/// let _ = helper::hex!(b"deadbeef");
/// ```
/// ```compile_fail
/// let _ = helper::hex!("de", "ad");
/// ```
/// ```compile_fail
/// let _ = helper::hex!();
/// ```
#[proc_macro]
pub fn hex(items: TokenStream) -> TokenStream {
    strings::hex(items.into()).into()
}

/// Concatenate byte literals, byte string literals, and byte arrays at compile
/// time, producing a `&'static [u8; N]`.
///
/// Array elements must be byte literals or integer literals in `0..=255`, with
/// no suffix or the `u8` suffix. Repeated arrays such as `[b'!'; 3]` are also
/// supported; the count must be a nonnegative integer literal with no suffix or
/// the `usize` suffix. Raw byte strings, empty input, and trailing commas are
/// allowed. The expansion is a byte string literal and works without `std` or
/// an allocator, including in `const` and `static` initializers.
///
/// # Example
/// ```
/// use helper::concat_bytes;
///
/// const HEADER: &[u8; 4] = concat_bytes!(b"PK", [3, 4]);
/// assert_eq!(HEADER, b"PK\x03\x04");
/// assert_eq!(
///     concat_bytes!(b'A', br"BC", [68u8, b'E'], [b'!'; 2],),
///     b"ABCDE!!"
/// );
/// ```
///
/// String and character literals must be byte literals:
/// ```compile_fail
/// let _ = helper::concat_bytes!("hello");
/// ```
/// ```compile_fail
/// let _ = helper::concat_bytes!('a');
/// ```
/// Array elements must fit in a byte and have the correct type:
/// ```compile_fail
/// let _ = helper::concat_bytes!([256]);
/// ```
/// ```compile_fail
/// let _ = helper::concat_bytes!([1u16]);
/// ```
/// Integer literals belong inside arrays:
/// ```compile_fail
/// let _ = helper::concat_bytes!(65);
/// ```
#[proc_macro]
pub fn concat_bytes(items: TokenStream) -> TokenStream {
    strings::concat_bytes(items.into()).into()
}

/// Include a UTF-8 file as a `&'static str`, trimming leading and trailing
/// ASCII whitespace at compile time.
///
/// Whitespace means space, tab, line feed, vertical tab, form feed, and
/// carriage return. Interior whitespace and non-ASCII whitespace are preserved.
/// Empty files and files containing only ASCII whitespace produce `""`.
///
/// Paths are relative to the calling file, just like [`include_str!`]. The path
/// may be a string literal or a macro expanding to one, such as `concat!` or
/// `env!`; a trailing comma is allowed. File changes trigger recompilation.
///
/// Copying and trimming happen at compile time. The expansion uses only `core`,
/// allocates nothing at runtime, contains no unsafe code, and works in `const`
/// and `static` initializers on Rust 1.72 and later.
///
/// # Example
/// ```
/// use helper::include_str_trimmed;
///
/// const MANIFEST: &str =
///     include_str_trimmed!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml",));
/// assert!(MANIFEST.starts_with("[package]"));
/// assert!(!MANIFEST.ends_with('\n'));
/// ```
///
/// Files must contain valid UTF-8:
/// ```compile_fail
/// let _ = helper::include_str_trimmed!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/fixtures/include_c_str/non_utf8.bin",
/// ));
/// ```
/// Exactly one path is required:
/// ```compile_fail
/// let _ = helper::include_str_trimmed!();
/// ```
/// ```compile_fail
/// let _ = helper::include_str_trimmed!("first.txt", "second.txt");
/// ```
#[proc_macro]
pub fn include_str_trimmed(items: TokenStream) -> TokenStream {
    strings::include_str_trimmed(items.into()).into()
}

/// Include a file as a [`&'static CStr`](core::ffi::CStr).
///
/// The path is resolved relative to the file containing the invocation, just
/// like [`include_bytes!`]. It must be a string literal or a macro expanding to
/// one, such as `concat!(env!("CARGO_MANIFEST_DIR"), "/file.txt")`. A trailing
/// comma is allowed.
///
/// A missing NUL terminator is appended; an existing single terminator is
/// preserved. Any other NUL byte causes a compile error, even when the macro is
/// used outside a `const` or `static`. Empty files and non-UTF-8 bytes are
/// allowed. File changes are tracked by the compiler as with `include_bytes!`.
///
/// The result can be used in `const` and `static` initializers. The expansion
/// uses only `core`, performs all copying and validation at compile time, and
/// requires no allocation or unsafe code. Requires Rust 1.72 or later.
///
/// # Example
/// ```
/// use helper::include_c_str;
///
/// const MANIFEST: &core::ffi::CStr =
///     include_c_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml",));
/// assert_eq!(MANIFEST.to_bytes(), include_bytes!("../Cargo.toml"));
/// ```
///
/// Interior NUL bytes are rejected during compilation:
/// ```compile_fail
/// let _ = helper::include_c_str!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/fixtures/include_c_str/interior_nul.bin",
/// ));
/// ```
/// Leading NUL bytes and repeated terminators are also rejected:
/// ```compile_fail
/// let _ = helper::include_c_str!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/fixtures/include_c_str/leading_nul.bin",
/// ));
/// ```
/// ```compile_fail
/// let _ = helper::include_c_str!(concat!(
///     env!("CARGO_MANIFEST_DIR"),
///     "/tests/fixtures/include_c_str/repeated_nul.bin",
/// ));
/// ```
///
/// Exactly one path is required:
/// ```compile_fail
/// let _ = helper::include_c_str!();
/// ```
/// ```compile_fail
/// let _ = helper::include_c_str!("first.txt", "second.txt");
/// ```
/// ```compile_fail
/// let _ = helper::include_c_str!(123);
/// ```
#[proc_macro]
pub fn include_c_str(items: TokenStream) -> TokenStream {
    strings::include_c_str(items.into()).into()
}

/// Create [`std::collections::HashMap`] from list of key-value pairs.
///
/// # Example
/// ```
/// use helper::hmap;
///
/// let m = hmap! {
///     "a": 1,
///     "b": 2,
/// };
/// assert_eq!(m["a"], 1);
/// assert_eq!(m["b"], 2);
/// ```
///
/// Keys and values are Rust expressions, including paths, generic arguments,
/// closures, and the `?` operator. Each key and value is evaluated once, in
/// input order. A later duplicate key replaces the earlier value.
/// ```
/// let limits = helper::hmap! {u8::MAX: Result::<u16, u32>::Ok(u16::MAX)};
/// assert_eq!(limits[&u8::MAX], Ok(u16::MAX));
/// ```
///
/// Every entry requires both a key and a value:
/// ```compile_fail
/// let _ = helper::hmap! {"a": 1, "b"};
/// ```
/// ```compile_fail
/// let _ = helper::hmap! {"a":};
/// ```
/// ```compile_fail
/// let _ = helper::hmap! {"a": 1 "b": 2};
/// ```
#[proc_macro]
pub fn hmap(items: TokenStream) -> TokenStream {
    collections::hmap(items.into()).into()
}

/// Create a [`std::collections::HashMap`], converting every key and value with
/// [`Into::into`].
///
/// The target types must be determined by the surrounding context, typically a
/// type annotation. Each key is evaluated and converted before its value, and
/// entries are processed once in input order. A later duplicate converted key
/// replaces the earlier value. Empty maps and trailing commas are supported.
///
/// # Example
/// ```
/// use helper::hmap_into;
/// use std::collections::HashMap;
///
/// let headers: HashMap<String, String> = hmap_into! {
///     "Accept": "application/json",
///     "User-Agent": String::from("helper"),
/// };
/// assert_eq!(headers["Accept"], "application/json");
/// ```
///
/// Every entry requires a key and a value:
/// ```compile_fail
/// let _: std::collections::HashMap<String, String> = helper::hmap_into! {"key":};
/// ```
#[proc_macro]
pub fn hmap_into(items: TokenStream) -> TokenStream {
    collections::hmap_into(items.into()).into()
}

/// Create [`std::collections::BTreeMap`] from list of key-value pairs.
///
/// # Example
/// ```
/// use helper::btmap;
///
/// let m = btmap! {
///     "a": 1,
///     "b": 2,
/// };
/// assert_eq!(m["a"], 1);
/// assert_eq!(m["b"], 2);
/// ```
///
/// Keys and values are Rust expressions. Each key and value is evaluated once,
/// in input order. A later duplicate key replaces the earlier value.
///
/// Every entry requires both a key and a value:
/// ```compile_fail
/// let _ = helper::btmap! {"a": 1, "b"};
/// ```
/// ```compile_fail
/// let _ = helper::btmap! {"a":};
/// ```
/// ```compile_fail
/// let _ = helper::btmap! {"a": 1,, "b": 2};
/// ```
#[proc_macro]
pub fn btmap(items: TokenStream) -> TokenStream {
    collections::btmap(items.into()).into()
}

/// Create a [`std::collections::BTreeMap`], converting each key and value with
/// [`Into::into`].
///
/// Target types are determined by the surrounding context, typically a type
/// annotation. Keys and values are evaluated and converted once in input order,
/// with each key converted before its value. A later duplicate converted key
/// replaces the earlier value. Empty maps and trailing commas are supported.
///
/// # Example
/// ```
/// use helper::btmap_into;
/// use std::collections::BTreeMap;
///
/// let values: BTreeMap<String, String> = btmap_into! {
///     "z": "last",
///     String::from("a"): String::from("first"),
/// };
/// assert_eq!(
///     values.keys().map(String::as_str).collect::<Vec<_>>(),
///     ["a", "z"]
/// );
/// ```
///
/// Every entry requires a key and a value:
/// ```compile_fail
/// let _: std::collections::BTreeMap<String, String> = helper::btmap_into! {"key":};
/// ```
#[proc_macro]
pub fn btmap_into(items: TokenStream) -> TokenStream {
    collections::btmap_into(items.into()).into()
}

/// Create a [`std::collections::HashSet`] from a list of elements.
///
/// # Example
/// ```
/// use helper::hset;
///
/// let set = hset! {"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// ```
///
/// Elements are Rust expressions and are evaluated once, in input order.
/// A trailing comma is allowed; empty elements are rejected:
/// ```compile_fail
/// let _ = helper::hset! {1,, 2};
/// ```
#[proc_macro]
pub fn hset(items: TokenStream) -> TokenStream {
    collections::hset(items.into()).into()
}

/// Create a [`std::collections::HashSet`], converting every element with
/// [`Into::into`].
///
/// The target element type must be determined by the surrounding context,
/// typically a type annotation. Elements are evaluated and converted once in
/// input order; equal converted values are deduplicated. Empty sets and
/// trailing commas are supported.
///
/// # Example
/// ```
/// use helper::hset_into;
/// use std::collections::HashSet;
///
/// let names: HashSet<String> = hset_into! {"alice", String::from("bob"), "alice"};
/// assert_eq!(names.len(), 2);
/// assert!(names.contains("alice"));
/// ```
///
/// Empty elements are rejected:
/// ```compile_fail
/// let _: std::collections::HashSet<String> = helper::hset_into! {"alice",, "bob"};
/// ```
#[proc_macro]
pub fn hset_into(items: TokenStream) -> TokenStream {
    collections::hset_into(items.into()).into()
}

/// Create a [`std::collections::BTreeSet`] from a list of elements.
///
/// # Example
/// ```
/// use helper::btset;
///
/// let set = btset! {"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// ```
///
/// Elements are Rust expressions and are evaluated once, in input order.
/// A trailing comma is allowed; empty elements are rejected:
/// ```compile_fail
/// let _ = helper::btset! {,1};
/// ```
#[proc_macro]
pub fn btset(items: TokenStream) -> TokenStream {
    collections::btset(items.into()).into()
}

/// Create a [`std::collections::BTreeSet`], converting each element with
/// [`Into::into`].
///
/// The target element type is determined by the surrounding context, typically
/// a type annotation. Elements are evaluated and converted once in input order,
/// and equal converted values are deduplicated. Empty sets and trailing commas
/// are supported.
///
/// # Example
/// ```
/// use helper::btset_into;
/// use std::collections::BTreeSet;
///
/// let names: BTreeSet<String> = btset_into! {"bob", String::from("alice"), "bob"};
/// assert_eq!(names.into_iter().collect::<Vec<_>>(), ["alice", "bob"]);
/// ```
///
/// Empty elements are rejected:
/// ```compile_fail
/// let _: std::collections::BTreeSet<String> = helper::btset_into! {"alice",, "bob"};
/// ```
#[proc_macro]
pub fn btset_into(items: TokenStream) -> TokenStream {
    collections::btset_into(items.into()).into()
}

// Type conversion macros for basic data types

/// Convert expression to `u8` type.
///
/// # Example
/// ```
/// use helper::u8;
///
/// let x = 42i32;
/// let y = u8!(x);
/// assert_eq!(y, 42u8);
/// ```
///
/// Exactly one Rust expression is required:
/// ```compile_fail
/// let _ = helper::u8!();
/// ```
/// ```compile_fail
/// let _ = helper::u8!(1, 2);
/// ```
#[proc_macro]
pub fn u8(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "u8").into()
}

/// Convert expression to `u16` type.
///
/// # Example
/// ```
/// use helper::u16;
///
/// let x = 42i32;
/// let y = u16!(x);
/// assert_eq!(y, 42u16);
/// ```
#[proc_macro]
pub fn u16(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "u16").into()
}

/// Convert expression to `u32` type.
///
/// # Example
/// ```
/// use helper::u32;
///
/// let x = 42i64;
/// let y = u32!(x);
/// assert_eq!(y, 42u32);
/// ```
#[proc_macro]
pub fn u32(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "u32").into()
}

/// Convert expression to `u64` type.
///
/// # Example
/// ```
/// use helper::u64;
///
/// let x = 42i32;
/// let y = u64!(x);
/// assert_eq!(y, 42u64);
/// ```
#[proc_macro]
pub fn u64(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "u64").into()
}

/// Convert expression to `u128` type.
///
/// # Example
/// ```
/// use helper::u128;
///
/// let x = 42i32;
/// let y = u128!(x);
/// assert_eq!(y, 42u128);
/// ```
#[proc_macro]
pub fn u128(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "u128").into()
}

/// Convert expression to `usize` type.
///
/// # Example
/// ```
/// use helper::usize;
///
/// let x = 42i32;
/// let y = usize!(x);
/// assert_eq!(y, 42usize);
/// ```
#[proc_macro]
pub fn usize(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "usize").into()
}

/// Convert expression to `i8` type.
///
/// # Example
/// ```
/// use helper::i8;
///
/// let x = 42i32;
/// let y = i8!(x);
/// assert_eq!(y, 42i8);
/// ```
#[proc_macro]
pub fn i8(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "i8").into()
}

/// Convert expression to `i16` type.
///
/// # Example
/// ```
/// use helper::i16;
///
/// let x = 42i32;
/// let y = i16!(x);
/// assert_eq!(y, 42i16);
/// ```
#[proc_macro]
pub fn i16(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "i16").into()
}

/// Convert expression to `i32` type.
///
/// # Example
/// ```
/// use helper::i32;
///
/// let x = 42i64;
/// let y = i32!(x);
/// assert_eq!(y, 42i32);
/// ```
#[proc_macro]
pub fn i32(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "i32").into()
}

/// Convert expression to `i64` type.
///
/// # Example
/// ```
/// use helper::i64;
///
/// let x = 42i32;
/// let y = i64!(x);
/// assert_eq!(y, 42i64);
/// ```
#[proc_macro]
pub fn i64(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "i64").into()
}

/// Convert expression to `i128` type.
///
/// # Example
/// ```
/// use helper::i128;
///
/// let x = 42i32;
/// let y = i128!(x);
/// assert_eq!(y, 42i128);
/// ```
#[proc_macro]
pub fn i128(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "i128").into()
}

/// Convert expression to `isize` type.
///
/// # Example
/// ```
/// use helper::isize;
///
/// let x = 42i32;
/// let y = isize!(x);
/// assert_eq!(y, 42isize);
/// ```
#[proc_macro]
pub fn isize(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "isize").into()
}

/// Convert expression to `f32` type.
///
/// # Example
/// ```
/// use helper::f32;
///
/// let x = 13.25f64;
/// let y = f32!(x);
/// assert_eq!(y, 13.25f32);
/// ```
#[proc_macro]
pub fn f32(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "f32").into()
}

/// Convert expression to `f64` type.
///
/// # Example
/// ```
/// use helper::f64;
///
/// let x = 13.25f32;
/// let y = f64!(x);
/// assert_eq!(y, 13.25f64);
/// ```
#[proc_macro]
pub fn f64(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "f64").into()
}

/// Convert expression to `char` type.
///
/// # Example
/// ```
/// use helper::char;
///
/// let x = 65u8;
/// let y = char!(x);
/// assert_eq!(y, 'A');
/// ```
#[proc_macro]
pub fn char(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "char").into()
}

/// Convert expression to `bool` type.
///
/// # Example
/// ```
/// use helper::bool;
///
/// let x = 1i32;
/// let y = bool!(x != 0);
/// assert_eq!(y, true);
/// ```
#[proc_macro]
pub fn bool(items: TokenStream) -> TokenStream {
    types::type_cast(items.into(), "bool").into()
}

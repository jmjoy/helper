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

pub(crate) mod collections;
pub(crate) mod control;
pub(crate) mod strings;
pub(crate) mod types;
pub(crate) mod utils;

use proc_macro::TokenStream;

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

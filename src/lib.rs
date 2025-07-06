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
pub(crate) mod types;
pub(crate) mod utils;

use proc_macro::TokenStream;

/// Ternary operator in many C-like languages.
///
/// `either!(expr ? left : right)` expand to `if expr { left } else { right }`.
///
/// # Example
/// ```
/// use helper::either;
///
/// assert_eq!(either!(1 > 2 ? "1" : "2"), "2");
/// ```
#[proc_macro]
pub fn either(items: TokenStream) -> TokenStream {
    control::either(items.into()).into()
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
/// let x = 3.14f64;
/// let y = f32!(x);
/// assert_eq!(y, 3.14f32);
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
/// let x = 3.14f32;
/// let y = f64!(x);
/// assert!((y - 3.14f64).abs() < 0.0001);
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

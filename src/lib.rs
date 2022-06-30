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

/// [`std::option::Option`] relative operations.
///
/// # Operations
///
/// ## unwrap .. or ..
///
/// `option!(unwrap .. or ..)` expand to `match .. { Some(x) => x, None => ..
/// }`.
///
/// ### Example
/// ```
/// use helper::option;
///
/// // Like Option::unwrap_or.
/// assert_eq!(option!(unwrap Some(true) or false), true);
/// assert_eq!(option!(unwrap None or false), false);
///
/// // Return in advance.
/// fn foo() -> bool {
///     let x = option!(unwrap Some("foo") or return false);
///     x == "foo"
/// }
/// assert!(foo());
///
/// // Break in advance.
/// let x = loop {
///     option!(unwrap None or break 1i32);
/// };
/// assert_eq!(x, 1);
/// ```
#[proc_macro]
pub fn option(items: TokenStream) -> TokenStream {
    control::option(items.into()).into()
}

/// Unwrap [`std::option::Option`], if none, return the alternative value.
///
/// `try_option!(.. ? ..)` expand to `match .. { Some(x) => x, None => return ..
/// }`.
///
/// ### Example
/// ```
/// use helper::try_option;
///
/// fn foo() -> bool {
///     let x = try_option!(Some("foo") ? false);
///     x == "foo"
/// }
/// assert!(foo());
/// ```
#[proc_macro]
pub fn try_option(items: TokenStream) -> TokenStream {
    control::try_option(items.into()).into()
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

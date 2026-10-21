// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Expr, Result, Token,
};

use crate::utils::parse_expr;

pub(crate) fn hmap(items: TokenStream) -> TokenStream {
    map(items, quote!(::std::collections::HashMap), true)
}

pub(crate) fn btmap(items: TokenStream) -> TokenStream {
    map(items, quote!(::std::collections::BTreeMap), false)
}

struct MapEntry {
    key: Expr,
    value: Expr,
}

impl Parse for MapEntry {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let key = parse_expr(input)?;
        input.parse::<Token![:]>()?;
        let value = parse_expr(input)?;
        Ok(Self { key, value })
    }
}

fn map(items: TokenStream, qualifier: TokenStream, has_capacity: bool) -> TokenStream {
    let entries = match Punctuated::<MapEntry, Token![,]>::parse_terminated.parse2(items) {
        Ok(entries) => entries,
        Err(error) => return error.into_compile_error(),
    };
    let constructor = constructor(&qualifier, entries.len(), has_capacity);

    if entries.is_empty() {
        return constructor;
    }

    // Keep the generated binding separate from identifiers in user expressions.
    let map = Ident::new("map", Span::mixed_site());
    let inserts = entries.iter().map(|MapEntry { key, value }| {
        quote! { let _ = #map.insert(#key, #value); }
    });

    quote! {{
        let mut #map = #constructor;
        #(#inserts)*
        #map
    }}
}

pub(crate) fn hset(items: TokenStream) -> TokenStream {
    seq(items, quote!(::std::collections::HashSet), true)
}

pub(crate) fn btset(items: TokenStream) -> TokenStream {
    seq(items, quote!(::std::collections::BTreeSet), false)
}

fn seq(items: TokenStream, qualifier: TokenStream, has_capacity: bool) -> TokenStream {
    let parser = |input: ParseStream<'_>| {
        Punctuated::<Expr, Token![,]>::parse_terminated_with(input, parse_expr)
    };
    let elements = match parser.parse2(items) {
        Ok(elements) => elements,
        Err(error) => return error.into_compile_error(),
    };
    let constructor = constructor(&qualifier, elements.len(), has_capacity);

    if elements.is_empty() {
        return constructor;
    }

    let seq = Ident::new("seq", Span::mixed_site());
    let elements = elements.iter();

    quote! {{
        let mut #seq = #constructor;
        #(let _ = #seq.insert(#elements);)*
        #seq
    }}
}

fn constructor(qualifier: &TokenStream, len: usize, has_capacity: bool) -> TokenStream {
    if has_capacity && len > 0 {
        quote!(#qualifier::with_capacity(#len))
    } else {
        quote!(#qualifier::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{pm_test_left, pm_test_right};

    use super::*;

    #[test]
    fn test_hmap() {
        assert_eq!(
            pm_test_left(hmap, ""),
            pm_test_right("::std::collections::HashMap::new()"),
        );

        assert_eq!(
            pm_test_left(
                hmap,
                r#"
                "key": "value",
                "key1": "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::HashMap::with_capacity(2usize);
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                hmap,
                r#"
                "key": "value",
                "key1": "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::HashMap::with_capacity(2usize);
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );
    }

    #[test]
    fn test_btmap() {
        assert_eq!(
            pm_test_left(btmap, ""),
            pm_test_right("::std::collections::BTreeMap::new()"),
        );

        assert_eq!(
            pm_test_left(
                btmap,
                r#"
                "key": "value",
                "key1": "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::BTreeMap::new();
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                btmap,
                r#"
                "key": "value",
                "key1": "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::BTreeMap::new();
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );
    }

    #[test]
    fn test_hset() {
        assert_eq!(
            pm_test_left(hset, ""),
            pm_test_right("::std::collections::HashSet::new()"),
        );

        assert_eq!(
            pm_test_left(
                hset,
                r#"
                "value",
                "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::HashSet::with_capacity(2usize);
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                hset,
                r#"
                "value", "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::HashSet::with_capacity(2usize);
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );
    }

    #[test]
    fn test_btset() {
        assert_eq!(
            pm_test_left(btset, ""),
            pm_test_right("::std::collections::BTreeSet::new()"),
        );

        assert_eq!(
            pm_test_left(
                btset,
                r#"
                "value",
                "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::BTreeSet::new();
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                btset,
                r#"
                "value", "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::BTreeSet::new();
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );
    }
}

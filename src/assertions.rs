// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token, TraitBound, TraitBoundModifier, Type,
};

use crate::utils::{parse_trait_bound, parse_type};

struct Assertion {
    target: Type,
    bounds: Punctuated<TraitBound, Token![,]>,
}

impl Parse for Assertion {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let target = parse_type(input)?;
        input.parse::<Token![:]>()?;
        let bounds =
            Punctuated::<TraitBound, Token![,]>::parse_terminated_with(input, parse_trait_bound)?;
        if bounds.is_empty() {
            return Err(input.error("expected at least one trait bound"));
        }
        for bound in &bounds {
            if !matches!(bound.modifier, TraitBoundModifier::None) {
                return Err(syn::Error::new_spanned(
                    bound,
                    "expected a trait bound without `?`",
                ));
            }
        }
        Ok(Self { target, bounds })
    }
}

pub(crate) fn assert_impl_all(items: TokenStream) -> TokenStream {
    let check = private_ident("__helper_assert_impl_all", &items);
    let target_param = private_ident("__HelperAssertType", &items);
    let Assertion { target, bounds } = match syn::parse2(items) {
        Ok(assertion) => assertion,
        Err(error) => return error.into_compile_error(),
    };
    let bounds = bounds.iter();

    quote! {
        const _: fn() = || {
            fn #check<#target_param: ?Sized + #(#bounds)+*>() {}
            let _ = #check::<#target>;
        };
    }
}

pub(crate) fn assert_not_impl_any(items: TokenStream) -> TokenStream {
    let [ambiguous, implemented, target_param, marker] = [
        "__HelperAmbiguousIfImplemented",
        "__HelperImplemented",
        "__HelperAssertType",
        "__helper_marker",
    ]
    .map(|name| private_ident(name, &items));
    let Assertion { target, bounds } = match syn::parse2(items) {
        Ok(assertion) => assertion,
        Err(error) => return error.into_compile_error(),
    };
    let checks = bounds.iter().map(|bound| {
        quote! {
            {
                trait #ambiguous<A> {
                    fn #marker() {}
                }

                impl<#target_param: ?Sized> #ambiguous<()> for #target_param {}
                struct #implemented;
                impl<#target_param: ?Sized + #bound> #ambiguous<#implemented> for #target_param {}

                // Implementing this bound makes the trait parameter ambiguous.
                let _ = <#target as #ambiguous<_>>::#marker;
            }
        }
    });

    quote! {
        const _: fn() = || {
            #(#checks)*
        };
    }
}

fn private_ident(base: &str, items: &TokenStream) -> Ident {
    fn contains(items: &TokenStream, name: &str) -> bool {
        items.clone().into_iter().any(|token| match token {
            TokenTree::Ident(ident) => ident.to_string().trim_start_matches("r#") == name,
            TokenTree::Group(group) => contains(&group.stream(), name),
            _ => false,
        })
    }

    let mut name = base.to_owned();
    while contains(items, &name) {
        name.push('_');
    }
    Ident::new(&name, Span::mixed_site())
}

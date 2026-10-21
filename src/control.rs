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
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    Expr, Token,
};

use crate::utils::{parse_expr, parse_statements};

pub(crate) fn either(items: TokenStream) -> TokenStream {
    let parser = |input: ParseStream<'_>| {
        Punctuated::<Expr, Token![,]>::parse_terminated_with(input, parse_expr)
    };
    let expressions = match parser.parse2(items) {
        Ok(expressions) => expressions,
        Err(error) => return error.into_compile_error(),
    };

    if expressions.len() != 3 {
        return syn::Error::new_spanned(
            &expressions,
            "expected exactly three expressions: either!(condition, true_expr, false_expr)",
        )
        .into_compile_error();
    }

    let condition = &expressions[0];
    let left = &expressions[1];
    let right = &expressions[2];

    quote! {
        if (#condition) { #left } else { #right }
    }
}

pub(crate) fn defer(items: TokenStream) -> TokenStream {
    let statements = match parse_statements.parse2(items) {
        Ok(statements) => statements,
        Err(error) => return error.into_compile_error(),
    };
    let guard = Ident::new("__helper_defer_guard", Span::mixed_site());
    let guard_type = Ident::new("__HelperDeferGuard", Span::mixed_site());

    // The binding must live in the caller's scope, outside the initializer
    // block.
    quote! {
        let #guard = {
            struct #guard_type<F: ::core::ops::FnOnce()>(::core::option::Option<F>);

            impl<F: ::core::ops::FnOnce()> ::core::ops::Drop for #guard_type<F> {
                fn drop(&mut self) {
                    if let ::core::option::Option::Some(callback) = self.0.take() {
                        callback();
                    }
                }
            }

            #guard_type
        }(::core::option::Option::Some(|| { #(#statements)* }));
    }
}

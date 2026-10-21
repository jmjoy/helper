// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    punctuated::Punctuated,
    Expr, Token,
};

use crate::utils::parse_expr;

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

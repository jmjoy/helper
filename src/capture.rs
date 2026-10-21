use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Result, Token,
};

use crate::utils::{parse_expr, unparenthesized_expr};

struct Capture {
    names: Punctuated<Ident, Token![,]>,
    expression: Expr,
}

impl Parse for Capture {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let names;
        bracketed!(names in input);
        let names = Punctuated::<Ident, Token![,]>::parse_terminated(&names)?;
        input.parse::<Token![,]>()?;
        let expression = parse_expr(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut seen = HashSet::new();
        for name in &names {
            if !seen.insert(name.to_string().trim_start_matches("r#").to_owned()) {
                return Err(syn::Error::new(name.span(), "duplicate capture"));
            }
        }

        let is_move = match unparenthesized_expr(&expression) {
            Expr::Closure(closure) => closure.capture.is_some(),
            Expr::Async(block) => block.capture.is_some(),
            _ => false,
        };
        if !is_move {
            return Err(syn::Error::new_spanned(
                &expression,
                "expected a `move` closure or an `async move` block",
            ));
        }

        Ok(Self { names, expression })
    }
}

pub(crate) fn capture(items: TokenStream) -> TokenStream {
    let Capture { names, expression } = match syn::parse2(items) {
        Ok(capture) => capture,
        Err(error) => return error.into_compile_error(),
    };
    let clones = names.iter().map(|name| {
        quote! {
            #[allow(unused_mut)]
            let mut #name = ::core::clone::Clone::clone(&#name);
        }
    });

    quote! {{
        #(#clones)*
        #expression
    }}
}

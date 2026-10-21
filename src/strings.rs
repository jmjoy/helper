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
use syn::{parse::Parser, punctuated::Punctuated, Expr, Lit, LitByteStr, LitStr, Token};

use crate::utils::{parse_expr, unparenthesized_expr};

pub(crate) fn indoc(items: TokenStream) -> TokenStream {
    let parser = |input: syn::parse::ParseStream<'_>| {
        let expression = parse_expr(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(expression)
    };
    let expression = match parser.parse2(items) {
        Ok(expression) => expression,
        Err(error) => return error.into_compile_error(),
    };
    if let Expr::Lit(literal) = unparenthesized_expr(&expression) {
        match &literal.lit {
            Lit::Str(text) => {
                let bytes = unindent(text.value().as_bytes());
                let value = match ::core::str::from_utf8(&bytes) {
                    Ok(value) => value,
                    Err(_) => {
                        return syn::Error::new(text.span(), "indoc!: invalid UTF-8")
                            .into_compile_error()
                    }
                };
                let output = LitStr::new(value, text.span());
                return quote!(#output);
            }
            Lit::ByteStr(text) => {
                let output = LitByteStr::new(&unindent(&text.value()), text.span());
                return quote!(#output);
            }
            _ => {}
        }
    }
    syn::Error::new_spanned(
        expression,
        "expected a string literal or byte string literal",
    )
    .into_compile_error()
}

fn unindent(input: &[u8]) -> Vec<u8> {
    let mut lines = input.split_inclusive(|byte| *byte == b'\n');
    let first = lines.next().unwrap_or_default();
    let margin = lines
        .clone()
        .filter_map(|line| {
            let count = indentation(line);
            match &line[count..] {
                b"" | b"\n" | b"\r\n" => None,
                _ => Some(count),
            }
        })
        .min()
        .unwrap_or(0);

    let mut output = Vec::with_capacity(input.len());
    if first != b"\n" && first != b"\r\n" {
        output.extend_from_slice(first);
    }
    for line in lines {
        let remove = margin.min(indentation(line));
        output.extend_from_slice(&line[remove..]);
    }
    output
}

fn indentation(line: &[u8]) -> usize {
    line.iter()
        .take_while(|byte| matches!(byte, b' ' | b'\t'))
        .count()
}

pub(crate) fn concat_bytes(items: TokenStream) -> TokenStream {
    let parser = |input: syn::parse::ParseStream<'_>| {
        Punctuated::<Expr, Token![,]>::parse_terminated_with(input, parse_expr)
    };
    let expressions = match parser.parse2(items) {
        Ok(expressions) => expressions,
        Err(error) => return error.into_compile_error(),
    };
    let mut bytes = Vec::new();
    for expression in &expressions {
        let result = match unparenthesized_expr(expression) {
            Expr::Lit(literal) => match &literal.lit {
                Lit::Byte(byte) => {
                    bytes.push(byte.value());
                    Ok(())
                }
                Lit::ByteStr(string) => {
                    bytes.extend(string.value());
                    Ok(())
                }
                _ => Err(syn::Error::new_spanned(
                    expression,
                    "expected a byte literal, byte string literal, or byte array",
                )),
            },
            Expr::Array(array) => array.elems.iter().try_for_each(|element| {
                bytes.push(parse_byte(element)?);
                Ok(())
            }),
            Expr::Repeat(repeat) => (|| {
                let byte = parse_byte(&repeat.expr)?;
                let count = match unparenthesized_expr(&repeat.len) {
                    Expr::Lit(literal) => match &literal.lit {
                        Lit::Int(count) if matches!(count.suffix(), "" | "usize") => {
                            count.base10_parse::<usize>()?
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &repeat.len,
                                "expected an integer literal repeat count",
                            ))
                        }
                    },
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &repeat.len,
                            "expected an integer literal repeat count",
                        ))
                    }
                };
                bytes.try_reserve(count).map_err(|_| {
                    syn::Error::new_spanned(repeat, "concatenated byte string is too large")
                })?;
                bytes.resize(bytes.len() + count, byte);
                Ok(())
            })(),
            _ => Err(syn::Error::new_spanned(
                expression,
                "expected a byte literal, byte string literal, or byte array",
            )),
        };
        if let Err(error) = result {
            return error.into_compile_error();
        }
    }
    let literal = LitByteStr::new(&bytes, Span::call_site());
    quote!(#literal)
}

fn parse_byte(expression: &Expr) -> syn::Result<u8> {
    if let Expr::Lit(literal) = unparenthesized_expr(expression) {
        match &literal.lit {
            Lit::Byte(byte) => return Ok(byte.value()),
            Lit::Int(integer) if matches!(integer.suffix(), "" | "u8") => {
                return integer.base10_parse()
            }
            _ => {}
        }
    }
    Err(syn::Error::new_spanned(
        expression,
        "expected a byte literal or an integer literal in 0..=255",
    ))
}

pub(crate) fn include_str_trimmed(items: TokenStream) -> TokenStream {
    let paths = match Punctuated::<Expr, Token![,]>::parse_terminated.parse2(items) {
        Ok(paths) => paths,
        Err(error) => return error.into_compile_error(),
    };
    if paths.len() != 1 {
        return syn::Error::new_spanned(
            &paths,
            "expected exactly one file path: include_str_trimmed!(\"path/to/file\")",
        )
        .into_compile_error();
    }
    let path = &paths[0];
    let [text, bounds, len, bytes, trimmed, data, start, end, index] = [
        "TEXT", "BOUNDS", "LEN", "BYTES", "TRIMMED", "data", "start", "end", "index",
    ]
    .map(|name| Ident::new(name, Span::mixed_site()));

    quote! {{
        const #text: &::core::primitive::str = ::core::include_str!(#path);
        const #bounds: (::core::primitive::usize, ::core::primitive::usize) = {
            let #data = #text.as_bytes();
            let mut #start = 0;
            while #start < #data.len()
                && (#data[#start] == b' ' || (#data[#start] >= b'\t' && #data[#start] <= b'\r'))
            {
                #start += 1;
            }
            let mut #end = #data.len();
            while #end > #start
                && (#data[#end - 1] == b' ' || (#data[#end - 1] >= b'\t' && #data[#end - 1] <= b'\r'))
            {
                #end -= 1;
            }
            (#start, #end)
        };
        const #len: ::core::primitive::usize = #bounds.1 - #bounds.0;
        const #bytes: [::core::primitive::u8; #len] = {
            let mut #data = [0; #len];
            let mut #index = 0;
            while #index < #len {
                #data[#index] = #text.as_bytes()[#bounds.0 + #index];
                #index += 1;
            }
            #data
        };
        const #trimmed: &::core::primitive::str = match ::core::str::from_utf8(&#bytes) {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(_) => ::core::panic!("include_str_trimmed!: invalid UTF-8"),
        };
        #trimmed
    }}
}

pub(crate) fn hex(items: TokenStream) -> TokenStream {
    let parser = |input: syn::parse::ParseStream<'_>| {
        let expression = parse_expr(input)?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(expression)
    };
    let expression = match parser.parse2(items) {
        Ok(expression) => expression,
        Err(error) => return error.into_compile_error(),
    };
    let literal = match unparenthesized_expr(&expression) {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Str(literal) => literal,
            _ => {
                return syn::Error::new_spanned(expression, "expected a string literal")
                    .into_compile_error()
            }
        },
        _ => {
            return syn::Error::new_spanned(expression, "expected a string literal")
                .into_compile_error()
        }
    };

    let text = literal.value();
    let mut bytes = Vec::with_capacity(text.len() / 2);
    let mut high = None;
    for character in text.chars() {
        if character.is_ascii_whitespace() || character == '\x0b' {
            continue;
        }
        let digit = match character.to_digit(16) {
            Some(digit) => digit as u8,
            None => {
                return syn::Error::new(
                    literal.span(),
                    format!("invalid hexadecimal digit `{character}`"),
                )
                .into_compile_error()
            }
        };
        if let Some(first) = high.take() {
            bytes.push((first << 4) | digit);
        } else {
            high = Some(digit);
        }
    }
    if high.is_some() {
        return syn::Error::new(
            literal.span(),
            "expected an even number of hexadecimal digits",
        )
        .into_compile_error();
    }

    let len = bytes.len();
    let bytes_name = Ident::new("HEX_BYTES", Span::mixed_site());
    quote! {{
        const #bytes_name: [::core::primitive::u8; #len] = [#(#bytes),*];
        #bytes_name
    }}
}

pub(crate) fn include_c_str(items: TokenStream) -> TokenStream {
    let paths = match Punctuated::<Expr, Token![,]>::parse_terminated.parse2(items) {
        Ok(paths) => paths,
        Err(error) => return error.into_compile_error(),
    };
    if paths.len() != 1 {
        return syn::Error::new_spanned(
            &paths,
            "expected exactly one file path: include_c_str!(\"path/to/file\")",
        )
        .into_compile_error();
    }
    let path = &paths[0];
    let [file, len, bytes, c_str, data, index, value] =
        ["FILE", "LEN", "BYTES", "C_STR", "data", "index", "value"]
            .map(|name| Ident::new(name, Span::mixed_site()));

    quote! {{
        // Let rustc resolve the caller's path and track the included file.
        const #file: &[::core::primitive::u8] = ::core::include_bytes!(#path);
        const #len: ::core::primitive::usize = if !#file.is_empty()
            && #file[#file.len() - 1] == 0
        {
            #file.len()
        } else {
            #file.len() + 1
        };
        const #bytes: [::core::primitive::u8; #len] = {
            let mut #data = [0; #len];
            let mut #index = 0;
            while #index < #file.len() {
                #data[#index] = #file[#index];
                #index += 1;
            }
            #data
        };
        const #c_str: &::core::ffi::CStr =
            match ::core::ffi::CStr::from_bytes_with_nul(&#bytes) {
                ::core::result::Result::Ok(#value) => #value,
                ::core::result::Result::Err(_) => ::core::panic!(::core::concat!(
                    "include_c_str!: file `", #path, "` contains an interior NUL byte",
                )),
            };
        #c_str
    }}
}

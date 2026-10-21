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
use syn::{parse::Parser, punctuated::Punctuated, Expr, Token};

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

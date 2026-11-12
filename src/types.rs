// Copyright (c) 2021 jmjoy
// SPDX-License-Identifier: MulanPSL-2.0 OR Apache-2.0 OR MIT
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See LICENSE-MULAN, LICENSE-APACHE, and LICENSE-MIT for details.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::Parser;

use crate::utils::parse_expr;

/// Generate a type conversion macro for the given type.
pub(crate) fn type_cast(items: TokenStream, target_type: &str) -> TokenStream {
    let expression = match parse_expr.parse2(items) {
        Ok(expression) => expression,
        Err(error) => return error.into_compile_error(),
    };
    let target_type = Ident::new(target_type, Span::call_site());

    quote! { (#expression) as ::core::primitive::#target_type }
}

#[cfg(test)]
mod tests {
    use crate::utils::{pm_test_left, pm_test_right};

    use super::*;

    #[test]
    fn test_type_cast() {
        assert_eq!(
            pm_test_left(|items| type_cast(items, "u8"), "42"),
            pm_test_right("(42) as ::core::primitive::u8"),
        );

        assert_eq!(
            pm_test_left(|items| type_cast(items, "f32"), "13.25"),
            pm_test_right("(13.25) as ::core::primitive::f32"),
        );

        assert_eq!(
            pm_test_left(|items| type_cast(items, "i32"), "some_var + 1"),
            pm_test_right("(some_var + 1) as ::core::primitive::i32"),
        );
    }
}

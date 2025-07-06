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

/// Generate a type conversion macro for the given type.
pub(crate) fn type_cast(items: TokenStream, target_type: &str) -> TokenStream {
    format!("({items}) as {target_type}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::{pm_test_left, pm_test_right};

    use super::*;

    #[test]
    fn test_type_cast() {
        assert_eq!(
            pm_test_left(|items| type_cast(items, "u8"), "42"),
            pm_test_right("(42) as u8"),
        );

        assert_eq!(
            pm_test_left(|items| type_cast(items, "f32"), "3.14"),
            pm_test_right("(3.14) as f32"),
        );

        assert_eq!(
            pm_test_left(|items| type_cast(items, "i32"), "some_var + 1"),
            pm_test_right("(some_var + 1) as i32"),
        );
    }
}

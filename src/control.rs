// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro2::{TokenStream, TokenTree};

enum EitherFlag {
    Condition,
    Left,
    Right,
}

pub(crate) fn either(items: TokenStream) -> TokenStream {
    let mut condition = TokenStream::new();
    let mut left = TokenStream::new();
    let mut right = TokenStream::new();
    let mut flag = EitherFlag::Condition;

    for item in items {
        match item {
            TokenTree::Punct(p) if p.as_char() == '?' => {
                flag = EitherFlag::Left;
            }
            TokenTree::Punct(p) if p.as_char() == ':' => {
                flag = EitherFlag::Right;
            }
            _ => match flag {
                EitherFlag::Condition => {
                    condition.extend([item]);
                }
                EitherFlag::Left => {
                    left.extend([item]);
                }
                EitherFlag::Right => {
                    right.extend([item]);
                }
            },
        }
    }

    format!("if {condition} {{ {left} }} else {{ {right} }}")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::{pm_test_left, pm_test_right};

    use super::*;

    #[test]
    fn test_either() {
        assert_eq!(
            pm_test_left(either, "1 > 2 ? true : false"),
            pm_test_right("if 1 > 2 { true } else { false }"),
        );
    }
}

// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
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

enum OptionUnwrapFlag {
    Option,
    Or,
}

pub(crate) fn option(items: TokenStream) -> TokenStream {
    let mut items = items.into_iter();
    let first = items.next();

    match first {
        Some(first) => {
            if let TokenTree::Ident(ref ident) = first {
                match &*ident.to_string() {
                    "unwrap" => {
                        let mut option = TokenStream::new();
                        let mut or = TokenStream::new();
                        let mut flag = OptionUnwrapFlag::Option;

                        for item in items {
                            match item {
                                TokenTree::Ident(ident) if ident == "or" => {
                                    flag = OptionUnwrapFlag::Or;
                                }
                                _ => match flag {
                                    OptionUnwrapFlag::Option => {
                                        option.extend([item]);
                                    }
                                    OptionUnwrapFlag::Or => {
                                        or.extend([item]);
                                    }
                                },
                            }
                        }

                        format!(
                            r#"
                            match {option} {{
                                Some(sth) => sth,
                                None => {or},
                            }}
                        "#
                        )
                        .parse()
                        .unwrap()
                    }
                    _ => panic!("unknown operation {first}"),
                }
            } else {
                panic!("unknown operation {first}")
            }
        }
        None => panic!("expr can't be empty"),
    }
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

    #[test]
    fn test_option() {
        assert_eq!(
            pm_test_left(option, "unwrap x or 1"),
            pm_test_right(
                r#"
                match x {
                    Some(sth) => sth,
                    None => 1,
                }
            "#
            ),
        );

        assert_eq!(
            pm_test_left(option, "unwrap x or return false"),
            pm_test_right(
                r#"
                match x {
                    Some(sth) => sth,
                    None => return false,
                }
            "#
            ),
        );

        assert_eq!(
            pm_test_left(option, "unwrap x or break"),
            pm_test_right(
                r#"
                match x {
                    Some(sth) => sth,
                    None => break,
                }
            "#
            ),
        );

        assert_eq!(
            pm_test_left(option, "unwrap x or continue"),
            pm_test_right(
                r#"
                match x {
                    Some(sth) => sth,
                    None => continue,
                }
            "#
            ),
        );
    }
}

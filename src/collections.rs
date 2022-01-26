// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR
// FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

#[derive(PartialEq)]
enum MapFlag {
    NEW,
    KEY,
    VALUE,
}

pub(crate) fn hmap(items: TokenStream) -> TokenStream {
    if items.is_empty() {
        return "::std::collections::HashMap::new()".parse().unwrap();
    }

    let mut len = 0usize;
    let mut flag = MapFlag::NEW;
    let mut new_items = Vec::new();
    let mut tmp_item = (TokenStream::new(), TokenStream::new());

    for item in items {
        if flag == MapFlag::NEW {
            flag = MapFlag::KEY;
            len += 1;
        }

        match item {
            TokenTree::Punct(p) if p.as_char() == ':' => {
                flag = MapFlag::VALUE;
            }
            TokenTree::Punct(p) if p.as_char() == ',' => {
                new_items.push(tmp_item);
                tmp_item = (TokenStream::new(), TokenStream::new());
                flag = MapFlag::NEW;
            }
            _ => {
                if flag == MapFlag::KEY {
                    tmp_item.0.extend([item]);
                } else if flag == MapFlag::VALUE {
                    tmp_item.1.extend([item]);
                }
            }
        }
    }

    if flag != MapFlag::NEW {
        new_items.push(tmp_item);
    }

    let mut stream = TokenStream::new();
    stream.extend(
        format!("let mut map = ::std::collections::HashMap::with_capacity({len});")
            .parse::<TokenStream>()
            .unwrap(),
    );

    for (key, value) in new_items {
        stream.extend(
            format!("let _ = map.insert({key}, {value});")
                .parse::<TokenStream>()
                .unwrap(),
        );
    }
    stream.extend("map".parse::<TokenStream>().unwrap());

    TokenTree::Group(Group::new(Delimiter::Brace, stream)).into()
}

#[cfg(test)]
mod tests {
    use crate::utils::{pm_test_left, pm_test_right};

    use super::*;

    #[test]
    fn test_hmap() {
        assert_eq!(
            pm_test_left(hmap, ""),
            pm_test_right("::std::collections::HashMap::new()"),
        );

        assert_eq!(
            pm_test_left(
                hmap,
                r#"
                "key": "value",
                "key1": "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::HashMap::with_capacity(2);
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                hmap,
                r#"
                "key": "value",
                "key1": "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::HashMap::with_capacity(2);
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );
    }
}

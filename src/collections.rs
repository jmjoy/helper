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

pub(crate) fn hmap(items: TokenStream) -> TokenStream {
    map(items, "::std::collections::HashMap", true)
}

pub(crate) fn btmap(items: TokenStream) -> TokenStream {
    map(items, "::std::collections::BTreeMap", false)
}

#[derive(PartialEq)]
enum MapFlag {
    Key,
    Value,
}

fn map(items: TokenStream, qualifier: &str, has_capacity: bool) -> TokenStream {
    if items.is_empty() {
        return format!("{qualifier}::new()").parse().unwrap();
    }

    let mut len = 0usize;
    let mut flag = MapFlag::Key;
    let mut new_items = Vec::new();
    let mut tmp_item = (TokenStream::new(), TokenStream::new());

    for item in items {
        match item {
            TokenTree::Punct(p) if p.as_char() == ':' => {
                flag = MapFlag::Value;
            }
            TokenTree::Punct(p) if p.as_char() == ',' => {
                new_items.push(tmp_item);
                tmp_item = (TokenStream::new(), TokenStream::new());
                flag = MapFlag::Key;
                len += 1;
            }
            _ => match flag {
                MapFlag::Key => {
                    tmp_item.0.extend([item]);
                }
                MapFlag::Value => {
                    tmp_item.1.extend([item]);
                }
            },
        }
    }

    if flag != MapFlag::Key {
        new_items.push(tmp_item);
        len += 1;
    }

    let inserts = new_items
        .into_iter()
        .map(|(key, value)| format!("let _ = map.insert({key}, {value});"))
        .collect::<Vec<_>>();
    let inserts = inserts.join("\n");

    let cap_construct = if has_capacity {
        format!("with_capacity({len})")
    } else {
        "new()".to_string()
    };

    let block = format!(
        r#"
        {{
            let mut map = {qualifier}::{cap_construct};
            {inserts}
            map
        }}
    "#
    );

    block.parse().unwrap()
}

pub(crate) fn hset(items: TokenStream) -> TokenStream {
    seq(items, "::std::collections::HashSet", true)
}

pub(crate) fn btset(items: TokenStream) -> TokenStream {
    seq(items, "::std::collections::BTreeSet", false)
}

fn seq(items: TokenStream, qualifier: &str, has_capacity: bool) -> TokenStream {
    if items.is_empty() {
        return format!("{qualifier}::new()").parse().unwrap();
    }

    let mut new_items = Vec::new();
    let mut new_item = TokenStream::new();
    let mut len = 0;

    for item in items {
        match item {
            TokenTree::Punct(p) if p.as_char() == ',' => {
                new_items.push(new_item);
                new_item = TokenStream::new();
                len += 1;
            }
            _ => {
                new_item.extend([item]);
            }
        }
    }

    if !new_item.is_empty() {
        new_items.push(new_item);
        len += 1;
    }

    let inserts = new_items
        .into_iter()
        .map(|item| format!("let _ = seq.insert({item});"))
        .collect::<Vec<_>>();
    let inserts = inserts.join("\n");

    let cap_construct = if has_capacity {
        format!("with_capacity({len})")
    } else {
        "new()".to_string()
    };

    let block = format!(
        r#"
        {{
            let mut seq = {qualifier}::{cap_construct};
            {inserts}
            seq
        }}
    "#
    );

    block.parse().unwrap()
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

    #[test]
    fn test_btmap() {
        assert_eq!(
            pm_test_left(btmap, ""),
            pm_test_right("::std::collections::BTreeMap::new()"),
        );

        assert_eq!(
            pm_test_left(
                btmap,
                r#"
                "key": "value",
                "key1": "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::BTreeMap::new();
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                btmap,
                r#"
                "key": "value",
                "key1": "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut map = ::std::collections::BTreeMap::new();
                    let _ = map.insert("key", "value");
                    let _ = map.insert("key1", "value1");
                    map
                }
            "#
            )
        );
    }

    #[test]
    fn test_hset() {
        assert_eq!(
            pm_test_left(hset, ""),
            pm_test_right("::std::collections::HashSet::new()"),
        );

        assert_eq!(
            pm_test_left(
                hset,
                r#"
                "value",
                "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::HashSet::with_capacity(2);
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                hset,
                r#"
                "value", "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::HashSet::with_capacity(2);
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );
    }

    #[test]
    fn test_btset() {
        assert_eq!(
            pm_test_left(btset, ""),
            pm_test_right("::std::collections::BTreeSet::new()"),
        );

        assert_eq!(
            pm_test_left(
                btset,
                r#"
                "value",
                "value1",
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::BTreeSet::new();
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );

        assert_eq!(
            pm_test_left(
                btset,
                r#"
                "value", "value1"
            "#
            ),
            pm_test_right(
                r#"
                {
                    let mut seq = ::std::collections::BTreeSet::new();
                    let _ = seq.insert("value");
                    let _ = seq.insert("value1");
                    seq
                }
            "#
            )
        );
    }
}

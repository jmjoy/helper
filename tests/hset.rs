// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use helper::hset;
use std::collections::HashSet;

#[test]
pub fn test_hset_0() {
    let s: HashSet<()> = HashSet::new();
    assert_eq!(hset! {}, s);
}

#[test]
pub fn test_hset_1() {
    let mut s = HashSet::new();
    s.insert("value");
    s.insert("value1");

    assert_eq!(
        hset! {
            "value",
            "value1",
        },
        s
    );

    assert_eq!(hset! { "value", "value1" }, s);
}

// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use helper::btset;
use std::collections::BTreeSet;

#[test]
pub fn test_btset_0() {
    let s: BTreeSet<()> = BTreeSet::new();
    assert_eq!(btset! {}, s);
}

#[test]
pub fn test_btset_1() {
    let mut s = BTreeSet::new();
    s.insert("value");
    s.insert("value1");

    assert_eq!(
        btset! {
            "value",
            "value1",
        },
        s
    );

    assert_eq!(btset! { "value", "value1" }, s);
}

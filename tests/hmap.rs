// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use helper::hmap;
use std::collections::HashMap;

#[test]
pub fn test_hmap_0() {
    let m: HashMap<(), ()> = HashMap::new();
    assert_eq!(hmap! {}, m);
}

#[test]
pub fn test_hmap_1() {
    let mut m = HashMap::new();
    m.insert("key", "value");
    m.insert("key1", "value1");

    assert_eq!(
        hmap! {
            "key": "value",
            "key1": "value1",
        },
        m
    );

    assert_eq!(
        hmap! {
            "key": "value",
            "key1": "value1"
        },
        m
    );
}

#[test]
pub fn test_hmap_2() {
    let mut m = HashMap::new();
    m.insert("key", {
        let mut m = HashMap::new();
        m.insert(1, true);
        m
    });
    m.insert("key1", {
        let mut m = HashMap::new();
        m.insert(2, false);
        m
    });

    assert_eq!(
        hmap! {
            "key": hmap! {
                1: true,
            },
            "key1": hmap! {
                2: false
            },
        },
        m
    );
}

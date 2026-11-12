// Copyright (c) 2021 jmjoy
// SPDX-License-Identifier: MulanPSL-2.0 OR Apache-2.0 OR MIT
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See LICENSE-MULAN, LICENSE-APACHE, and LICENSE-MIT for details.

use helper::btmap;
use std::collections::BTreeMap;

#[test]
pub fn test_btmap_0() {
    let m: BTreeMap<(), ()> = BTreeMap::new();
    assert_eq!(btmap! {}, m);
}

#[test]
pub fn test_btmap_1() {
    let mut m = BTreeMap::new();
    m.insert("key", "value");
    m.insert("key1", "value1");

    assert_eq!(
        btmap! {
            "key": "value",
            "key1": "value1",
        },
        m
    );

    assert_eq!(
        btmap! {
            "key": "value",
            "key1": "value1"
        },
        m
    );
}

#[test]
pub fn test_btmap_2() {
    let mut m = BTreeMap::new();
    m.insert("key", {
        let mut m = BTreeMap::new();
        m.insert(1, true);
        m
    });
    m.insert("key1", {
        let mut m = BTreeMap::new();
        m.insert(2, false);
        m
    });

    assert_eq!(
        btmap! {
            "key": btmap! {
                1: true,
            },
            "key1": btmap! {
                2: false
            },
        },
        m
    );
}

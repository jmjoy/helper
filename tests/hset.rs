// Copyright (c) 2021 jmjoy
// SPDX-License-Identifier: MulanPSL-2.0 OR Apache-2.0 OR MIT
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See LICENSE-MULAN, LICENSE-APACHE, and LICENSE-MIT for details.

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

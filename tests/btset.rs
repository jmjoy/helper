// Copyright (c) 2021 jmjoy
// SPDX-License-Identifier: MulanPSL-2.0 OR Apache-2.0 OR MIT
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See LICENSE-MULAN, LICENSE-APACHE, and LICENSE-MIT for details.

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

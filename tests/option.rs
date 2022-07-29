// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![allow(deprecated)]

use helper::option;

#[test]
pub fn test_option_0() {
    let x = option!(unwrap Some(1) or 2);
    assert_eq!(x, 1);

    let x = option!(unwrap None or 2);
    assert_eq!(x, 2);
}

#[test]
pub fn test_option_1() {
    let mut b = false;
    let mut f = || {
        let _ = option!(unwrap Some(1) or return);
        b = true;
    };
    f();
    assert_eq!(b, true);

    let mut b = false;
    let mut f = || {
        let _ = option!(unwrap None::<()> or return);
        b = true;
    };
    f();
    assert_eq!(b, false);
}

#[test]
pub fn test_option_2() {
    let mut x = 0;

    for i in 0..100usize {
        x = i;

        let o = if i < 10 { Some(i) } else { None };

        let _ = option!(unwrap o or break);
    }

    assert_eq!(x, 10);
}

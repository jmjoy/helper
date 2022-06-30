// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use helper::try_option;

#[test]
pub fn test_try_option_0() {
    let x = (|| try_option!(Some(1) ? 2))();
    assert_eq!(x, 1);

    let x = (|| try_option!(None ? 2))();
    assert_eq!(x, 2);
}

#[test]
pub fn test_try_option_1() {
    let mut b = false;
    (|| {
        try_option!(Some(1)?());
        b = true;
    })();
    assert_eq!(b, true);

    let mut b = false;
    (|| {
        let _ = try_option!(None::<()>?());
        b = true;
    })();
    assert_eq!(b, false);
}

// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use helper::either;

#[test]
fn test_both_branches_and_trailing_comma() {
    assert_eq!(either!(true, "left", "right"), "left");
    assert_eq!(either!(false, "left", "right"), "right");
    assert_eq!(either!(1 > 2, "1", "2",), "2");
}

#[test]
fn test_nested_either() {
    assert_eq!(either!(either!(1 > 2, true, false), "l", "r"), "r");
    assert_eq!(either!(true, either!(false, 1, 2), 3), 2);
    assert_eq!(either!(false, 1, either!(true, 2, 3)), 2);
}

#[test]
fn test_result_propagation() {
    fn select(
        condition: Result<bool, &'static str>, left: Result<i32, &'static str>,
        right: Result<i32, &'static str>,
    ) -> Result<i32, &'static str> {
        Ok(either!(condition?, left?, right?))
    }

    assert_eq!(select(Err("condition"), Ok(1), Ok(2)), Err("condition"));
    assert_eq!(select(Ok(true), Err("left"), Ok(2)), Err("left"));
    assert_eq!(select(Ok(false), Ok(1), Err("right")), Err("right"));
    assert_eq!(select(Ok(true), Ok(1), Err("unselected")), Ok(1));
    assert_eq!(select(Ok(false), Err("unselected"), Ok(2)), Ok(2));
}

#[test]
fn test_option_propagation() {
    fn select(condition: Option<bool>, left: Option<i32>, right: Option<i32>) -> Option<i32> {
        Some(either!(condition?, left?, right?))
    }

    assert_eq!(select(None, Some(1), Some(2)), None);
    assert_eq!(select(Some(true), None, Some(2)), None);
    assert_eq!(select(Some(false), Some(1), None), None);
    assert_eq!(select(Some(true), Some(1), None), Some(1));
    assert_eq!(select(Some(false), None, Some(2)), Some(2));
}

#[test]
fn test_discarded_value_still_propagates_errors() {
    fn select(condition: bool) -> Result<(), &'static str> {
        let left: Result<i32, &str> = Err("left");
        let right: Result<i32, &str> = Err("right");
        let _ = either!(condition, left?, right?);
        Ok(())
    }

    assert_eq!(select(true), Err("left"));
    assert_eq!(select(false), Err("right"));
}

#[test]
fn test_evaluates_condition_once_and_only_selected_branch() {
    for condition in [true, false] {
        let mut evaluations = Vec::new();
        let selected = either!(
            {
                evaluations.push("condition");
                condition
            },
            {
                evaluations.push("left");
                "left"
            },
            {
                evaluations.push("right");
                "right"
            },
        );

        assert_eq!(selected, if condition { "left" } else { "right" });
        assert_eq!(evaluations, ["condition", selected]);
    }
}

#[test]
fn test_paths_and_generic_arguments() {
    fn first<T, U>(value: T, _: U) -> T {
        value
    }

    assert_eq!(
        either!(
            first::<bool, u8>(true, 0),
            std::cmp::max::<i32>(1, 2),
            first::<i32, u16>(3, 0),
        ),
        2,
    );
    assert_eq!(either!(false, 0, first::<i32, u16>(3, 0)), 3);
}

#[test]
fn test_closures_with_multiple_arguments() {
    for condition in [true, false] {
        let operation: fn(i32, i32) -> i32 = either!(condition, |a, b| a + b, |a, b| a - b);
        assert_eq!(operation(5, 2), if condition { 7 } else { 3 });
    }
}

#[test]
fn test_struct_literal_in_condition() {
    struct Flag {
        enabled: bool,
    }

    assert_eq!(either!(Flag { enabled: true }.enabled, 1, 2), 1);
    assert_eq!(either!(Flag { enabled: false }.enabled, 1, 2), 2);
}

#[test]
fn test_macro_hygiene() {
    macro_rules! select_local {
        ($condition:expr) => {{
            let local = 42;
            either!($condition, local, 0)
        }};
    }

    assert_eq!(select_local!(true), 42);
    assert_eq!(select_local!(false), 0);
}

#[test]
fn test_return_in_branch_exits_calling_function() {
    fn select(condition: bool) -> i32 {
        let value = either!(condition, return 1, 2);
        value + 1
    }

    assert_eq!(select(true), 1);
    assert_eq!(select(false), 3);
}

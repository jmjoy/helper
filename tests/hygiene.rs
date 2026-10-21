// Cover every public macro's handling of identifiers forwarded by macro_rules!.
macro_rules! cast_hygiene_test {
    ($cast:ident, $outer:expr, $inner:expr) => {
        #[test]
        fn $cast() {
            // An unqualified cast target would resolve to this alias.
            #[allow(non_camel_case_types)]
            type $cast = ();
            let _: Option<$cast> = None;

            macro_rules! wrapper {
                ($input: expr) => {{
                    let value = $inner;
                    let _ = &value;
                    helper::$cast!($input)
                }};
            }

            let value = $outer;
            let result: ::core::primitive::$cast = wrapper!(value);
            assert_eq!(result, value as ::core::primitive::$cast);
        }
    };
}

cast_hygiene_test!(u8, 42u16, 99u16);
cast_hygiene_test!(u16, 42u16, 99u16);
cast_hygiene_test!(u32, 42u16, 99u16);
cast_hygiene_test!(u64, 42u16, 99u16);
cast_hygiene_test!(u128, 42u16, 99u16);
cast_hygiene_test!(usize, 42u16, 99u16);
cast_hygiene_test!(i8, 42u16, 99u16);
cast_hygiene_test!(i16, 42u16, 99u16);
cast_hygiene_test!(i32, 42u16, 99u16);
cast_hygiene_test!(i64, 42u16, 99u16);
cast_hygiene_test!(i128, 42u16, 99u16);
cast_hygiene_test!(isize, 42u16, 99u16);
cast_hygiene_test!(f32, 42u16, 99u16);
cast_hygiene_test!(f64, 42u16, 99u16);
cast_hygiene_test!(char, 65u8, 66u8);
cast_hygiene_test!(bool, true, false);

macro_rules! map_hygiene_test {
    ($map:ident) => {
        #[test]
        fn $map() {
            macro_rules! wrapper {
                ($input: expr) => {{
                    let value = 99;
                    let _ = &value;
                    helper::$map! {0: $input}
                }};
            }

            let value = 42;
            assert_eq!(wrapper!(value)[&0], value);
        }
    };
}

map_hygiene_test!(hmap);
map_hygiene_test!(btmap);

macro_rules! set_hygiene_test {
    ($set:ident) => {
        #[test]
        fn $set() {
            macro_rules! wrapper {
                ($input: expr) => {{
                    let value = 99;
                    let _ = &value;
                    helper::$set! {$input}
                }};
            }

            let value = 42;
            assert!(wrapper!(value).contains(&value));
        }
    };
}

set_hygiene_test!(hset);
set_hygiene_test!(btset);

#[test]
fn either() {
    macro_rules! wrapper {
        ($input:expr) => {{
            let value = 99;
            let _ = &value;
            helper::either!(true, $input, 0)
        }};
    }

    let value = 42;
    assert_eq!(wrapper!(value), value);
}

#[test]
fn forwarded_expressions_keep_precedence() {
    macro_rules! cast_expression {
        ($input:expr) => {
            helper::u8!($input * 2)
        };
    }
    macro_rules! hash_map_expression {
        ($input:expr) => {
            helper::hmap! {$input * 2: $input * 3}
        };
    }
    macro_rules! btree_map_expression {
        ($input:expr) => {
            helper::btmap! {$input * 2: $input * 3}
        };
    }
    macro_rules! hash_set_expression {
        ($input:expr) => {
            helper::hset! {$input * 2}
        };
    }
    macro_rules! btree_set_expression {
        ($input:expr) => {
            helper::btset! {$input * 2}
        };
    }
    macro_rules! either_expression {
        ($input:expr) => {
            helper::either!(true, $input * 2, 0)
        };
    }

    assert_eq!(cast_expression!(1 + 2), 6);
    assert_eq!(hash_map_expression!(1 + 2)[&6], 9);
    assert_eq!(btree_map_expression!(1 + 2)[&6], 9);
    assert!(hash_set_expression!(1 + 2).contains(&6));
    assert!(btree_set_expression!(1 + 2).contains(&6));
    assert_eq!(either_expression!(1 + 2), 6);
}

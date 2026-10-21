#![no_std]
#![forbid(unsafe_code)]

use helper::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(u8: Send, Sync, Copy, Clone, Into<u16>,);
assert_impl_all!(str: Send, Sync);
assert_not_impl_any!(str: Sized, Copy,);
assert_not_impl_any!(core::cell::Cell<u8>: Sync, Copy);
assert_impl_all!(core::ops::Range<u8>: Iterator<Item = u8>, Clone);
assert_not_impl_any!(core::ops::Range<u16>: Iterator<Item = u8>);

#[test]
fn supports_generic_traits_and_associated_type_bindings() {
    trait Pair<A, B> {}
    impl Pair<u8, u16> for u32 {}

    assert_impl_all!(u32: Pair<u8, u16>, Into<u64>);
    assert_not_impl_any!(u8: Pair<u8, u16>, Iterator<Item = u8>);
}

#[test]
fn supports_higher_ranked_bounds() {
    type Function = fn(&str) -> usize;

    assert_impl_all!(Function: for<'a> Fn(&'a str) -> usize, Send, Sync);
    assert_not_impl_any!(Function: for<'a> Fn(&'a str) -> bool);

    let function: Function = str::len;
    assert_eq!(function("hello"), 5);
}

#[test]
fn generated_types_do_not_shadow_target_types_or_bound_arguments() {
    type __HelperAssertType = u16;
    type __HelperImplemented = u32;
    trait __HelperAmbiguousIfImplemented {}

    assert_impl_all!(u8: Into<__HelperAssertType>);
    assert_not_impl_any!(__HelperImplemented: Into<__HelperAssertType>);
    assert_not_impl_any!(u8: __HelperAmbiguousIfImplemented);
}

#[test]
fn supports_types_and_trait_paths_forwarded_by_macro_rules() {
    macro_rules! check {
        ($ty:ty, $bound:path) => {
            assert_impl_all!($ty: $bound);
        };
    }
    macro_rules! check_not {
        ($ty:ty, $bound:path) => {
            assert_not_impl_any!($ty: $bound);
        };
    }

    check!(u8, Into<u16>);
    check_not!(u16, Into<u8>);
}

#[test]
fn preserves_precedence_in_forwarded_const_generics() {
    trait Length<const N: usize> {}
    impl Length<6> for [u8; 6] {}

    macro_rules! check {
        ($value:expr) => {
            assert_impl_all!([u8; $value * 2]: Length<6>);
            assert_impl_all!([u8; 6]: Length<{ $value * 2 }>);
            assert_not_impl_any!([u8; 5]: Length<{ $value * 2 }>);
        };
    }
    check!(1 + 2);
}

use helper::*;

#[test]
fn test_u8_conversion() {
    let x = 42i32;
    let y = u8!(x);
    assert_eq!(y, 42u8);
}

#[test]
fn test_u16_conversion() {
    let x = 42i32;
    let y = u16!(x);
    assert_eq!(y, 42u16);
}

#[test]
fn test_u32_conversion() {
    let x = 42i64;
    let y = u32!(x);
    assert_eq!(y, 42u32);
}

#[test]
fn test_u64_conversion() {
    let x = 42i32;
    let y = u64!(x);
    assert_eq!(y, 42u64);
}

#[test]
fn test_u128_conversion() {
    let x = 42i32;
    let y = u128!(x);
    assert_eq!(y, 42u128);
}

#[test]
fn test_usize_conversion() {
    let x = 42i32;
    let y = usize!(x);
    assert_eq!(y, 42usize);
}

#[test]
fn test_i8_conversion() {
    let x = 42i32;
    let y = i8!(x);
    assert_eq!(y, 42i8);
}

#[test]
fn test_i16_conversion() {
    let x = 42i32;
    let y = i16!(x);
    assert_eq!(y, 42i16);
}

#[test]
fn test_i32_conversion() {
    let x = 42i64;
    let y = i32!(x);
    assert_eq!(y, 42i32);
}

#[test]
fn test_i64_conversion() {
    let x = 42i32;
    let y = i64!(x);
    assert_eq!(y, 42i64);
}

#[test]
fn test_i128_conversion() {
    let x = 42i32;
    let y = i128!(x);
    assert_eq!(y, 42i128);
}

#[test]
fn test_isize_conversion() {
    let x = 42i32;
    let y = isize!(x);
    assert_eq!(y, 42isize);
}

#[test]
fn test_f32_conversion() {
    let x = 13.25f64;
    let y = f32!(x);
    assert!((y - 13.25f32).abs() < f32::EPSILON);
}

#[test]
fn test_f64_conversion() {
    let x = 13.25f32;
    let y = f64!(x);
    assert!((y - 13.25f64).abs() < f64::EPSILON);
}

#[test]
fn test_char_conversion() {
    let x = 65u8;
    let y = char!(x);
    assert_eq!(y, 'A');
}

#[test]
fn test_complex_expressions() {
    let x = 10i32;
    let y = 20i32;

    // Test with complex expressions
    let result = u64!(x + y * 2);
    assert_eq!(result, 50u64);

    let result = f32!(x as f64 / 3.0);
    assert!((result - (10.0 / 3.0) as f32).abs() < f32::EPSILON);
}

#[test]
fn test_chained_conversions() {
    let x = 42i32;
    let y = f64!(i64!(u32!(x)));
    assert!((y - 42.0f64).abs() < f64::EPSILON);
}

#[test]
fn test_bool_conversion() {
    assert!(bool!(true));
    assert!(!bool!(false));
}

#[test]
fn test_expression_is_evaluated_once() {
    let mut calls = 0;
    let value = u8!({
        calls += 1;
        42
    });

    assert_eq!(value, 42);
    assert_eq!(calls, 1);
}

#[test]
fn test_cast_in_const_and_expression_contexts() {
    const VALUE: u8 = helper::u8!(256u16 + 1);
    assert_eq!(VALUE, 1);
    assert_eq!(helper::u8!(40 + 2).to_string(), "42");
    assert_eq!(helper::i32!(-2.5f64), -2);
}

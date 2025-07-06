use helper::*;

fn main() {
    println!("=== Type Conversion Examples ===");

    // Integer conversions
    let x = 42i32;
    println!("Converting {} from i32 to:", x);
    println!("  u8: {}", u8!(x));
    println!("  u16: {}", u16!(x));
    println!("  u32: {}", u32!(x));
    println!("  u64: {}", u64!(x));
    println!("  u128: {}", u128!(x));
    println!("  usize: {}", usize!(x));
    println!("  i8: {}", i8!(x));
    println!("  i16: {}", i16!(x));
    println!("  i64: {}", i64!(x));
    println!("  i128: {}", i128!(x));
    println!("  isize: {}", isize!(x));

    // Float conversions
    let y = 3.14f32;
    println!("\nConverting {} from f32 to:", y);
    println!("  f64: {}", f64!(y));

    let z = 2.718f64;
    println!("\nConverting {} from f64 to:", z);
    println!("  f32: {}", f32!(z));

    // Character conversion
    let ascii_value = 65u8;
    println!(
        "\nConverting {} from u8 to char: {}",
        ascii_value,
        char!(ascii_value)
    );

    // Complex expressions
    let a = 10i32;
    let b = 20i32;
    println!("\nComplex expression conversions:");
    println!("u64!({} + {} * 2) = {}", a, b, u64!(a + b * 2));
    println!("f32!({} as f64 / 3.0) = {}", a, f32!(a as f64 / 3.0));

    // Chained conversions
    let original = 42i32;
    let result = f64!(i64!(u32!(original)));
    println!("\nChained conversion:");
    println!("f64!(i64!(u32!({}))) = {}", original, result);
}

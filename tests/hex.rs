#![no_std]
#![forbid(unsafe_code)]

use helper::hex;

#[test]
fn works_in_const_and_static_initializers() {
    const MAGIC: [u8; 4] = hex!("DE AD BE EF");
    static BYTES: [u8; 4] = hex!("deadbeef");

    assert_eq!(MAGIC, [0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(BYTES, MAGIC);
}

#[test]
fn supports_all_digits_and_both_cases() {
    assert_eq!(
        hex!("0123456789abcdef"),
        [1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
    );
    assert_eq!(hex!("ABCDEF00FF"), [0xab, 0xcd, 0xef, 0, 255]);
}

#[test]
fn ignores_ascii_whitespace_including_between_nibbles() {
    assert_eq!(hex!(" \t\r\n\x0b\x0c A b C\tD\nE\rF "), [0xab, 0xcd, 0xef]);
    assert_eq!(
        hex!(
            r#"
        de ad
        be ef
    "#
        ),
        [0xde, 0xad, 0xbe, 0xef]
    );
}

#[test]
fn handles_empty_literals_and_optional_trailing_commas() {
    let empty = hex!("");
    assert_eq!(empty.len(), 0);
    assert_eq!(hex!(" \t\n",).len(), 0);
    assert_eq!(hex!("00 ff",), [0, 255]);
}

#[test]
fn accepts_literals_and_expressions_forwarded_by_macro_rules() {
    macro_rules! literal {
        ($value:literal) => {
            hex!($value)
        };
    }
    macro_rules! expression {
        ($value:expr) => {
            hex!($value)
        };
    }

    assert_eq!(literal!("de ad"), [0xde, 0xad]);
    assert_eq!(expression!("be ef"), [0xbe, 0xef]);
}

#[test]
fn primitive_type_and_generated_names_are_hygienic() {
    #[allow(non_camel_case_types)]
    type u8 = ();
    let _: u8 = ();
    const HEX_BYTES: usize = 42;

    let first = hex!("ff");
    let second = hex!("00");
    assert_eq!(first, [255]);
    assert_eq!(second, [0]);
    assert_eq!(HEX_BYTES, 42);
}

#![no_std]
#![forbid(unsafe_code)]

use helper::concat_bytes;

#[test]
fn combines_bytes_strings_and_arrays_in_input_order() {
    const HEADER: &[u8; 7] = concat_bytes!(b'A', b"BC", [68, b'E'], [b'!'; 2]);
    static MAGIC: &[u8; 4] = concat_bytes!(b"PK", [3, 4]);

    assert_eq!(HEADER, b"ABCDE!!");
    assert_eq!(MAGIC, b"PK\x03\x04");
}

#[test]
fn accepts_raw_strings_escapes_and_all_integer_radices() {
    assert_eq!(concat_bytes!(br#"a\nb"#, b"\0\xff"), b"a\\nb\0\xff");
    assert_eq!(
        concat_bytes!([0xff, 0b1010u8, 0o17, 1_00, 0u8]),
        &[255, 10, 15, 100, 0]
    );
}

#[test]
fn supports_empty_inputs_and_trailing_commas() {
    let empty: &[u8; 0] = concat_bytes!();
    assert!(empty.is_empty());
    assert_eq!(concat_bytes!(b"", [], [42; 0],).len(), 0);
    assert_eq!(concat_bytes!(b"x",), b"x");
}

#[test]
fn supports_repeated_bytes_and_integer_repeat_counts() {
    assert_eq!(
        concat_bytes!([0xffu8; 3usize], [b'!'; 0x2],),
        b"\xff\xff\xff!!"
    );
}

#[test]
fn supports_forwarded_literals_and_expressions() {
    macro_rules! literal {
        ($value:literal) => {
            concat_bytes!($value)
        };
    }
    macro_rules! expression {
        ($value:expr) => {
            concat_bytes!($value)
        };
    }
    macro_rules! array {
        ($value:expr, $count:expr) => {
            concat_bytes!([$value; $count])
        };
    }

    assert_eq!(literal!(b"hello"), b"hello");
    assert_eq!(expression!([1, 2]), &[1, 2]);
    assert_eq!(array!(b'!', 3), b"!!!");
}

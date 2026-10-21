// Copyright (c) 2021 jmjoy
// Helper is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![no_std]
#![forbid(unsafe_code)]

use core::ffi::CStr;

use helper::include_c_str;

#[test]
fn test_text_and_static_lifetime() {
    fn included() -> &'static CStr {
        include_c_str!("fixtures/include_c_str/plain.txt")
    }

    assert_eq!(included().to_bytes(), b"hello\n");
    assert_eq!(included().to_bytes_with_nul(), b"hello\n\0");
}

#[test]
fn test_const_and_static_initializers() {
    const TEXT: &CStr = include_c_str!("fixtures/include_c_str/plain.txt");
    static STATIC_TEXT: &CStr = include_c_str!("fixtures/include_c_str/plain.txt");

    assert_eq!(TEXT.to_bytes(), b"hello\n");
    assert_eq!(STATIC_TEXT, TEXT);
}

#[test]
fn test_existing_terminator_is_preserved() {
    let text = include_c_str!("fixtures/include_c_str/terminated.bin");
    assert_eq!(text.to_bytes_with_nul(), b"hello\n\0");
}

#[test]
fn test_empty_file_and_single_nul() {
    let empty = include_c_str!("fixtures/include_c_str/empty.txt");
    let nul = include_c_str!("fixtures/include_c_str/nul.bin");

    assert_eq!(empty.to_bytes_with_nul(), b"\0");
    assert_eq!(nul.to_bytes_with_nul(), b"\0");
}

#[test]
fn test_non_utf8_bytes() {
    let text = include_c_str!("fixtures/include_c_str/non_utf8.bin");

    assert_eq!(text.to_bytes_with_nul(), b"\xff\xfe\x80\0");
    assert!(text.to_str().is_err());
}

#[test]
fn test_unicode_and_raw_path() {
    let text = include_c_str!(r#"fixtures/include_c_str/unicode.txt"#);

    assert_eq!(text.to_str().unwrap(), "你好，Rust!\n");
}

#[test]
fn test_path_macros_and_trailing_comma() {
    let text = include_c_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/include_c_str/plain.txt",
    ),);
    assert_eq!(text.to_bytes(), b"hello\n");

    macro_rules! fixture_path {
        () => {
            "fixtures/include_c_str/plain.txt"
        };
    }
    assert_eq!(include_c_str!(fixture_path!()).to_bytes(), b"hello\n");
}

#[test]
fn test_forwarded_paths() {
    macro_rules! include_literal {
        ($path:literal) => {
            include_c_str!($path)
        };
    }
    macro_rules! include_expression {
        ($path:expr) => {
            include_c_str!($path)
        };
    }

    assert_eq!(
        include_literal!("fixtures/include_c_str/plain.txt").to_bytes(),
        b"hello\n",
    );
    assert_eq!(
        include_expression!("fixtures/include_c_str/plain.txt").to_bytes(),
        b"hello\n",
    );
    assert_eq!(
        include_expression!(concat!("fixtures/include_c_str/", "plain.txt")).to_bytes(),
        b"hello\n",
    );
}

#[test]
fn test_multiple_invocations_and_shadowed_names() {
    #[allow(non_camel_case_types)]
    type u8 = ();
    #[allow(non_camel_case_types)]
    type usize = ();
    let _: (u8, usize) = ((), ());
    let (data, index, value) = (1, 2, 3);

    let first = include_c_str!("fixtures/include_c_str/plain.txt");
    let second = include_c_str!("fixtures/include_c_str/unicode.txt");

    assert_eq!(first.to_bytes(), b"hello\n");
    assert_eq!(second.to_str().unwrap(), "你好，Rust!\n");
    assert_eq!((data, index, value), (1, 2, 3));
}

#![no_std]
#![forbid(unsafe_code)]

use helper::include_str_trimmed;

#[test]
fn trims_ascii_whitespace_and_preserves_interior_content() {
    const TEXT: &str = include_str_trimmed!("fixtures/include_str_trimmed/text.txt");
    static STATIC_TEXT: &str = include_str_trimmed!("fixtures/include_str_trimmed/text.txt");

    assert_eq!(TEXT, "first  line\nsecond\tline");
    assert_eq!(STATIC_TEXT, TEXT);
}

#[test]
fn handles_empty_and_all_whitespace_files() {
    assert_eq!(include_str_trimmed!("fixtures/include_c_str/empty.txt"), "");
    assert_eq!(
        include_str_trimmed!("fixtures/include_str_trimmed/whitespace.txt"),
        ""
    );
}

#[test]
fn preserves_utf8_and_non_ascii_whitespace() {
    assert_eq!(
        include_str_trimmed!("fixtures/include_c_str/unicode.txt"),
        "你好，Rust!"
    );
    assert_eq!(
        include_str_trimmed!("fixtures/include_str_trimmed/unicode_whitespace.txt"),
        "\u{a0}hello\u{3000}"
    );
}

#[test]
fn preserves_nul_bytes_and_text_without_whitespace() {
    assert_eq!(include_str_trimmed!("fixtures/include_c_str/nul.bin"), "\0");
    assert_eq!(
        include_str_trimmed!("fixtures/include_str_trimmed/untrimmed.txt"),
        "unchanged"
    );
}

#[test]
fn paths_are_relative_to_the_caller_and_accept_macros() {
    assert_eq!(
        include_str_trimmed!(r#"fixtures/include_c_str/plain.txt"#,),
        "hello"
    );
    assert_eq!(
        include_str_trimmed!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/include_c_str/plain.txt",
        )),
        "hello"
    );

    macro_rules! path {
        () => {
            "fixtures/include_c_str/plain.txt"
        };
    }
    assert_eq!(include_str_trimmed!(path!()), "hello");
}

#[test]
fn supports_forwarded_paths_and_hygienic_names() {
    macro_rules! literal {
        ($path:literal) => {
            include_str_trimmed!($path)
        };
    }
    macro_rules! expression {
        ($path:expr) => {
            include_str_trimmed!($path)
        };
    }

    #[allow(non_camel_case_types)]
    type str = ();
    let _: str = ();
    let (data, start, end, index, value) = (1, 2, 3, 4, 5);

    assert_eq!(literal!("fixtures/include_c_str/plain.txt"), "hello");
    assert_eq!(
        expression!(concat!("fixtures/include_c_str/", "plain.txt")),
        "hello"
    );
    assert_eq!((data, start, end, index, value), (1, 2, 3, 4, 5));
}

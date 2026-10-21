#![no_std]
#![forbid(unsafe_code)]

use helper::indoc;

#[test]
fn removes_common_indentation_in_const_and_static_initializers() {
    const SQL: &str = indoc! {r#"
        SELECT id, name
        FROM users
        WHERE active = true
    "#};
    static TEXT: &str = indoc!("\n    hello\n    world\n");

    assert_eq!(SQL, "SELECT id, name\nFROM users\nWHERE active = true\n");
    assert_eq!(TEXT, "hello\nworld\n");
}

#[test]
fn preserves_relative_indentation() {
    let text = indoc! {r#"
        {
            "enabled": true,
            "limits": {
                "requests": 10
            }
        }
    "#};

    assert_eq!(
        text,
        "{\n    \"enabled\": true,\n    \"limits\": {\n        \"requests\": 10\n    }\n}\n"
    );
}

#[test]
fn preserves_a_nonempty_first_line_and_excludes_it_from_the_margin() {
    assert_eq!(
        indoc!("  heading\n      detail\n    tail"),
        "  heading\n  detail\ntail"
    );
    assert_eq!(indoc!(" \n    text"), " \ntext");
}

#[test]
fn preserves_single_lines_and_handles_empty_input() {
    assert_eq!(indoc!(""), "");
    assert_eq!(indoc!("\n"), "");
    assert_eq!(indoc!("\r\n"), "");
    assert_eq!(indoc!("    single line  "), "    single line  ");
    assert_eq!(indoc!("   "), "   ");
}

#[test]
fn blank_lines_do_not_reduce_the_common_margin() {
    assert_eq!(
        indoc!("\n    first\n \n      second\n\n    last\n  "),
        "first\n\n  second\n\nlast\n",
    );
}

#[test]
fn only_removes_one_initial_line_ending() {
    assert_eq!(indoc!("\n\n    text\n"), "\ntext\n");
    assert_eq!(indoc!("\n\n\n"), "\n\n");
    assert_eq!(indoc!("\n  \n\t"), "  \n\t");
}

#[test]
fn preserves_extra_whitespace_on_blank_lines_and_at_line_ends() {
    assert_eq!(
        indoc!("\n    first  \n    second\t \n"),
        "first  \nsecond\t \n"
    );
    assert_eq!(indoc!("\n  first\n    \n  last\n"), "first\n  \nlast\n");
}

#[test]
fn spaces_and_tabs_each_count_as_one_indentation_character() {
    assert_eq!(
        indoc!("\n\t first\n \t  second\n\t third\n"),
        "first\n  second\nthird\n"
    );
    assert_eq!(indoc!("\n\t\tfirst\n\t\t\tsecond\n"), "first\n\tsecond\n");
}

#[test]
fn preserves_crlf_line_endings_and_ignores_crlf_blank_lines() {
    assert_eq!(
        indoc!("\r\n    first\r\n  \r\n      second\r\n    "),
        "first\r\n\r\n  second\r\n"
    );
    assert_eq!(
        indoc!("first\r\n  second\n  third"),
        "first\r\nsecond\nthird"
    );
}

#[test]
fn preserves_unicode_other_whitespace_and_escaped_content() {
    assert_eq!(
        indoc!("\n    \u{2003}你好\n      café 🦀\n"),
        "\u{2003}你好\n  café 🦀\n"
    );
    assert_eq!(indoc!("\n    \x0b\x0c\r\0\"\\\n"), "\x0b\x0c\r\0\"\\\n");
}

#[test]
fn supports_byte_strings_with_non_utf8_bytes() {
    const BYTES: &[u8; 7] = indoc!(b"\n    \xff\0\n      \x80\n",);
    static EMPTY: &[u8; 0] = indoc!(b"\n");

    assert_eq!(BYTES, b"\xff\0\n  \x80\n");
    assert_eq!(EMPTY, b"");
}

#[test]
fn raw_byte_strings_preserve_backslashes() {
    let bytes = indoc! {br#"
        \xff
          text\n
    "#};

    assert_eq!(bytes, b"\\xff\n  text\\n\n");
}

#[test]
fn supports_trailing_commas_and_forwarded_literals_or_expressions() {
    macro_rules! literal {
        ($value:literal) => {
            indoc!($value)
        };
    }
    macro_rules! expression {
        ($value:expr) => {
            indoc!($value,)
        };
    }

    assert_eq!(indoc!("\n    hello\n",), "hello\n");
    assert_eq!(literal!("\n    hello\n"), "hello\n");
    assert_eq!(expression!("\n    hello\n"), "hello\n");
    assert_eq!(literal!(b"\n    \xff\n"), b"\xff\n");
    assert_eq!(expression!(b"\n    \xff\n"), b"\xff\n");
}

#[test]
fn emits_a_literal_usable_by_other_builtin_macros() {
    const JOINED: &str = concat!(indoc!("\n    hello\n"), "world");
    assert_eq!(JOINED, "hello\nworld");
}

#![warn(rust_2018_idioms, non_ascii_idents)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]

/// This is a useful macro to write expression like `?:` or `if expr { left } else { right }`.
///
/// # Example
///
/// ```
/// use helper::if_or_else;
///
/// let s = "hello";
/// let len = s.len();
/// let len = if_or_else!(len > 10, 10, len);
/// assert_eq!(len, 5);
/// ```
#[macro_export]
macro_rules! if_or_else {
    ($cond:expr, $left:expr, $right:expr) => {
        if $cond { $left } else { $right }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($x:expr) => {
        unwrap_or_return!($x, ())
    };
    ($x:expr, $ret:expr) => {
        match x {
            Some(x) => x,
            None => return $ret,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_break {
    ($x:expr) => {
        unwrap_or_break!($x, ())
    };
    ($x:expr, $ret:expr) => {
        match x {
            Some(x) => x,
            None => break $ret,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($x:expr) => {
        match x {
            Some(x) => x,
            None => continue,
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_or_else() {
        let a = 1;
        assert_eq!(if_or_else!(a > 0, 1, 2), 1);
        assert_eq!(if_or_else!(a > 1, 1, 2), 2);
    }
}

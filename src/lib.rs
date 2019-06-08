/// This is a useful macro to write expression like `?:` or `if expr { left } else { right }`.
///
/// # Example
///
/// ```
/// use helper::r#if;
///
/// let s = "hello";
/// let len = s.len();
/// let len = r#if!(len > 10, 10, len);
/// assert_eq!(len, 5);
/// ```
#[macro_export]
macro_rules! r#if {
    ($cond:expr, $left:expr, $right:expr) => {
        if $cond { $left } else { $right }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if() {
        let a = 1;
        assert_eq!(r#if!(a > 0, 1, 2), 1);
        assert_eq!(r#if!(a > 1, 1, 2), 2);
    }
}
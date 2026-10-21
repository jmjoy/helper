#![no_std]
#![forbid(unsafe_code)]

use core::cell::Cell;

#[test]
fn capture_and_defer_work_without_std_or_alloc() {
    #[derive(Clone)]
    struct Value(u8);

    let value = Value(42);
    let callback = helper::capture!([value], move || value.0);
    assert_eq!(callback(), 42);
    assert_eq!(value.0, 42);

    let task = helper::capture!([value], async move { value.0 });
    drop(task);
    assert_eq!(value.0, 42);

    let count = Cell::new(0);
    {
        helper::defer! { count.set(count.get() + 1); }
        helper::defer! { count.set(count.get() + 1); }
        assert_eq!(count.get(), 0);
    }
    assert_eq!(count.get(), 2);
}

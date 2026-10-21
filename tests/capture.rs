#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    future::Future,
    pin::pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use helper::capture;

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

#[test]
fn originals_remain_available_and_closures_can_run_repeatedly() {
    let text = String::from("hello");
    let callback = capture!([text], move |count: usize| text.repeat(count));

    assert_eq!(callback(1), "hello");
    assert_eq!(callback(2), "hellohello");
    assert_eq!(text, "hello");
}

#[test]
fn clones_are_created_once_in_list_order_before_the_closure_runs() {
    struct Tracked(&'static str, Rc<RefCell<Vec<&'static str>>>);

    impl Clone for Tracked {
        fn clone(&self) -> Self {
            self.1.borrow_mut().push(self.0);
            Self(self.0, Rc::clone(&self.1))
        }
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    let first = Tracked("first", Rc::clone(&events));
    let second = Tracked("second", Rc::clone(&events));
    let callback = capture!([first, second], move || (first.0, second.0));

    assert_eq!(*events.borrow(), ["first", "second"]);
    assert_eq!(callback(), ("first", "second"));
    assert_eq!(callback(), ("first", "second"));
    assert_eq!(*events.borrow(), ["first", "second"]);
}

#[test]
fn cloned_bindings_can_be_mutated() {
    let values = vec![1];
    let mut callback = capture!([values], move || {
        values.push(2);
        values.len()
    });

    assert_eq!(callback(), 2);
    assert_eq!(callback(), 3);
    assert_eq!(values, [1]);
}

#[test]
fn unlisted_values_follow_normal_move_capture_rules() {
    let text = String::from("hello");
    let owned = Box::new(42);
    let callback = capture!([text], move || (text, owned));

    assert_eq!(callback(), (String::from("hello"), Box::new(42)));
    assert_eq!(text, "hello");
}

#[test]
fn async_blocks_clone_immediately_and_can_await_after_the_original_is_dropped() {
    let text = Rc::new(String::from("hello"));
    let task = capture!([text], async move { std::future::ready(text.len()).await });

    assert_eq!(Rc::strong_count(&text), 2);
    drop(text);

    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut task = pin!(task);
    assert_eq!(task.as_mut().poll(&mut context), Poll::Ready(5));
}

#[test]
fn dropping_an_unpolled_future_releases_its_clones() {
    let text = Rc::new(String::from("hello"));
    let task = capture!([text], async move { text.len() });

    assert_eq!(Rc::strong_count(&text), 2);
    drop(task);
    assert_eq!(Rc::strong_count(&text), 1);
}

#[test]
fn uses_clone_trait_even_with_an_inherent_clone_method() {
    #[derive(Clone)]
    struct Value(u8);

    impl Value {
        fn clone(&self) -> u8 {
            99
        }
    }

    let value = Value(42);
    let callback = capture!([value], move || value.0);
    assert_eq!(callback(), 42);
    assert_eq!(value.clone(), 99);
}

#[test]
fn supports_empty_lists_raw_identifiers_and_trailing_commas() {
    assert_eq!(capture!([], move || 42,)(), 42);

    let r#type = String::from("value");
    let callback = capture!([r#type,], move || r#type.len(),);
    assert_eq!(callback(), 5);
    assert_eq!(r#type, "value");
}

#[test]
fn preserves_identifiers_and_expressions_forwarded_by_macro_rules() {
    macro_rules! wrapper {
        ($name:ident, $body:expr) => {{
            let text = "inner";
            let _ = &text;
            capture!([$name], $body)
        }};
    }

    let text = String::from("outer");
    let callback = wrapper!(text, move || text.len());
    assert_eq!(callback(), 5);
    assert_eq!(text, "outer");
}

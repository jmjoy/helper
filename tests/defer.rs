#![forbid(unsafe_code)]

use std::{
    cell::{Cell, RefCell},
    future::{pending, Future},
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use helper::defer;

#[test]
fn runs_at_scope_exit_in_reverse_registration_order() {
    let events = RefCell::new(Vec::new());
    {
        defer! { events.borrow_mut().push(1); }
        defer! { events.borrow_mut().push(2); }
        events.borrow_mut().push(0);
        assert_eq!(*events.borrow(), [0]);
    }
    assert_eq!(*events.borrow(), [0, 2, 1]);
}

#[test]
fn nested_scopes_have_independent_cleanup() {
    let events = RefCell::new(Vec::new());
    {
        defer! { events.borrow_mut().push("outer"); }
        {
            defer! { events.borrow_mut().push("inner"); }
        }
        assert_eq!(*events.borrow(), ["inner"]);
    }
    assert_eq!(*events.borrow(), ["inner", "outer"]);
}

#[test]
fn cleans_up_on_early_return_and_question_mark() {
    fn run(events: &RefCell<Vec<&'static str>>, result: Result<(), ()>) -> Result<(), ()> {
        defer! { events.borrow_mut().push("cleanup"); }
        result?;
        events.borrow_mut().push("body");
        Ok(())
    }

    let events = RefCell::new(Vec::new());
    assert_eq!(run(&events, Err(())), Err(()));
    assert_eq!(*events.borrow(), ["cleanup"]);
    assert_eq!(run(&events, Ok(())), Ok(()));
    assert_eq!(*events.borrow(), ["cleanup", "body", "cleanup"]);

    fn early_return(count: &Cell<u8>) {
        defer! { count.set(count.get() + 1); }
        if count.get() == 0 {
            return;
        }
        count.set(99);
    }

    let count = Cell::new(0);
    early_return(&count);
    assert_eq!(count.get(), 1);
}

#[test]
fn cleans_up_on_continue_and_break() {
    let events = RefCell::new(Vec::new());
    for index in 0..4 {
        defer! { events.borrow_mut().push(index); }
        if index == 0 {
            continue;
        }
        if index == 2 {
            break;
        }
    }
    assert_eq!(*events.borrow(), [0, 1, 2]);
}

#[test]
fn cleans_up_during_panic_unwinding() {
    let events = RefCell::new(Vec::new());
    let result = catch_unwind(AssertUnwindSafe(|| {
        defer! { events.borrow_mut().push(1); }
        defer! { events.borrow_mut().push(2); }
        panic!("trigger unwinding");
    }));

    assert!(result.is_err());
    assert_eq!(*events.borrow(), [2, 1]);
}

#[test]
fn can_move_and_consume_a_non_clone_resource() {
    struct Resource<'a>(&'a Cell<u8>);

    impl Drop for Resource<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drops = Cell::new(0);
    let resource = Resource(&drops);
    {
        defer! { drop(resource); }
        assert_eq!(drops.get(), 0);
    }
    assert_eq!(drops.get(), 1);
}

#[test]
fn can_borrow_mutably_for_cleanup() {
    let mut values = vec![1];
    {
        defer! { values.push(2); }
    }
    assert_eq!(values, [1, 2]);
}

#[test]
fn return_in_cleanup_only_exits_the_cleanup_closure() {
    let events = RefCell::new(Vec::new());
    {
        defer! { events.borrow_mut().push(1); }
        defer! {
            events.borrow_mut().push(2);
            if events.borrow().len() == 1 {
                return;
            }
            events.borrow_mut().push(3);
        }
    }
    assert_eq!(*events.borrow(), [2, 1]);
}

#[test]
fn cleans_up_when_a_suspended_future_is_cancelled() {
    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    let cleaned = Cell::new(false);
    let mut task = Box::pin(async {
        defer! { cleaned.set(true); }
        pending::<()>().await;
    });
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    assert_eq!(task.as_mut().poll(&mut context), Poll::Pending);
    assert!(!cleaned.get());
    drop(task);
    assert!(cleaned.get());
}

#[test]
fn generated_names_do_not_shadow_user_bindings() {
    let events = RefCell::new(Vec::new());
    let __helper_defer_guard = 42;
    {
        defer! { events.borrow_mut().push(__helper_defer_guard); }
        macro_rules! register {
            ($value:expr) => {
                defer! { events.borrow_mut().push($value); }
            };
        }
        register!(__helper_defer_guard + 1);
        assert!(events.borrow().is_empty());
    }
    assert_eq!(*events.borrow(), [43, 42]);
}

#[test]
fn generated_type_does_not_shadow_user_types_in_cleanup() {
    struct __HelperDeferGuard(u8);

    let value = Cell::new(0);
    {
        defer! { value.set(__HelperDeferGuard(42).0); }
    }
    assert_eq!(value.get(), 42);
}

#[test]
fn forwarded_expressions_keep_precedence() {
    macro_rules! register {
        ($target:expr, $value:expr) => {
            defer! { $target.set($value * 2); }
        };
    }

    let value = Cell::new(0);
    {
        register!(value, 1 + 2);
        assert_eq!(value.get(), 0);
    }
    assert_eq!(value.get(), 6);
}

#[test]
fn accepts_empty_cleanup_and_parenthesized_invocations() {
    let value = Cell::new(0);
    {
        defer!();
        defer!(value.set(42));
        assert_eq!(value.get(), 0);
    }
    assert_eq!(value.get(), 42);
}

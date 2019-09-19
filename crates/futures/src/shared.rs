use js_sys::Promise;
use std::cell::RefCell;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use wasm_bindgen::prelude::*;

/// A Rust `Future` backed by a JavaScript `Promise`.
///
/// This type is constructed with a JavaScript `Promise` object and translates
/// it to a Rust `Future`. This type implements the `Future` trait from the
/// `futures` crate and will either succeed or fail depending on what happens
/// with the JavaScript `Promise`.
///
/// Currently this type is constructed with `JsFuture::from`.
pub struct JsFuture {
    inner: Rc<RefCell<Inner>>,
}

struct Inner {
    result: Option<Result<JsValue, JsValue>>,
    task: Option<Waker>,
    callbacks: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
}

impl fmt::Debug for JsFuture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JsFuture {{ ... }}")
    }
}

impl From<Promise> for JsFuture {
    fn from(js: Promise) -> JsFuture {
        // Use the `then` method to schedule two callbacks, one for the
        // resolved value and one for the rejected value. We're currently
        // assuming that JS engines will unconditionally invoke precisely one of
        // these callbacks, no matter what.
        //
        // Ideally we'd have a way to cancel the callbacks getting invoked and
        // free up state ourselves when this `JsFuture` is dropped. We don't
        // have that, though, and one of the callbacks is likely always going to
        // be invoked.
        //
        // As a result we need to make sure that no matter when the callbacks
        // are invoked they are valid to be called at any time, which means they
        // have to be self-contained. Through the `Closure::once` and some
        // `Rc`-trickery we can arrange for both instances of `Closure`, and the
        // `Rc`, to all be destroyed once the first one is called.
        let state = Rc::new(RefCell::new(Inner {
            result: None,
            task: None,
            callbacks: None,
        }));
        let state2 = state.clone();
        let resolve = Closure::once(move |val| finish(&state2, Ok(val)));
        let state2 = state.clone();
        let reject = Closure::once(move |val| finish(&state2, Err(val)));

        js.then2(&resolve, &reject);
        state.borrow_mut().callbacks = Some((resolve, reject));

        return JsFuture { inner: state };

        fn finish(state: &RefCell<Inner>, val: Result<JsValue, JsValue>) {
            // First up drop our closures as they'll never be invoked again and
            // this is our chance to clean up their state. Next store the value
            // into the internal state, and then finally if any task was waiting
            // on the value wake it up and let them know it's there.
            let task = {
                let mut state = state.borrow_mut();
                debug_assert!(state.callbacks.is_some());
                debug_assert!(state.result.is_none());
                drop(state.callbacks.take());
                state.result = Some(val);
                state.task.take()
            };
            if let Some(task) = task {
                task.wake()
            }
        }
    }
}

impl Future for JsFuture {
    type Output = Result<JsValue, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();
        // If our value has come in then we return it...
        if let Some(val) = inner.result.take() {
            return Poll::Ready(val)
        }
        // ... otherwise we arrange ourselves to get woken up once the value
        // does come in
        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// Converts a Rust `Future` on a local task queue.
///
/// The `future` provided must adhere to `'static` because it'll be scheduled
/// to run in the background and cannot contain any stack references.
///
/// # Panics
///
/// This function has the same panic behavior as `future_to_promise`.
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    crate::future_to_promise(async {
        future.await;
        Ok(JsValue::undefined())
    });
}

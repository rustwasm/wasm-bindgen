use std::cell::RefCell;
use futures::future;
use std::fmt;
use std::rc::Rc;
use futures::prelude::*;
use futures::sync::oneshot;
use js_sys::Promise;
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
    rx: oneshot::Receiver<Result<JsValue, JsValue>>,
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
        let (tx, rx) = oneshot::channel();
        let state = Rc::new(RefCell::new(None));
        let state2 = state.clone();
        let resolve = Closure::once(move |val| finish(&state2, Ok(val)));
        let state2 = state.clone();
        let reject = Closure::once(move |val| finish(&state2, Err(val)));

        js.then2(&resolve, &reject);
        *state.borrow_mut() = Some((tx, resolve, reject));

        return JsFuture { rx };

        fn finish(
            state: &RefCell<
                Option<(
                    oneshot::Sender<Result<JsValue, JsValue>>,
                    Closure<dyn FnMut(JsValue)>,
                    Closure<dyn FnMut(JsValue)>,
                )>,
            >,
            val: Result<JsValue, JsValue>,
        ) {
            match state.borrow_mut().take() {
                // We don't have any guarantee that anyone's still listening at this
                // point (the Rust `JsFuture` could have been dropped) so simply
                // ignore any errors here.
                Some((tx, _, _)) => drop(tx.send(val)),
                None => wasm_bindgen::throw_str("cannot finish twice"),
            }
        }
    }
}

impl Future for JsFuture {
    type Item = JsValue;
    type Error = JsValue;

    fn poll(&mut self) -> Poll<JsValue, JsValue> {
        match self.rx.poll() {
            Ok(Async::Ready(val)) => val.map(Async::Ready),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(_) => wasm_bindgen::throw_str("cannot cancel"),
        }
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
    F: Future<Item = (), Error = ()> + 'static,
{
    crate::future_to_promise(
        future
            .map(|()| JsValue::undefined())
            .or_else(|()| future::ok::<JsValue, JsValue>(JsValue::undefined())),
    );
}

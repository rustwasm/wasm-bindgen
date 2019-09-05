pub use crate::shared::{spawn_local, JsFuture};
use js_sys::Promise;
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Converts a Rust `Future` into a JavaScript `Promise`.
///
/// This function will take any future in Rust and schedule it to be executed,
/// returning a JavaScript `Promise` which can then be passed back to JavaScript
/// to get plumbed into the rest of a system.
///
/// The `future` provided must adhere to `'static` because it'll be scheduled
/// to run in the background and cannot contain any stack references. The
/// returned `Promise` will be resolved or rejected when the future completes,
/// depending on whether it finishes with `Ok` or `Err`.
///
/// # Panics
///
/// Note that in wasm panics are currently translated to aborts, but "abort" in
/// this case means that a JavaScript exception is thrown. The wasm module is
/// still usable (likely erroneously) after Rust panics.
///
/// If the `future` provided panics then the returned `Promise` **will not
/// resolve**. Instead it will be a leaked promise. This is an unfortunate
/// limitation of wasm currently that's hoped to be fixed one day!
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    _future_to_promise(Box::new(future))
}

fn _future_to_promise(future: Box<dyn Future<Output = Result<JsValue, JsValue>>>) -> Promise {
    let mut future = Some(future);
    js_sys::Promise::new(&mut move |resolve, reject| {
        let future = match future.take() {
            Some(f) => f,
            None => wasm_bindgen::throw_str("cannot invoke twice"),
        };
        let state = Rc::new(State {
            queued: Cell::new(true),
            future: RefCell::new(Some(Pin::from(future))),
            resolve,
            reject,
        });
        State::poll(&state);
    })
}

struct State {
    queued: Cell<bool>,
    future: RefCell<Option<Pin<Box<dyn Future<Output = Result<JsValue, JsValue>>>>>>,
    resolve: js_sys::Function,
    reject: js_sys::Function,
}

impl State {
    fn poll(me: &Rc<State>) {
        debug_assert!(me.queued.get());
        me.queued.set(false);
        let waker = unsafe { Waker::from_raw(State::into_raw_waker(me.clone())) };
        let mut future = me.future.borrow_mut();

        let mut done = false;
        if let Some(future) = &mut *future {
            match Future::poll(future.as_mut(), &mut Context::from_waker(&waker)) {
                Poll::Ready(Ok(val)) => {
                    me.resolve.call1(&JsValue::undefined(), &val).unwrap_throw();
                    done = true;
                }
                Poll::Ready(Err(val)) => {
                    me.reject.call1(&JsValue::undefined(), &val).unwrap_throw();
                    done = true;
                }
                Poll::Pending => {}
            }
        }
        if done {
            debug_assert!(future.is_some());
            drop(future.take());
            return;
        }
    }

    fn wake(me: &Rc<State>) {
        // If we're already enqueued on the microtask queue there's nothing else
        // to do, this is a duplicate notification.
        if me.queued.replace(true) {
            return;
        }

        // Use `Promise.then` on a resolved promise to place our execution
        // onto the next turn of the microtask queue, enqueueing our poll
        // operation. If we were to poll immediately we run the risk of blowing
        // the stack.
        let promise = Promise::resolve(&JsValue::undefined());
        let promise = promise.unchecked_ref::<MyPromise>();
        let me = me.clone();
        let closure = Closure::once_into_js(move || {
            State::poll(&me);
        });
        promise.then(&closure);

        #[wasm_bindgen]
        extern "C" {
            type MyPromise;
            #[wasm_bindgen(js_class = Promise, method)]
            fn then(this: &MyPromise, closure: &JsValue);
        }
    }

    unsafe fn into_raw_waker(me: Rc<State>) -> RawWaker {
        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);
        RawWaker::new(Rc::into_raw(me) as *const (), &VTABLE)
    }
}

unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
    let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const State));
    State::into_raw_waker((*ptr).clone())
}

unsafe fn raw_wake(ptr: *const ()) {
    let ptr = Rc::from_raw(ptr as *const State);
    State::wake(&ptr);
}

unsafe fn raw_wake_by_ref(ptr: *const ()) {
    let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const State));
    State::wake(&ptr);
}

unsafe fn raw_drop(ptr: *const ()) {
    drop(Rc::from_raw(ptr as *const State));
}

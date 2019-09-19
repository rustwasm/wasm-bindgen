use js_sys::Promise;
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::atomic::{AtomicI32, Ordering::SeqCst};
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Duplicate a bit here because `then` takes a `JsValue` instead of a `Closure`.
#[wasm_bindgen]
extern "C" {
    type MyPromise;
    #[wasm_bindgen(method)]
    fn then(this: &MyPromise, cb: &JsValue);

    type Atomics;
    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync)]
    fn wait_async(buf: &JsValue, index: i32, value: i32) -> js_sys::Promise;
    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync, getter)]
    fn get_wait_async() -> JsValue;
}

pub use crate::shared::{spawn_local, JsFuture};

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
        let state = Arc::new(State {
            value: AtomicI32::new(1),
        });
        Package {
            // Note that the unsafety should be ok here since we're always
            // passing in valid pointers and we're handling cleanup with
            // `Waker`.
            waker: unsafe { Waker::from_raw(State::into_raw_waker(state.clone())) },
            state,
            future: Pin::from(future),
            resolve,
            reject,
        }
        .poll();
    })
}

struct Package {
    state: Arc<State>,
    future: Pin<Box<dyn Future<Output = Result<JsValue, JsValue>>>>,
    resolve: js_sys::Function,
    reject: js_sys::Function,
    waker: Waker,
}

struct State {
    value: AtomicI32,
}

impl Package {
    fn poll(mut self) {
        // Flag ourselves as ready to receive another notification. We should
        // never enter this method unless our `value` is set to `1`, so assert
        // that as well.
        let prev = self.state.value.swap(0, SeqCst);
        debug_assert_eq!(prev, 1);

        // Afterwards start making progress on the future by calling the `poll`
        // function. If we get `Ready` then we simply invoke the appropriate JS
        // function to resolve the JS `Promise` we're connected to.
        //
        // If `Pending` is received then we're guaranteed to eventually receive
        // an `atomic.notify` as well as a store to `1` in `self.state.value`.
        // In this case we create a new promise (using `Atomics.waitAsync`) and
        // then we register that promise to continue polling when it's resolved.
        // Note that if a `wake` happened while we were polling or after we see
        // `Pending` then the promise should end up resolving immediately due to
        // the atomicity of `Atomics.waitAsync` with `Atomics.notify`.
        let mut cx = Context::from_waker(&self.waker);
        let (f, val) = match self.future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(val)) => (&self.resolve, val),
            Poll::Ready(Err(val)) => (&self.reject, val),

            // Create a `js_sys::Promise` using `Atomics.waitAsync` (or our
            // polyfill) and then register its completion callback as simply
            // calling this function again.
            Poll::Pending => {
                let promise = wait_async(&self.state.value, 0).unchecked_into::<MyPromise>();
                let closure = Closure::once_into_js(move || {
                    self.poll();
                });
                promise.then(&closure);
                return;
            }
        };
        f.call1(&JsValue::undefined(), &val).unwrap_throw();
    }
}

impl State {
    fn wake(&self) {
        // Attempt to notify us by storing 1. If we're already 1 then we were
        // previously notified and there's nothing to do. Otherwise we execute
        // the native `notify` instruction to wake up the corresponding
        // `waitAsync` that was waiting for the transition from 0 to 1.
        let prev = self.value.swap(1, SeqCst);
        if prev == 1 {
            return;
        }
        debug_assert_eq!(prev, 0);
        unsafe {
            core::arch::wasm32::atomic_notify(
                &self.value as *const AtomicI32 as *mut i32,
                1, // number of threads to notify
            );
        }
    }

    unsafe fn into_raw_waker(me: Arc<State>) -> RawWaker {
        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);
        RawWaker::new(Arc::into_raw(me) as *const (), &VTABLE)
    }
}

unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
    let ptr = ManuallyDrop::new(Arc::from_raw(ptr as *const State));
    State::into_raw_waker((*ptr).clone())
}

unsafe fn raw_wake(ptr: *const ()) {
    let ptr = Arc::from_raw(ptr as *const State);
    ptr.wake();
}

unsafe fn raw_wake_by_ref(ptr: *const ()) {
    let ptr = ManuallyDrop::new(Arc::from_raw(ptr as *const State));
    ptr.wake();
}

unsafe fn raw_drop(ptr: *const ()) {
    drop(Arc::from_raw(ptr as *const State));
}

fn wait_async(ptr: &AtomicI32, val: i32) -> js_sys::Promise {
    // If `Atomics.waitAsync` isn't defined (as it isn't defined anywhere today)
    // then we use our fallback, otherwise we use the native function.
    if Atomics::get_wait_async().is_undefined() {
        crate::wait_async_polyfill::wait_async(ptr, val)
    } else {
        let mem = wasm_bindgen::memory().unchecked_into::<js_sys::WebAssembly::Memory>();
        Atomics::wait_async(&mem.buffer(), ptr as *const AtomicI32 as i32 / 4, val)
    }
}

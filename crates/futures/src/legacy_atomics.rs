use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use futures::executor::{self, Notify, Spawn};
use futures::future;
use futures::prelude::*;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Duplicate a bit here because `then` takes a `JsValue` instead of a `Closure`.
#[wasm_bindgen]
extern "C" {
    type Promise;
    #[wasm_bindgen(method)]
    fn then(this: &Promise, cb: &JsValue) -> Promise;

    type Atomics;
    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync)]
    fn wait_async(buf: &JsValue, index: i32, value: i32) -> js_sys::Promise;
    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync, getter)]
    fn get_wait_async() -> JsValue;
}

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
pub fn future_to_promise<F>(future: F) -> js_sys::Promise
where
    F: Future<Item = JsValue, Error = JsValue> + 'static,
{
    _future_to_promise(Box::new(future))
}

// Implementation of actually transforming a future into a JavaScript `Promise`.
//
// The main primitives used here are `Promise::new` to actually create a JS
// promise to return as well as `Atomics.waitAsync` to create a promise that we
// can asynchronously wait on. The general idea here is that we'll create a
// promise to return and schedule work to happen in `Atomics.waitAsync`
// callbacks.
//
// After we've created a promise we start polling a future, and whenever it's
// not ready we'll execute `Atomics.waitAsync`. When that resolves we'll keep
// polling the future, and this happens until the future is done. Finally
// when it's all finished we call either resolver or reject depending on the
// result of the future.
fn _future_to_promise(future: Box<dyn Future<Item = JsValue, Error = JsValue>>) -> js_sys::Promise {
    let mut future = Some(executor::spawn(future));
    return js_sys::Promise::new(&mut |resolve, reject| {
        Package {
            spawn: future.take().unwrap(),
            resolve,
            reject,
            waker: Arc::new(Waker {
                value: AtomicI32::new(1), // 1 == "notified, ready to poll"
            }),
        }
        .poll();
    });

    struct Package {
        // Our "spawned future". This'll have everything we need to poll the
        // future and continue to move it forward.
        spawn: Spawn<Box<dyn Future<Item = JsValue, Error = JsValue>>>,

        // Our two callbacks connected to the `Promise` that we returned to
        // JavaScript.  We'll be invoking one of these at the end.
        resolve: Function,
        reject: Function,

        // Shared state used to communicate waking up this future, this is the
        // `Send + Sync` piece needed by the async task system.
        waker: Arc<Waker>,
    }

    struct Waker {
        value: AtomicI32,
    };

    impl Notify for Waker {
        fn notify(&self, _id: usize) {
            // Attempt to notify us by storing 1. If we're already 1 then we
            // were previously notified and there's nothing to do. Otherwise
            // we execute the native `notify` instruction to wake up the
            // corresponding `waitAsync` that was waiting for the transition
            // from 0 to 1.
            let prev = self.value.swap(1, Ordering::SeqCst);
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
    }

    impl Package {
        fn poll(mut self) {
            // Poll in a loop waiting for the future to become ready. Note that
            // we probably shouldn't maximize synchronous work here but rather
            // we should occasionally yield back to the runtime and schedule
            // ourselves to resume this future later on.
            //
            // Note that 0 here means "need a notification" and 1 means "we got
            // a notification". That means we're storing 0 into the `notified`
            // slot and we're trying to read 1 to keep on going.
            while self.waker.value.swap(0, Ordering::SeqCst) == 1 {
                let (val, f) = match self.spawn.poll_future_notify(&self.waker, 0) {
                    // If the future is ready, immediately call the
                    // resolve/reject callback and then return as we're done.
                    Ok(Async::Ready(value)) => (value, &self.resolve),
                    Err(value) => (value, &self.reject),

                    // ... otherwise let's break out and wait
                    Ok(Async::NotReady) => break,
                };

                // Call the resolution function, and then when we're done
                // destroy ourselves through `drop` since our future is no
                // longer needed.
                drop(f.call1(&JsValue::undefined(), &val));
                return;
            }

            // Create a `js_sys::Promise` using `Atomics.waitAsync` (or our
            // polyfill) and then register its completion callback as simply
            // calling this function again.
            let promise = wait_async(&self.waker.value, 0).unchecked_into::<Promise>();
            let closure = Closure::once_into_js(move || {
                self.poll();
            });
            promise.then(&closure);
        }
    }
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
    future_to_promise(
        future
            .map(|()| JsValue::undefined())
            .or_else(|()| future::ok::<JsValue, JsValue>(JsValue::undefined())),
    );
}

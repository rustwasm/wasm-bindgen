use std::fmt;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::future::Future;
use std::task::{Poll, Context};
use std::collections::VecDeque;

use futures_util::task::ArcWake;
use futures_util::future::FutureExt;
use futures_channel::oneshot;

use lazy_static::lazy_static;

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
    resolved: oneshot::Receiver<JsValue>,
    rejected: oneshot::Receiver<JsValue>,
    _cb_resolve: Closure<FnMut(JsValue)>,
    _cb_reject: Closure<FnMut(JsValue)>,
}

impl fmt::Debug for JsFuture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JsFuture {{ ... }}")
    }
}

impl From<Promise> for JsFuture {
    fn from(js: Promise) -> JsFuture {
        // Use the `then` method to schedule two callbacks, one for the
        // resolved value and one for the rejected value. These two callbacks
        // will be connected to oneshot channels which feed back into our
        // future.
        //
        // This may not be the speediest option today but it should work!
        let (tx1, rx1) = oneshot::channel();

        let cb_resolve = Closure::once(move |val| {
            tx1.send(val).unwrap_throw();
        });

        let (tx2, rx2) = oneshot::channel();

        let cb_reject = Closure::once(move |val| {
            tx2.send(val).unwrap_throw();
        });

        js.then2(&cb_resolve, &cb_reject);

        JsFuture {
            resolved: rx1,
            rejected: rx2,
            _cb_resolve: cb_resolve,
            _cb_reject: cb_reject,
        }
    }
}

impl Future for JsFuture {
    type Output = Result<JsValue, JsValue>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Test if either our resolved or rejected side is finished yet.
        if let Poll::Ready(val) = self.resolved.poll_unpin(cx) {
            return Poll::Ready(Ok(val.unwrap_throw()));
        }

        if let Poll::Ready(val) = self.rejected.poll_unpin(cx) {
            return Poll::Ready(Err(val.unwrap_throw()));
        }

        Poll::Pending
    }
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
pub fn future_to_promise<F>(future: F) -> Promise
where
    F: Future<Output = Result<JsValue, JsValue>> + 'static,
{
    let mut future = Some(future);

    Promise::new(&mut |resolve, reject| {
        // TODO change Promise::new to be FnOnce
        spawn_local(future.take().unwrap_throw().map(move |val| {
            match val {
                Ok(val) => {
                    resolve.call1(&JsValue::undefined(), &val).unwrap_throw();
                },
                Err(val) => {
                    reject.call1(&JsValue::undefined(), &val).unwrap_throw();
                },
            }
        }));
    })
}

/// Runs a Rust `Future` on a local task queue.
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
    struct Task {
        future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>,
        is_queued: AtomicBool,
    }

    impl Task {
        #[inline]
        fn new<F>(future: F) -> Arc<Self> where F: Future<Output = ()> + 'static {
            Arc::new(Self {
                future: Mutex::new(Some(Box::pin(future))),
                is_queued: AtomicBool::new(false),
            })
        }
    }

    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            // TODO can this be more relaxed ?
            if !arc_self.is_queued.swap(true, Ordering::SeqCst) {
                let mut lock = EXECUTOR.tasks.lock().unwrap_throw();

                lock.push_back(arc_self.clone());

                EXECUTOR.next_tick.schedule();
            }
        }
    }


    struct NextTick {
        is_spinning: AtomicBool,
        promise: Promise,
        closure: Closure<dyn FnMut(JsValue)>,
    }

    impl NextTick {
        fn new<F>(mut f: F) -> Self where F: FnMut() + 'static {
            Self {
                is_spinning: AtomicBool::new(false),
                promise: Promise::resolve(&JsValue::null()),
                closure: Closure::wrap(Box::new(move |_| {
                    f();
                })),
            }
        }

        fn schedule(&self) {
            // TODO can this be more relaxed ?
            if !self.is_spinning.swap(true, Ordering::SeqCst) {
                // TODO avoid creating a new Promise
                self.promise.then(&self.closure);
            }
        }

        fn done(&self) {
            // TODO can this be more relaxed ?
            self.is_spinning.store(false, Ordering::SeqCst);
        }
    }


    struct Executor {
        tasks: Mutex<VecDeque<Arc<Task>>>,
        next_tick: NextTick,
    }

    // This is only safe because JS is currently single-threaded
    unsafe impl Send for Executor {}
    unsafe impl Sync for Executor {}

    lazy_static! {
        static ref EXECUTOR: Executor = Executor {
            tasks: Mutex::new(VecDeque::new()),
            next_tick: NextTick::new(|| {
                let tasks = &EXECUTOR.tasks;

                loop {
                    let mut lock = tasks.lock().unwrap_throw();

                    match lock.pop_front() {
                        Some(task) => {
                            // This is necessary because the polled task might queue more tasks
                            drop(lock);

                            let mut future = task.future.lock().unwrap_throw();

                            let poll = future.as_mut().map(|mut future| {
                                // Clear `is_queued` flag so that it will re-queue if poll calls waker.wake()
                                task.is_queued.store(false, Ordering::SeqCst);

                                // TODO is there some way of saving these so they don't need to be recreated all the time ?
                                let waker = ArcWake::into_waker(task.clone());
                                let cx = &mut Context::from_waker(&waker);
                                Pin::new(&mut future).poll(cx)
                            });

                            if let Some(Poll::Ready(_)) = poll {
                                *future = None;
                            }
                        },
                        None => {
                            EXECUTOR.next_tick.done();
                            break;
                        },
                    }
                }
            }),
        };
    }


    ArcWake::wake_by_ref(&Task::new(future));
}

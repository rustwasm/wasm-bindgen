use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_channel::oneshot;
use futures_util::future::FutureExt;
use futures_util::task::ArcWake;

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
    _cb_resolve: Closure<dyn FnMut(JsValue)>,
    _cb_reject: Closure<dyn FnMut(JsValue)>,
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
        spawn_local(future.take().unwrap_throw().map(move |val| match val {
            Ok(val) => {
                resolve.call1(&JsValue::undefined(), &val).unwrap_throw();
            }
            Err(val) => {
                reject.call1(&JsValue::undefined(), &val).unwrap_throw();
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
        // This is an Option so that the Future can be immediately dropped when it's finished
        future: RefCell<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>,

        // This is used to ensure that the Task will only be queued once
        is_queued: Cell<bool>,
    }

    // TODO This is only safe because JS is currently single-threaded
    unsafe impl Send for Task {}
    unsafe impl Sync for Task {}

    impl Task {
        #[inline]
        fn new<F>(future: F) -> Arc<Self>
        where
            F: Future<Output = ()> + 'static,
        {
            Arc::new(Self {
                future: RefCell::new(Some(Box::pin(future))),
                is_queued: Cell::new(false),
            })
        }
    }

    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            // This ensures that it's only queued once
            if arc_self.is_queued.replace(true) {
                return;
            }

            let mut lock = EXECUTOR.tasks.borrow_mut();

            lock.push_back(arc_self.clone());

            // The Task will be polled on the next microtask event tick
            EXECUTOR.next_tick.schedule();
        }
    }

    struct NextTick {
        is_spinning: Cell<bool>,
        promise: Promise,
        closure: Closure<dyn FnMut(JsValue)>,
    }

    impl NextTick {
        #[inline]
        fn new<F>(mut f: F) -> Self
        where
            F: FnMut() + 'static,
        {
            Self {
                is_spinning: Cell::new(false),
                promise: Promise::resolve(&JsValue::null()),
                closure: Closure::wrap(Box::new(move |_| {
                    f();
                })),
            }
        }

        fn schedule(&self) {
            // This ensures that it's only scheduled once
            if self.is_spinning.replace(true) {
                return;
            }

            // TODO avoid creating a new Promise
            self.promise.then(&self.closure);
        }

        fn done(&self) {
            self.is_spinning.set(false);
        }
    }

    struct Executor {
        // This is a queue of Tasks which will be polled in order
        tasks: RefCell<VecDeque<Arc<Task>>>,

        // This is used to ensure that Tasks are polled on the next microtask event tick
        next_tick: NextTick,
    }

    // TODO This is only safe because JS is currently single-threaded
    unsafe impl Send for Executor {}
    unsafe impl Sync for Executor {}

    lazy_static! {
        static ref EXECUTOR: Executor = Executor {
            tasks: RefCell::new(VecDeque::new()),

            // This closure will only be called on the next microtask event tick
            next_tick: NextTick::new(|| {
                let tasks = &EXECUTOR.tasks;

                loop {
                    let mut lock = tasks.borrow_mut();

                    match lock.pop_front() {
                        Some(task) => {
                            let mut borrow = task.future.borrow_mut();

                            // This will only be None if the Future wakes up the Waker after returning Poll::Ready
                            if let Some(future) = borrow.as_mut() {
                                let poll = {
                                    // Clear `is_queued` flag so that it will re-queue if poll calls waker.wake()
                                    task.is_queued.set(false);

                                    // This is necessary because the polled task might queue more tasks
                                    drop(lock);

                                    // TODO is there some way of saving these so they don't need to be recreated all the time ?
                                    let waker = ArcWake::into_waker(task.clone());
                                    let cx = &mut Context::from_waker(&waker);
                                    Pin::new(future).poll(cx)
                                };

                                if let Poll::Ready(_) = poll {
                                    // Cleanup the Future immediately
                                    *borrow = None;
                                }
                            }
                        },
                        None => {
                            // All of the Tasks have been polled, so it's now possible to schedule the NextTick again
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

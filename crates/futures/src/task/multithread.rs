use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::SeqCst;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    type Atomics;

    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync)]
    fn wait_async(buf: &JsValue, index: i32, value: i32) -> js_sys::Promise;

    #[wasm_bindgen(static_method_of = Atomics, js_name = waitAsync, getter)]
    fn get_wait_async() -> JsValue;
}

fn wait_async(ptr: &AtomicI32, current_value: i32) -> js_sys::Promise {
    // If `Atomics.waitAsync` isn't defined (as it isn't defined anywhere today)
    // then we use our fallback, otherwise we use the native function.
    if Atomics::get_wait_async().is_undefined() {
        crate::task::wait_async_polyfill::wait_async(ptr, current_value)

    } else {
        let mem = wasm_bindgen::memory().unchecked_into::<js_sys::WebAssembly::Memory>();
        Atomics::wait_async(&mem.buffer(), ptr as *const AtomicI32 as i32 / 4, current_value)
    }
}

fn notify(atomic: &AtomicI32, wakeup: u32) {
    unsafe {
        core::arch::wasm32::atomic_notify(
            atomic as *const AtomicI32 as *mut i32,
            wakeup, // Number of threads to notify
        );
    }
}


const SLEEPING: i32 = 0;
const AWAKE: i32 = 1;

struct AtomicWaker {
    state: AtomicI32,
}

impl AtomicWaker {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            state: AtomicI32::new(AWAKE),
        })
    }

    // If we're already AWAKE then we previously notified and there's nothing to do.
    // Otherwise we execute the native `notify` instruction to wake up the corresponding
    // `waitAsync` that was waiting for the transition from SLEEPING to AWAKE.
    fn wake_by_ref(this: &Arc<Self>) {
        let prev = this.state.swap(AWAKE, SeqCst);

        if prev == AWAKE {
            return;
        }

        debug_assert_eq!(prev, SLEEPING);

        notify(&this.state, 1);
    }

    // This is to avoid a dependency on futures_util::task::ArcWake
    unsafe fn into_raw_waker(this: Arc<Self>) -> RawWaker {
        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Arc::from_raw(ptr as *const AtomicWaker));
            AtomicWaker::into_raw_waker((*ptr).clone())
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Arc::from_raw(ptr as *const AtomicWaker);
            AtomicWaker::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Arc::from_raw(ptr as *const AtomicWaker));
            AtomicWaker::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Arc::from_raw(ptr as *const AtomicWaker));
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Arc::into_raw(this) as *const (), &VTABLE)
    }
}


struct Inner {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    closure: Closure<dyn FnMut(JsValue)>,
}

pub(crate) struct Task {
    atomic: Arc<AtomicWaker>,
    waker: Waker,
    // This is an Option so that the Future and Closure can be immediately dropped when it's finished
    inner: RefCell<Option<Inner>>,
}

impl Task {
    pub(crate) fn spawn(future: Pin<Box<dyn Future<Output = ()> + 'static>>) {
        let atomic = AtomicWaker::new();

        let waker = unsafe { Waker::from_raw(AtomicWaker::into_raw_waker(Arc::clone(&atomic))) };

        let this = Rc::new(Task {
            atomic,
            waker,
            inner: RefCell::new(None),
        });

        let closure = {
            let this = Rc::clone(&this);

            Closure::wrap(Box::new(move |_| Task::run(&this)) as Box<dyn FnMut(JsValue)>)
        };

        *this.inner.borrow_mut() = Some(Inner { future, closure });

        crate::queue::QUEUE.with(move |queue| {
            // This will cause it to call crate::queue::Task::run on the next microtask tick
            queue.push_task(this);
        });
    }

    pub(crate) fn run(&self) {
        let mut borrow = self.inner.borrow_mut();

        // This will only be None if the Future wakes up the Waker after returning Poll::Ready
        if let Some(inner) = borrow.as_mut() {
            // Flag ourselves as ready to receive another notification.
            // This has to be at the top so that it will re-queue if poll calls waker.wake()
            let prev = self.atomic.state.swap(SLEEPING, SeqCst);

            // We should never enter this method unless our `value` is set to `AWAKE`, so assert
            // that as well.
            debug_assert_eq!(prev, AWAKE);

            let poll = {
                let cx = &mut Context::from_waker(&self.waker);
                inner.future.as_mut().poll(cx)
            };

            match poll {
                Poll::Ready(()) => {
                    // Cleanup the Future and Closure immediately
                    *borrow = None;
                },
                Poll::Pending => {
                    // If `self.state` is `SLEEPING` then it will wait until it is notified
                    // If `self.state` is `AWAKE` then it immediately resolves the Promise
                    wait_async(&self.atomic.state, SLEEPING).then(&inner.closure);
                },
            }
        }
    }
}

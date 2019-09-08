use std::cell::{Cell, RefCell};
use std::future::Future;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use crate::queue::Queue;


thread_local! {
    static QUEUE: Queue<Rc<Task>> = Queue::new();
}


struct Inner {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

pub(crate) struct Task {
    // This is an Option so that the Future can be immediately dropped when it's finished
    inner: RefCell<Option<Inner>>,

    // This is used to ensure that the Task will only be queued once
    is_queued: Cell<bool>,
}

impl Task {
    pub(crate) fn spawn(future: Pin<Box<dyn Future<Output = ()> + 'static>>) {
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            is_queued: Cell::new(false),
        });

        let waker = unsafe { Waker::from_raw(Task::into_raw_waker(Rc::clone(&this))) };

        *this.inner.borrow_mut() = Some(Inner {
            future,
            waker,
        });

        Task::wake_by_ref(&this);
    }

    fn wake_by_ref(this: &Rc<Self>) {
        // This ensures that it's only queued once
        if this.is_queued.replace(true) {
            return;
        }

        QUEUE.with(|queue| {
            queue.push_task(Rc::clone(this));
        });
    }

    // This is to avoid a dependency on futures_util::task::ArcWake
    unsafe fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::into_raw_waker((*ptr).clone())
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Rc::from_raw(ptr as *const Task);
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Rc::from_raw(ptr as *const Task));
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this) as *const (), &VTABLE)
    }
}

impl crate::queue::Task for Rc<Task> {
    fn run(self) {
        let mut borrow = self.inner.borrow_mut();

        // This will only be None if the Future wakes up the Waker after returning Poll::Ready
        if let Some(inner) = borrow.as_mut() {
            // Clear `is_queued` flag so that it will re-queue if poll calls waker.wake()
            self.is_queued.set(false);

            let poll = {
                let cx = &mut Context::from_waker(&inner.waker);
                inner.future.as_mut().poll(cx)
            };

            if let Poll::Ready(_) = poll {
                // Cleanup the Future immediately
                *borrow = None;
            }
        }
    }
}

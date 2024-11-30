use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn queueMicrotask(closure: &Closure<dyn FnMut(JsValue)>);

    type Global;

    #[wasm_bindgen(method, getter, js_name = queueMicrotask)]
    fn hasQueueMicrotask(this: &Global) -> JsValue;
}

struct QueueState {
    // The queue of Tasks which are to be run in order. In practice this is all the
    // synchronous work of futures, and each `Task` represents calling `poll` on
    // a future "at the right time".
    tasks: RefCell<VecDeque<Rc<crate::task::Task>>>,

    // This flag indicates whether we've scheduled `run_all` to run in the future.
    // This is used to ensure that it's only scheduled once.
    is_scheduled: Cell<bool>,
}

impl QueueState {
    fn run_all(&self) {
        // "consume" the schedule
        let _was_scheduled = self.is_scheduled.replace(false);
        debug_assert!(_was_scheduled);

        // Stop when all tasks that have been scheduled before this tick have been run.
        // Tasks that are scheduled while running tasks will run on the next tick.
        let mut task_count_left = self.tasks.borrow().len();
        while task_count_left > 0 {
            task_count_left -= 1;
            let task = match self.tasks.borrow_mut().pop_front() {
                Some(task) => task,
                None => break,
            };
            task.run();
        }

        // All of the Tasks have been run, so it's now possible to schedule the
        // next tick again
    }
}

pub(crate) struct Queue {
    state: Rc<QueueState>,
    promise: Promise,
    closure: Closure<dyn FnMut(JsValue)>,
    has_queue_microtask: bool,
}

impl Queue {
    // Schedule a task to run on the next tick
    pub(crate) fn schedule_task(&self, task: Rc<crate::task::Task>) {
        self.state.tasks.borrow_mut().push_back(task);
        // Use queueMicrotask to execute as soon as possible. If it does not exist
        // fall back to the promise resolution
        if !self.state.is_scheduled.replace(true) {
            if self.has_queue_microtask {
                queueMicrotask(&self.closure);
            } else {
                let _ = self.promise.then(&self.closure);
            }
        }
    }
    // Append a task to the currently running queue, or schedule it
    #[cfg(not(target_feature = "atomics"))]
    pub(crate) fn push_task(&self, task: Rc<crate::task::Task>) {
        // It would make sense to run this task on the same tick.  For now, we
        // make the simplifying choice of always scheduling tasks for a future tick.
        self.schedule_task(task)
    }
}

impl Queue {
    fn new() -> Self {
        let state = Rc::new(QueueState {
            is_scheduled: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        });

        let has_queue_microtask = js_sys::global()
            .unchecked_into::<Global>()
            .hasQueueMicrotask()
            .is_function();

        Self {
            promise: Promise::resolve(&JsValue::undefined()),

            closure: {
                let state = Rc::clone(&state);

                // This closure will only be called on the next microtask event
                // tick
                Closure::new(move |_| state.run_all())
            },

            state,
            has_queue_microtask,
        }
    }

    #[cfg(feature = "std")]
    pub(crate) fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        thread_local! {
            static QUEUE: Queue = Queue::new();
        }

        QUEUE.with(f)
    }

    #[cfg(not(feature = "std"))]
    pub(crate) fn with<R>(f: impl FnOnce(&Self) -> R) -> R {
        use once_cell::unsync::Lazy;

        struct Wrapper<T>(Lazy<T>);

        #[cfg(not(target_feature = "atomics"))]
        unsafe impl<T> Sync for Wrapper<T> {}

        #[cfg(not(target_feature = "atomics"))]
        unsafe impl<T> Send for Wrapper<T> {}

        #[cfg_attr(target_feature = "atomics", thread_local)]
        static QUEUE: Wrapper<Queue> = Wrapper(Lazy::new(Queue::new));

        f(&QUEUE.0)
    }
}

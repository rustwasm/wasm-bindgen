use js_sys::Promise;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::mem;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

struct QueueState {
    // The queue of Tasks which are currently run in order. In practice this is all the
    // synchronous work of futures, and each `Task` represents calling `poll` on
    // a future "at the right time". If we're not currently in run_all, this should be empty.
    front_tasks: RefCell<VecDeque<Rc<crate::task::Task>>>,
    // We use a double-buffer approach to running tasks. Tasks initially enter the
    // back_tasks which gets swapped to the front when we resume spinning.
    back_tasks: RefCell<VecDeque<Rc<crate::task::Task>>>,

    // This flag indicates whether we've scheduled `run_all` to run in the future.
    // This is used to ensure that it's only scheduled once.
    is_scheduled: Cell<bool>,
    // This flag indicates whether we're currently executing inside of `run_all`.
    is_spinning: Cell<bool>,
}

impl QueueState {
    fn run_all(&self) {
        // "consume" the schedule
        let _was_scheduled = self.is_scheduled.replace(false);
        debug_assert!(_was_scheduled);
        // start spinning
        self.is_spinning.set(true);
        // swap queues
        debug_assert!(
            self.front_tasks.borrow().is_empty(),
            "shouldn't swap tasks out of the front queue"
        );
        mem::swap(
            &mut *self.front_tasks.borrow_mut(),
            &mut *self.back_tasks.borrow_mut(),
        );

        // Runs all Tasks until empty. This blocks the event loop if a Future is
        // stuck in an infinite loop, so we may want to yield back to the main
        // event loop occasionally. For now though greedy execution should get
        // the job done.
        loop {
            let task = match self.front_tasks.borrow_mut().pop_front() {
                Some(task) => task,
                None => break,
            };
            task.run();
        }

        // All of the Tasks have been run, so it's now possible to schedule the
        // next tick again
        self.is_spinning.set(false);
    }
}

pub(crate) struct Queue {
    state: Rc<QueueState>,
    promise: Promise,
    closure: Closure<dyn FnMut(JsValue)>,
}

impl Queue {
    // Schedule a task to run on the next tick
    pub(crate) fn schedule_task(&self, task: Rc<crate::task::Task>) {
        self.state.back_tasks.borrow_mut().push_back(task);
        // Note that we currently use a promise and a closure to do this, but
        // eventually we should probably use something like `queueMicrotask`:
        // https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/queueMicrotask
        if !self.state.is_scheduled.replace(true) {
            let _ = self.promise.then(&self.closure);
        }
    }
    // Append a task to the currently running queue, or schedule it
    pub(crate) fn push_task(&self, task: Rc<crate::task::Task>) {
        if self.state.is_spinning.get() {
            // If we're already inside the `run_all` loop then that'll pick up the
            // task we just enqueued. If we're not in `run_all`, though, then we need
            // to schedule a microtask.
            self.state.front_tasks.borrow_mut().push_back(task);
        } else {
            self.schedule_task(task);
        }
    }
}

impl Queue {
    fn new() -> Self {
        let state = Rc::new(QueueState {
            is_spinning: Cell::new(false),
            is_scheduled: Cell::new(false),
            front_tasks: RefCell::new(VecDeque::new()),
            back_tasks: RefCell::new(VecDeque::new()),
        });

        Self {
            promise: Promise::resolve(&JsValue::undefined()),

            closure: {
                let state = Rc::clone(&state);

                // This closure will only be called on the next microtask event
                // tick
                Closure::wrap(Box::new(move |_| state.run_all()))
            },

            state,
        }
    }
}

thread_local! {
    pub(crate) static QUEUE: Queue = Queue::new();
}

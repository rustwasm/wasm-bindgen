use js_sys::Promise;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;


pub(crate) trait Task {
    fn run(self);
}


struct QueueState<A> {
    // This is used to ensure that it's only scheduled once
    is_spinning: Cell<bool>,

    // This is a queue of Tasks which will be run in order
    tasks: RefCell<VecDeque<A>>,
}

impl<A> QueueState<A> where A: Task {
    fn run_all(&self) {
        loop {
            // We immediately drop the borrow_mut because `run` might queue more tasks
            let task = self.tasks.borrow_mut().pop_front();

            match task {
                Some(task) => {
                    task.run();
                },
                None => {
                    // All of the Tasks have been run, so it's now possible to schedule the next tick again
                    self.is_spinning.set(false);
                    break;
                },
            }
        }
    }
}


pub(crate) struct Queue<A> {
    state: Rc<QueueState<A>>,
    promise: Promise,
    closure: Closure<dyn FnMut(JsValue)>,
}

impl<A> Queue<A> {
    pub(crate) fn push_task(&self, task: A) {
        let mut lock = self.state.tasks.borrow_mut();

        lock.push_back(task);

        // If we already scheduled the next tick then do nothing
        if self.state.is_spinning.replace(true) {
            return;
        }

        // The Task will be run on the next microtask event tick
        // TODO replace with https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope/queueMicrotask
        self.promise.then(&self.closure);
    }
}

impl<A> Queue<A> where A: Task + 'static {
    pub(crate) fn new() -> Self {
        let state = Rc::new(QueueState {
            is_spinning: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        });

        Self {
            promise: Promise::resolve(&JsValue::null()),

            closure: {
                let state = Rc::clone(&state);

                // This closure will only be called on the next microtask event tick
                Closure::wrap(Box::new(move |_| {
                    state.run_all();
                }))
            },

            state,
        }
    }
}

use crate::controller::Controller;
use crate::exit;
use crate::view::View;
use crate::Message;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates an event loop that starts each time a message is added
pub struct Scheduler {
    controller: Rc<RefCell<Option<Controller>>>,
    view: Rc<RefCell<Option<View>>>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>,
}

fn deadlock() -> ! {
    exit("This might be a deadlock");
}

impl Scheduler {
    /// Construct a new `Scheduler`
    pub fn new() -> Scheduler {
        Scheduler {
            controller: Rc::new(RefCell::new(None)),
            view: Rc::new(RefCell::new(None)),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false),
        }
    }

    pub fn set_controller(&self, controller: Controller) {
        let Ok(mut controller_data) = self.controller.try_borrow_mut() else { deadlock() };
        *controller_data = Some(controller);
    }

    pub fn set_view(&self, view: View) {
        let Ok(mut view_data) = self.view.try_borrow_mut() else { deadlock() };
        *view_data = Some(view);
    }

    /// Add a new message onto the event stack
    ///
    /// Triggers running the event loop if it's not already running
    pub fn add_message(&self, message: Message) {
        let running = {
            let Ok(running) = self.running.try_borrow() else { deadlock() };
            *running
        };
        {
            let Ok(mut events) = self.events.try_borrow_mut() else { deadlock() };
            events.push(message);
        }
        if !running {
            self.run();
        }
    }

    /// Start the event loop, taking messages from the stack to run
    fn run(&self) {
        let events_len = {
            let Ok(events) = self.events.try_borrow() else { deadlock() };
            events.len()
        };
        if events_len == 0 {
            let Ok(mut running) = self.running.try_borrow_mut() else { deadlock() };
            *running = false;
        } else {
            {
                let Ok(mut running) = self.running.try_borrow_mut() else { deadlock() };
                *running = true;
            }
            self.next_message();
        }
    }

    fn next_message(&self) {
        let event = {
            let Ok(mut events) = self.events.try_borrow_mut() else { deadlock() };
            events.pop()
        };
        if let Some(event) = event {
            match event {
                Message::Controller(e) => {
                    let Ok(mut controller) = self.controller.try_borrow_mut() else { deadlock() };
                    if let Some(ref mut ag) = *controller {
                        ag.call(e);
                    }
                }
                Message::View(e) => {
                    let Ok(mut view) = self.view.try_borrow_mut() else { deadlock() };
                    if let Some(ref mut ag) = *view {
                        ag.call(e);
                    }
                }
            }
            self.run();
        } else {
            let Ok(mut running) = self.running.try_borrow_mut() else { deadlock() };
            *running = false;
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        exit("calling drop on Scheduler");
    }
}

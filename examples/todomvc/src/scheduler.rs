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
        if let Ok(mut controller_data) = self.controller.try_borrow_mut() {
            *controller_data = Some(controller);
        } else {
            exit("This might be a deadlock");
        }
    }

    pub fn set_view(&self, view: View) {
        if let Ok(mut view_data) = self.view.try_borrow_mut() {
            *view_data = Some(view);
        } else {
            exit("This might be a deadlock");
        }
    }

    /// Add a new message onto the event stack
    ///
    /// Triggers running the event loop if it's not already running
    pub fn add_message(&self, message: Message) {
        let running = {
            if let Ok(running) = self.running.try_borrow() {
                running.clone()
            } else {
                exit("This might be a deadlock");
                false
            }
        };
        {
            if let Ok(mut events) = self.events.try_borrow_mut() {
                events.push(message);
            } else {
                exit("This might be a deadlock");
            }
        }
        if !running {
            self.run();
        }
    }

    /// Start the event loop, taking messages from the stack to run
    fn run(&self) {
        let mut events_len = 0;
        {
            if let Ok(events) = self.events.try_borrow() {
                events_len = events.len().clone();
            } else {
                exit("This might be a deadlock");
            }
        }
        if events_len == 0 {
            if let Ok(mut running) = self.running.try_borrow_mut() {
                *running = false;
            } else {
                exit("This might be a deadlock");
            }
        } else {
            {
                if let Ok(mut running) = self.running.try_borrow_mut() {
                    *running = true;
                } else {
                    exit("This might be a deadlock");
                }
            }
            self.next_message();
        }
    }

    fn next_message(&self) {
        let event = {
            if let Ok(mut events) = self.events.try_borrow_mut() {
                Some(events.pop())
            } else {
                exit("This might be a deadlock");
                None
            }
        };
        if let Some(Some(event)) = event {
            match event {
                Message::Controller(e) => {
                    if let Ok(mut controller) = self.controller.try_borrow_mut() {
                        if let Some(ref mut ag) = *controller {
                            ag.call(e);
                        }
                    } else {
                        exit("This might be a deadlock");
                    }
                }
                Message::View(e) => {
                    if let Ok(mut view) = self.view.try_borrow_mut() {
                        if let Some(ref mut ag) = *view {
                            ag.call(e);
                        }
                    } else {
                        exit("This might be a deadlock");
                    }
                }
            }
            self.run();
        } else if let Ok(mut running) = self.running.try_borrow_mut() {
            *running = false;
        } else {
            exit("This might be a deadlock");
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        exit("calling drop on Scheduler");
    }
}

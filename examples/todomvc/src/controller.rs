use crate::exit;
use crate::store::*;
use crate::view::ViewMessage;
use crate::{Message, Scheduler};
use js_sys::Date;

use std::cell::RefCell;
use std::rc::Weak;

/// The controller of the application turns page state into functionality
pub struct Controller {
    store: Store,
    sched: RefCell<Option<Weak<Scheduler>>>,
    active_route: String,
    last_active_route: String,
}

/// Messages that represent the methods to be called on the Controller
pub enum ControllerMessage {
    AddItem(String),
    SetPage(String),
    EditItemSave(String, String),
    ToggleItem(String, bool),
    EditItemCancel(String),
    RemoveCompleted(),
    RemoveItem(String),
    ToggleAll(bool),
}

impl Controller {
    pub fn new(store: Store, sched: Weak<Scheduler>) -> Controller {
        Controller {
            store,
            sched: RefCell::new(Some(sched)),
            active_route: "".into(),
            last_active_route: "none".into(),
        }
    }

    /// Used by `Scheduler` to convert a `ControllerMessage` into a function call on a `Controller`
    pub fn call(&mut self, method_name: ControllerMessage) {
        use self::ControllerMessage::*;
        match method_name {
            AddItem(title) => self.add_item(title),
            SetPage(hash) => self.set_page(hash),
            EditItemSave(id, value) => self.edit_item_save(id, value),
            EditItemCancel(id) => self.edit_item_cancel(id),
            RemoveCompleted() => self.remove_completed_items(),
            RemoveItem(id) => self.remove_item(&id),
            ToggleAll(completed) => self.toggle_all(completed),
            ToggleItem(id, completed) => self.toggle_item(id, completed),
        }
    }

    fn toggle_item(&mut self, id: String, completed: bool) {
        self.toggle_completed(id, completed);
        self._filter(completed);
    }

    fn add_message(&self, view_message: ViewMessage) {
        if let Ok(sched) = self.sched.try_borrow_mut() {
            if let Some(ref sched) = *sched {
                if let Some(sched) = sched.upgrade() {
                    sched.add_message(Message::View(view_message));
                }
            }
        }
    }

    pub fn set_page(&mut self, raw: String) {
        let route = raw.trim_start_matches("#/");
        self.active_route = route.to_string();
        self._filter(false);
        self.add_message(ViewMessage::UpdateFilterButtons(route.to_string()));
    }

    /// Add an Item to the Store and display it in the list.
    fn add_item(&mut self, title: String) {
        self.store.insert(Item {
            id: Date::now().to_string(),
            title,
            completed: false,
        });
        self.add_message(ViewMessage::ClearNewTodo());
        self._filter(true);
    }

    /// Save an Item in edit.
    fn edit_item_save(&mut self, id: String, title: String) {
        if !title.is_empty() {
            self.store.update(ItemUpdate::Title {
                id: id.clone(),
                title: title.clone(),
            });
            self.add_message(ViewMessage::EditItemDone(id.to_string(), title));
        } else {
            self.remove_item(&id);
        }
    }

    /// Cancel the item editing mode.
    fn edit_item_cancel(&mut self, id: String) {
        let mut message = None;
        if let Some(data) = self.store.find(ItemQuery::Id { id: id.clone() }) {
            if let Some(todo) = data.get(0) {
                let title = todo.title.to_string();
                message = Some(ViewMessage::EditItemDone(id, title));
            }
        }
        if let Some(message) = message {
            self.add_message(message);
        }
    }

    /// Remove the data and elements related to an Item.
    fn remove_item(&mut self, id: &String) {
        self.store.remove(ItemQuery::Id { id: id.clone() });
        self._filter(false);
        let ritem = id.to_string();
        self.add_message(ViewMessage::RemoveItem(ritem));
    }

    /// Remove all completed items.
    fn remove_completed_items(&mut self) {
        self.store.remove(ItemQuery::Completed { completed: true });
        self._filter(true);
    }

    /// Update an Item in storage based on the state of completed.
    fn toggle_completed(&mut self, id: String, completed: bool) {
        self.store.update(ItemUpdate::Completed {
            id: id.clone(),
            completed,
        });
        let tid = id;
        self.add_message(ViewMessage::SetItemComplete(tid, completed));
    }

    /// Set all items to complete or active.
    fn toggle_all(&mut self, completed: bool) {
        let mut vals = Vec::new();
        if let Some(data) = self.store.find(ItemQuery::EmptyItemQuery) {
            vals.extend(data.iter().map(|item| item.id.clone()));
        }
        for id in vals.iter() {
            self.toggle_completed(id.to_string(), completed);
        }
        self._filter(false);
    }

    /// Refresh the list based on the current route.
    fn _filter(&mut self, force: bool) {
        let route = &self.active_route;

        if force || !self.last_active_route.is_empty() || &self.last_active_route != route {
            let query = match route.as_str() {
                "completed" => ItemQuery::Completed { completed: true },
                "active" => ItemQuery::Completed { completed: false },
                _ => ItemQuery::EmptyItemQuery,
            };
            let mut v = None;
            {
                let store = &mut self.store;
                if let Some(res) = store.find(query) {
                    v = Some(res.into());
                }
            }
            if let Some(res) = v {
                self.add_message(ViewMessage::ShowItems(res));
            }
        }

        if let Some((total, active, completed)) = self.store.count() {
            self.add_message(ViewMessage::SetItemsLeft(active));
            self.add_message(ViewMessage::SetClearCompletedButtonVisibility(
                completed > 0,
            ));
            self.add_message(ViewMessage::SetCompleteAllCheckbox(completed == total));
            self.add_message(ViewMessage::SetMainVisibility(total > 0));
        }

        self.last_active_route = route.to_string();
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        exit("calling drop on Controller");
    }
}

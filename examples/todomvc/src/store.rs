use js_sys::JSON;
use wasm_bindgen::prelude::*;

/// Stores items into localstorage
pub struct Store {
    local_storage: web_sys::Storage,
    data: ItemList,
    name: String,
}

impl Store {
    /// Creates a new store with `name` as the localstorage value name
    pub fn new(name: &str) -> Option<Store> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut store = Store {
                local_storage,
                data: ItemList::new(),
                name: String::from(name),
            };
            store.fetch_local_storage();
            Some(store)
        } else {
            None
        }
    }

    /// Read the local ItemList from localStorage.
    /// Returns an &Option<ItemList> of the stored database
    /// Caches the store into `self.data` to reduce calls to JS
    ///
    /// Uses mut here as the return is something we might want to manipulate
    ///
    fn fetch_local_storage(&mut self) -> Option<()> {
        let mut item_list = ItemList::new();
        // If we have an existing cached value, return early.
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            let data = JSON::parse(&value).ok()?;
            let iter = js_sys::try_iter(&data).ok()??;
            for item in iter {
                let item = item.ok()?;
                let item_array: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&item)?;
                let title = item_array.shift().as_string()?;
                let completed = item_array.shift().as_bool()?;
                let id = item_array.shift().as_string()?;
                let temp_item = Item {
                    title,
                    completed,
                    id,
                };
                item_list.push(temp_item);
            }
        }
        self.data = item_list;
        Some(())
    }

    /// Write the local ItemList to localStorage.
    fn sync_local_storage(&mut self) {
        let array = js_sys::Array::new();
        for item in self.data.iter() {
            let child = js_sys::Array::new();
            child.push(&JsValue::from(&item.title));
            child.push(&JsValue::from(item.completed));
            child.push(&JsValue::from(&item.id));

            array.push(&JsValue::from(child));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.into();
            self.local_storage
                .set_item(&self.name, &storage_string)
                .unwrap();
        }
    }

    /// Find items with properties matching those on query.
    /// `ItemQuery` query Query to match
    ///
    /// ```
    ///  let data = db.find(ItemQuery::Completed {completed: true});
    ///  // data will contain items whose completed properties are true
    /// ```
    pub fn find(&mut self, query: ItemQuery) -> Option<ItemListSlice<'_>> {
        Some(
            self.data
                .iter()
                .filter(|todo| query.matches(todo))
                .collect(),
        )
    }

    /// Update an item in the Store.
    ///
    /// `ItemUpdate` update Record with an id and a property to update
    pub fn update(&mut self, update: ItemUpdate) {
        let id = update.id();
        self.data.iter_mut().for_each(|todo| {
            if id == todo.id {
                todo.update(&update);
            }
        });
        self.sync_local_storage();
    }

    /// Insert an item into the Store.
    ///
    /// `Item` item Item to insert
    pub fn insert(&mut self, item: Item) {
        self.data.push(item);
        self.sync_local_storage();
    }

    /// Remove items from the Store based on a query.
    /// query is an `ItemQuery` query Query matching the items to remove
    pub fn remove(&mut self, query: ItemQuery) {
        self.data.retain(|todo| !query.matches(todo));
        self.sync_local_storage();
    }

    /// Count total, active, and completed todos.
    pub fn count(&mut self) -> Option<(usize, usize, usize)> {
        self.find(ItemQuery::EmptyItemQuery).map(|data| {
            let total = data.length();

            let mut completed = 0;
            for item in data.iter() {
                if item.completed {
                    completed += 1;
                }
            }
            (total, total - completed, completed)
        })
    }
}

/// Represents a todo item
pub struct Item {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Item {
    pub fn update(&mut self, update: &ItemUpdate) {
        match update {
            ItemUpdate::Title { title, .. } => {
                self.title = title.to_string();
            }
            ItemUpdate::Completed { completed, .. } => {
                self.completed = *completed;
            }
        }
    }
}

pub trait ItemListTrait<T> {
    fn new() -> Self;
    fn get(&self, i: usize) -> Option<&T>;
    fn length(&self) -> usize;
    fn push(&mut self, item: T);
    fn iter(&self) -> std::slice::Iter<'_, T>;
}

pub struct ItemList {
    list: Vec<Item>,
}
impl ItemList {
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Item) -> bool,
    {
        self.list.retain(f);
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Item> {
        self.list.iter_mut()
    }
}
impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<'_, Item> {
        self.list.iter()
    }
}
use std::iter::FromIterator;
impl FromIterator<Item> for ItemList {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = ItemList::new();
        for i in iter {
            c.push(i);
        }

        c
    }
}

/// A borrowed set of Items filtered from the store
pub struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}

impl<'a> ItemListTrait<&'a Item> for ItemListSlice<'a> {
    fn new() -> ItemListSlice<'a> {
        ItemListSlice { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&&'a Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: &'a Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<'_, &'a Item> {
        self.list.iter()
    }
}

impl<'a> FromIterator<&'a Item> for ItemListSlice<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Item>>(iter: I) -> Self {
        let mut c = ItemListSlice::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl From<ItemListSlice<'_>> for ItemList {
    fn from(s: ItemListSlice<'_>) -> Self {
        let mut i = ItemList::new();
        let items = s.list.into_iter();
        for j in items {
            // TODO neaten this cloning?
            let item = Item {
                id: j.id.clone(),
                completed: j.completed,
                title: j.title.clone(),
            };
            i.push(item);
        }
        i
    }
}

/// Represents a search into the store
pub enum ItemQuery {
    Id { id: String },
    Completed { completed: bool },
    EmptyItemQuery,
}

impl ItemQuery {
    fn matches(&self, item: &Item) -> bool {
        match *self {
            ItemQuery::EmptyItemQuery => true,
            ItemQuery::Id { ref id } => &item.id == id,
            ItemQuery::Completed { completed } => item.completed == completed,
        }
    }
}

pub enum ItemUpdate {
    Title { id: String, title: String },
    Completed { id: String, completed: bool },
}

impl ItemUpdate {
    fn id(&self) -> String {
        match self {
            ItemUpdate::Title { id, .. } => id.clone(),
            ItemUpdate::Completed { id, .. } => id.clone(),
        }
    }
}

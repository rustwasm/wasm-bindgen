use std::thread_local;
use std::string::String;
use std::borrow::ToOwned;
use std::cell::{Cell, RefCell};
use crate::JsValue;
use uluru::{LRUCache, Entry};


struct Pair {
    key: String,
    value: JsValue,
}

// TODO figure out a good default capacity
type Entries = LRUCache::<[Entry<Pair>; 1_024]>;

struct Cache {
    enabled: Cell<bool>,
    max_str_len: Cell<usize>,
    entries: RefCell<Entries>,
}

// TODO figure out a good max_str_len
thread_local! {
    static CACHE: Cache = Cache {
        enabled: Cell::new(true),
        max_str_len: Cell::new(128),
        entries: RefCell::new(LRUCache::default()),
    };
}

fn get_js_string(cache: &mut Entries, key: &str) -> JsValue {
    if let Some(p) = cache.find(|p| p.key == key) {
        p.value.clone()

    } else {
        let value = JsValue::from(key);

        cache.insert(Pair {
            key: key.to_owned(),
            value: value.clone(),
        });

        value
    }
}

fn cache_str(s: &str) -> JsValue {
    CACHE.with(|cache| {
        let should_cache =
            cache.enabled.get() &&
            s.len() <= cache.max_str_len.get();

        if should_cache {
            get_js_string(&mut cache.entries.borrow_mut(), s)

        } else {
            JsValue::from(s)
        }
    })
}

#[inline]
pub fn str(s: &str) -> JsValue {
    if cfg!(feature = "disable-interning") {
        JsValue::from(s)

    } else {
        cache_str(s)
    }
}

#[inline]
pub fn set_max_str_len(len: usize) {
    if !cfg!(feature = "disable-interning") {
        CACHE.with(|cache| cache.max_str_len.set(len));
    }
}

#[inline]
pub fn enable() {
    if !cfg!(feature = "disable-interning") {
        CACHE.with(|cache| cache.enabled.set(true));
    }
}

#[inline]
pub fn disable() {
    if !cfg!(feature = "disable-interning") {
        CACHE.with(|cache| cache.enabled.set(false));
    }
}

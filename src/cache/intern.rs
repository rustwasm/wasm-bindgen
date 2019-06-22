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
    entries: RefCell<Entries>,
}

// TODO figure out a good max_str_len
thread_local! {
    static CACHE: Cache = Cache {
        entries: RefCell::new(LRUCache::default()),
    };
}

fn get_js_string<'a>(cache: &'a mut Entries, key: &str) -> Option<&'a JsValue> {
    cache.find(|p| p.key == key).map(|x| &x.value)
}

fn insert_js_string(cache: &mut Entries, key: &str, value: JsValue) {
    cache.insert(Pair {
        key: key.to_owned(),
        value,
    });
}

fn get_str(s: &str) -> JsValue {
    CACHE.with(|cache| {
        let mut cache = cache.entries.borrow_mut();

        if let Some(value) = get_js_string(&mut cache, s) {
            value.clone()

        } else {
            JsValue::from(s)
        }
    })
}

fn intern_str(s: &str) {
    CACHE.with(|cache| {
        let mut cache = cache.entries.borrow_mut();

        if get_js_string(&mut cache, s).is_none() {
            insert_js_string(&mut cache, s, JsValue::from(s));
        }
    })
}

#[inline]
pub(crate) fn str(s: &str) -> JsValue {
    if cfg!(feature = "disable-interning") {
        JsValue::from(s)

    } else {
        get_str(s)
    }
}

#[inline]
pub fn intern(s: &str) -> &str {
    if !cfg!(feature = "disable-interning") {
        intern_str(s);
    }

    s
}

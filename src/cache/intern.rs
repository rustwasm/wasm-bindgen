use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        use std::thread_local;
        use std::string::String;
        use std::borrow::ToOwned;
        use std::cell::RefCell;
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

        thread_local! {
            static CACHE: Cache = Cache {
                entries: RefCell::new(LRUCache::default()),
            };
        }

        fn get_js_string<'a>(cache: &'a mut Entries, key: &str) -> Option<&'a JsValue> {
            cache.find(|p| p.key == key).map(|x| &x.value)
        }

        pub(crate) fn get_str(s: &str) -> Option<JsValue> {
            CACHE.with(|cache| {
                let mut cache = cache.entries.borrow_mut();

                if let Some(value) = get_js_string(&mut cache, s) {
                    // This is safe because the cache values are never removed
                    Some(value._unsafe_clone())

                } else {
                    None
                }
            })
        }

        fn intern_str(key: &str) {
            CACHE.with(|cache| {
                let mut cache = cache.entries.borrow_mut();

                if get_js_string(&mut cache, key).is_none() {
                    cache.insert(Pair {
                        key: key.to_owned(),
                        value: JsValue::from(key),
                    });
                }
            })
        }
    }
}


#[inline]
pub fn intern(s: &str) -> &str {
    #[cfg(feature = "enable-interning")]
    intern_str(s);

    s
}

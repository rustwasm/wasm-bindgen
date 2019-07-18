use cfg_if::cfg_if;


cfg_if! {
    if #[cfg(feature = "enable-interning")] {
        use std::thread_local;
        use std::string::String;
        use std::borrow::ToOwned;
        use std::cell::RefCell;
        use std::collections::HashMap;
        use crate::JsValue;
        use crate::convert::IntoWasmAbi;

        struct Cache {
            entries: RefCell<HashMap<String, JsValue>>,
        }

        thread_local! {
            static CACHE: Cache = Cache {
                entries: RefCell::new(HashMap::new()),
            };
        }

        /// This returns the raw index of the cached JsValue, so you must take care
        /// so that you don't use it after it is freed.
        pub(crate) fn unsafe_get_str(s: &str) -> Option<<JsValue as IntoWasmAbi>::Abi> {
            CACHE.with(|cache| {
                let cache = cache.entries.borrow();

                cache.get(s).map(|x| x.into_abi())
            })
        }

        fn intern_str(key: &str) {
            CACHE.with(|cache| {
                let mut cache = cache.entries.borrow_mut();

                // Can't use `entry` because `entry` requires a `String`
                if !cache.contains_key(key) {
                    cache.insert(key.to_owned(), JsValue::from(key));
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

use crate::generated::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! read_test_suite {
    ($maplike:ty, $name:ident) => {
        #[wasm_bindgen_test]
        fn $name() {
            // { "a": 1, "b": 2, "c": 3 }
            let maplike = <$maplike>::new().unwrap();

            // readonly attribute unsigned long size;
            assert_eq!(maplike.size(), 3);

            // boolean has(K key);
            assert!(maplike.has("a"));
            assert!(maplike.has("b"));
            assert!(maplike.has("c"));
            assert!(!maplike.has("d"));

            // V? get(K key);
            assert_eq!(maplike.get("a"), Some(1));
            assert_eq!(maplike.get("b"), Some(2));
            assert_eq!(maplike.get("c"), Some(3));
            assert_eq!(maplike.get("d"), None);

            // { "a": 1, "b": 2, "c": 3 }
            let cb = Closure::wrap(Box::new(|value: u32, key: String| match key.as_str() {
                "a" => assert_eq!(value, 1),
                "b" => assert_eq!(value, 2),
                "c" => assert_eq!(value, 3),
                _ => panic!("unexpected key"),
            }) as Box<dyn Fn(u32, String)>);

            maplike.for_each(cb.as_ref().unchecked_ref()).unwrap();

            let mut entries_vec = vec![];

            for entry in maplike.entries().into_iter() {
                let entry = entry.unwrap();
                let pair = entry.dyn_into::<js_sys::Array>().unwrap();
                let key = pair.get(0).as_string().unwrap();
                let value = pair.get(1).as_f64().unwrap() as u32;

                entries_vec.push((key, value));
            }

            assert_eq!(
                &entries_vec,
                &[
                    ("a".to_string(), 1),
                    ("b".to_string(), 2),
                    ("c".to_string(), 3)
                ]
            );

            let mut keys_vec = vec![];

            for key in maplike.keys().into_iter() {
                let key = key.unwrap();
                keys_vec.push(key.as_string().unwrap());
            }

            assert_eq!(
                &keys_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );

            let mut values_vec = vec![];

            for value in maplike.values().into_iter() {
                let value = value.unwrap();
                values_vec.push(value.as_f64().unwrap() as u32);
            }

            assert_eq!(&values_vec, &[1, 2, 3]);
        }
    };
}

read_test_suite!(TestReadOnlyMapLike, read_readonly_maplike);
read_test_suite!(TestReadWriteMapLike, read_maplike);

#[wasm_bindgen_test]
fn write_maplike() {
    // { "a": 1, "b": 2, "c": 3 }
    let maplike = TestReadWriteMapLike::new().unwrap();

    // undefined set(K key, V value);
    let ret1 = maplike.set("a", 4);
    let ret2 = maplike.set("d", 5);
    assert_eq!(maplike.get("a"), Some(4));
    assert_eq!(maplike.get("d"), Some(5));
    assert_eq!(ret1, maplike);
    assert_eq!(ret2, maplike);

    // boolean delete(K key);
    assert!(maplike.delete("a"));
    assert_eq!(maplike.get("a"), None);
    assert!(!maplike.delete("a"));

    // undefined clear();
    maplike.clear();
    assert_eq!(maplike.size(), 0);
}

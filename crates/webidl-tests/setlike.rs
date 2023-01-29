use crate::generated::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! read_test_suite {
    ($setlike:ty, $name:ident) => {
        #[wasm_bindgen_test]
        fn $name() {
            // { "a", "b", "c" }
            let setlike = <$setlike>::new().unwrap();

            // readonly attribute unsigned long size;
            assert_eq!(setlike.size(), 3);

            // boolean has(K key);
            assert!(setlike.has("a"));
            assert!(setlike.has("b"));
            assert!(setlike.has("c"));
            assert!(!setlike.has("d"));

            // { "a", "b", "c" }
            let cb = Closure::wrap(Box::new(|value: String| match value.as_str() {
                "a" => assert_eq!(value, "a"),
                "b" => assert_eq!(value, "b"),
                "c" => assert_eq!(value, "c"),
                _ => panic!("unexpected key"),
            }) as Box<dyn Fn(String)>);

            setlike.for_each(cb.as_ref().unchecked_ref()).unwrap();

            let mut entries_vec = vec![];

            for entry in setlike.entries().into_iter() {
                let entry = entry.unwrap();
                let value = entry
                    .dyn_into::<js_sys::JsString>()
                    .unwrap()
                    .as_string()
                    .unwrap();

                entries_vec.push(value);
            }

            assert_eq!(
                &entries_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );

            let mut keys_vec = vec![];

            for (i, _) in setlike.keys().into_iter().enumerate() {
                keys_vec.push(i);
            }

            assert_eq!(&keys_vec, &[0, 1, 2]);

            let mut values_vec = vec![];

            for entry in setlike.values().into_iter() {
                let entry = entry.unwrap();
                let value = entry
                    .dyn_into::<js_sys::JsString>()
                    .unwrap()
                    .as_string()
                    .unwrap();

                values_vec.push(value);
            }

            assert_eq!(
                &values_vec,
                &["a".to_string(), "b".to_string(), "c".to_string()]
            );
        }
    };
}

read_test_suite!(TestReadOnlySetLike, read_readonly_setlike);
read_test_suite!(TestReadWriteSetLike, read_setlike);

#[wasm_bindgen_test]
fn write_setlike() {
    // { "a", "b", "c" }
    let setlike = TestReadWriteSetLike::new().unwrap();

    setlike.add("d");
    assert_eq!(setlike.size(), 4);

    assert!(setlike.delete("d"));
    assert_eq!(setlike.size(), 3);

    // undefined clear();
    setlike.clear();
    assert_eq!(setlike.size(), 0);
}

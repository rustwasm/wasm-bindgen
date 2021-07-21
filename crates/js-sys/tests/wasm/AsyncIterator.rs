use js_sys::*;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/AsyncIterator.js")]
extern "C" {
    fn get_async_iterable() -> AsyncIterator;
}

#[wasm_bindgen_test]
async fn next() {
    async fn run() -> Result<(), Error> {
        let iter = get_async_iterable();

        let result = JsFuture::from(iter.next()?)
            .await?
            .unchecked_into::<IteratorNext>();
        assert!(!result.done());
        assert_eq!(Into::<JsValue>::into("one"), result.value());

        let result = JsFuture::from(iter.next()?)
            .await?
            .unchecked_into::<IteratorNext>();
        assert!(!result.done());
        assert_eq!(Into::<JsValue>::into("two"), result.value());

        let result = JsFuture::from(iter.next()?)
            .await?
            .unchecked_into::<IteratorNext>();
        assert!(!result.done());
        assert_eq!(Into::<JsValue>::into("three"), result.value());

        let result = JsFuture::from(iter.next()?)
            .await?
            .unchecked_into::<IteratorNext>();
        assert!(result.done());

        Ok(())
    }

    run().await.unwrap();
}

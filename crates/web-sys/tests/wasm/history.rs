use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::{History, ScrollRestoration};
use js_sys::Object;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = history, js_namespace = window)]
    static HISTORY: History;
}

#[wasm_bindgen_test]
fn history() {
    HISTORY.push_state(
        Object::new().into(),
        "I am a title",
        Some("part/of/some/url"),
    ).unwrap();
    assert_eq!(HISTORY.length().unwrap(), 2);

    assert!(HISTORY.go(1).is_ok());
    assert!(HISTORY.back().is_ok());
    assert!(HISTORY.forward().is_ok());
    assert!(HISTORY.go(-1).is_ok());

    HISTORY.set_scroll_restoration(ScrollRestoration::Manual).expect("failure to set scroll restoration");
    assert_eq!(HISTORY.scroll_restoration().unwrap(), ScrollRestoration::Manual);

    HISTORY.set_scroll_restoration(ScrollRestoration::Auto).expect("failure to set scroll restoration");
    assert_eq!(HISTORY.scroll_restoration().unwrap(), ScrollRestoration::Auto);
}

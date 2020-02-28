use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = History , typescript_name = History ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `History` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub type History;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = length ) ]
    #[doc = "Getter for the `length` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/length)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn length(this: &History) -> Result<u32, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrollRestoration ) ]
    #[cfg(feature = "ScrollRestoration")]
    #[doc = "Getter for the `scrollRestoration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)\n\n*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    pub fn scroll_restoration(this: &History) -> Result<ScrollRestoration, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = scrollRestoration ) ]
    #[cfg(feature = "ScrollRestoration")]
    #[doc = "Setter for the `scrollRestoration` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)\n\n*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*"]
    pub fn set_scroll_restoration(this: &History, value: ScrollRestoration) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = state ) ]
    #[doc = "Getter for the `state` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/state)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn state(this: &History) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = back ) ]
    #[doc = "The `back()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/back)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn back(this: &History) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = forward ) ]
    #[doc = "The `forward()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn forward(this: &History) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = go ) ]
    #[doc = "The `go()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn go(this: &History) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = go ) ]
    #[doc = "The `go()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn go_with_delta(this: &History, delta: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = pushState ) ]
    #[doc = "The `pushState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn push_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = pushState ) ]
    #[doc = "The `pushState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn push_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceState ) ]
    #[doc = "The `replaceState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn replace_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = replaceState ) ]
    #[doc = "The `replaceState()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)\n\n*This API requires the following crate features to be activated: `History`*"]
    pub fn replace_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;
}

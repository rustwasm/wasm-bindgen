use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = History , typescript_name = History ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `History` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub type History;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "History" , js_name = length ) ]
    ///Getter for the `length` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/length)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn length(this: &History) -> Result<u32, JsValue>;

    #[cfg(feature = "ScrollRestoration")]
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "History" , js_name = scrollRestoration ) ]
    ///Getter for the `scrollRestoration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)
    ///
    ///*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*
    pub fn scroll_restoration(this: &History) -> Result<ScrollRestoration, JsValue>;

    #[cfg(feature = "ScrollRestoration")]
    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "History" , js_name = scrollRestoration ) ]
    ///Setter for the `scrollRestoration` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)
    ///
    ///*This API requires the following crate features to be activated: `History`, `ScrollRestoration`*
    pub fn set_scroll_restoration(this: &History, value: ScrollRestoration) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "History" , js_name = state ) ]
    ///Getter for the `state` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/state)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn state(this: &History) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = back ) ]
    ///The `back()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/back)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn back(this: &History) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = forward ) ]
    ///The `forward()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn forward(this: &History) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = go ) ]
    ///The `go()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn go(this: &History) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = go ) ]
    ///The `go()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/go)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn go_with_delta(this: &History, delta: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = pushState ) ]
    ///The `pushState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn push_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = pushState ) ]
    ///The `pushState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn push_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = replaceState ) ]
    ///The `replaceState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn replace_state(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "History" , js_name = replaceState ) ]
    ///The `replaceState()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)
    ///
    ///*This API requires the following crate features to be activated: `History`*
    pub fn replace_state_with_url(
        this: &History,
        data: &::wasm_bindgen::JsValue,
        title: &str,
        url: Option<&str>,
    ) -> Result<(), JsValue>;

}

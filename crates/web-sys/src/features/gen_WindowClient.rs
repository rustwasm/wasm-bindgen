use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Client , extends = :: js_sys :: Object , js_name = WindowClient , typescript_name = WindowClient ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WindowClient` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)
    ///
    ///*This API requires the following crate features to be activated: `WindowClient`*
    pub type WindowClient;

    #[cfg(feature = "VisibilityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "WindowClient" , js_name = visibilityState ) ]
    ///Getter for the `visibilityState` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)
    ///
    ///*This API requires the following crate features to be activated: `VisibilityState`, `WindowClient`*
    pub fn visibility_state(this: &WindowClient) -> VisibilityState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WindowClient" , js_name = focused ) ]
    ///Getter for the `focused` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)
    ///
    ///*This API requires the following crate features to be activated: `WindowClient`*
    pub fn focused(this: &WindowClient) -> bool;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WindowClient" , js_name = focus ) ]
    ///The `focus()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)
    ///
    ///*This API requires the following crate features to be activated: `WindowClient`*
    pub fn focus(this: &WindowClient) -> Result<::js_sys::Promise, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "WindowClient" , js_name = navigate ) ]
    ///The `navigate()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)
    ///
    ///*This API requires the following crate features to be activated: `WindowClient`*
    pub fn navigate(this: &WindowClient, url: &str) -> Result<::js_sys::Promise, JsValue>;

}

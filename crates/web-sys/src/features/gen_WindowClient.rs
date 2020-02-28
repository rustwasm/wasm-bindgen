use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Client , extends = :: js_sys :: Object , js_name = WindowClient , typescript_name = WindowClient ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WindowClient` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    pub type WindowClient;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WindowClient" , js_name = visibilityState ) ]
    #[cfg(feature = "VisibilityState")]
    #[doc = "Getter for the `visibilityState` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)\n\n*This API requires the following crate features to be activated: `VisibilityState`, `WindowClient`*"]
    pub fn visibility_state(this: &WindowClient) -> VisibilityState;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WindowClient" , js_name = focused ) ]
    #[doc = "Getter for the `focused` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn focused(this: &WindowClient) -> bool;
    # [ wasm_bindgen ( catch , method , structural , js_class = "WindowClient" , js_name = focus ) ]
    #[doc = "The `focus()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn focus(this: &WindowClient) -> Result<::js_sys::Promise, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "WindowClient" , js_name = navigate ) ]
    #[doc = "The `navigate()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    pub fn navigate(this: &WindowClient, url: &str) -> Result<::js_sys::Promise, JsValue>;
}

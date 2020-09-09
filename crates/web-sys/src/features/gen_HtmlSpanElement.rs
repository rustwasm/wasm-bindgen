#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSpanElement , typescript_type = "HTMLSpanElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSpanElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSpanElement`*"]
    pub type HtmlSpanElement;
}

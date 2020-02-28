use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlMediaElement , extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLAudioElement , typescript_name = HTMLAudioElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAudioElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub type HtmlAudioElement;
    #[wasm_bindgen(catch, js_class = "HTMLAudioElement", constructor)]
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub fn new(this: &HtmlAudioElement) -> Result<HtmlAudioElement, JsValue>;
    #[wasm_bindgen(catch, js_class = "HTMLAudioElement", constructor)]
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    pub fn new_with_src(this: &HtmlAudioElement, src: &str) -> Result<HtmlAudioElement, JsValue>;
}

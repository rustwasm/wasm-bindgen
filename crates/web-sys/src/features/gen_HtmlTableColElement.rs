use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableColElement , typescript_name = HTMLTableColElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableColElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub type HtmlTableColElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = span ) ]
    #[doc = "Getter for the `span` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn span(this: &HtmlTableColElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = span ) ]
    #[doc = "Setter for the `span` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_span(this: &HtmlTableColElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn align(this: &HtmlTableColElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_align(this: &HtmlTableColElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = ch ) ]
    #[doc = "Getter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn ch(this: &HtmlTableColElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = ch ) ]
    #[doc = "Setter for the `ch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_ch(this: &HtmlTableColElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = chOff ) ]
    #[doc = "Getter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn ch_off(this: &HtmlTableColElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = chOff ) ]
    #[doc = "Setter for the `chOff` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_ch_off(this: &HtmlTableColElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = vAlign ) ]
    #[doc = "Getter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn v_align(this: &HtmlTableColElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = vAlign ) ]
    #[doc = "Setter for the `vAlign` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_v_align(this: &HtmlTableColElement, value: String);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn width(this: &HtmlTableColElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)\n\n*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_width(this: &HtmlTableColElement, value: String);
}

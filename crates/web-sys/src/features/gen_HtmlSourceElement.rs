use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSourceElement , typescript_name = HTMLSourceElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSourceElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub type HtmlSourceElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn src(this: &HtmlSourceElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_src(this: &HtmlSourceElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn type_(this: &HtmlSourceElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_type(this: &HtmlSourceElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = srcset ) ]
    #[doc = "Getter for the `srcset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn srcset(this: &HtmlSourceElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = srcset ) ]
    #[doc = "Setter for the `srcset` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_srcset(this: &HtmlSourceElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = sizes ) ]
    #[doc = "Getter for the `sizes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn sizes(this: &HtmlSourceElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = sizes ) ]
    #[doc = "Setter for the `sizes` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_sizes(this: &HtmlSourceElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = media ) ]
    #[doc = "Getter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn media(this: &HtmlSourceElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = media ) ]
    #[doc = "Setter for the `media` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)\n\n*This API requires the following crate features to be activated: `HtmlSourceElement`*"]
    pub fn set_media(this: &HtmlSourceElement, value: &str);
}

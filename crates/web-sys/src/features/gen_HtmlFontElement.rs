use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFontElement , typescript_name = HTMLFontElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFontElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub type HtmlFontElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = color ) ]
    #[doc = "Getter for the `color` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn color(this: &HtmlFontElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = color ) ]
    #[doc = "Setter for the `color` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_color(this: &HtmlFontElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = face ) ]
    #[doc = "Getter for the `face` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn face(this: &HtmlFontElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = face ) ]
    #[doc = "Setter for the `face` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_face(this: &HtmlFontElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = size ) ]
    #[doc = "Getter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn size(this: &HtmlFontElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = size ) ]
    #[doc = "Setter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)\n\n*This API requires the following crate features to be activated: `HtmlFontElement`*"]
    pub fn set_size(this: &HtmlFontElement, value: &str);
}

use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLModElement , typescript_name = HTMLModElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlModElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)\n\n*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub type HtmlModElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = cite ) ]
    #[doc = "Getter for the `cite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)\n\n*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn cite(this: &HtmlModElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = cite ) ]
    #[doc = "Setter for the `cite` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)\n\n*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn set_cite(this: &HtmlModElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = dateTime ) ]
    #[doc = "Getter for the `dateTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)\n\n*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn date_time(this: &HtmlModElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = dateTime ) ]
    #[doc = "Setter for the `dateTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)\n\n*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn set_date_time(this: &HtmlModElement, value: &str);
}

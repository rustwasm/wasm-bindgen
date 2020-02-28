use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ScrollBoxObject , typescript_name = ScrollBoxObject ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollBoxObject` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub type ScrollBoxObject;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = positionX ) ]
    #[doc = "Getter for the `positionX` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionX)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn position_x(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = positionY ) ]
    #[doc = "Getter for the `positionY` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionY)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn position_y(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrolledWidth ) ]
    #[doc = "Getter for the `scrolledWidth` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledWidth)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scrolled_width(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = scrolledHeight ) ]
    #[doc = "Getter for the `scrolledHeight` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledHeight)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scrolled_height(this: &ScrollBoxObject) -> Result<i32, JsValue>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_name = ensureElementIsVisible ) ]
    #[doc = "The `ensureElementIsVisible()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/ensureElementIsVisible)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    pub fn ensure_element_is_visible(
        this: &ScrollBoxObject,
        child: &Element,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = scrollBy ) ]
    #[doc = "The `scrollBy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollBy)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_by(this: &ScrollBoxObject, dx: i32, dy: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = scrollByIndex ) ]
    #[doc = "The `scrollByIndex()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollByIndex)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_by_index(this: &ScrollBoxObject, dindexes: i32) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = scrollTo ) ]
    #[doc = "The `scrollTo()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollTo)\n\n*This API requires the following crate features to be activated: `ScrollBoxObject`*"]
    pub fn scroll_to(this: &ScrollBoxObject, x: i32, y: i32) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_name = scrollToElement ) ]
    #[doc = "The `scrollToElement()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollToElement)\n\n*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*"]
    pub fn scroll_to_element(this: &ScrollBoxObject, child: &Element) -> Result<(), JsValue>;
}

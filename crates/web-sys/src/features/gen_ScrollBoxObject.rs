use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ScrollBoxObject , typescript_name = ScrollBoxObject ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ScrollBoxObject` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub type ScrollBoxObject;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScrollBoxObject" , js_name = positionX ) ]
    ///Getter for the `positionX` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionX)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn position_x(this: &ScrollBoxObject) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScrollBoxObject" , js_name = positionY ) ]
    ///Getter for the `positionY` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/positionY)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn position_y(this: &ScrollBoxObject) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScrollBoxObject" , js_name = scrolledWidth ) ]
    ///Getter for the `scrolledWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledWidth)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn scrolled_width(this: &ScrollBoxObject) -> Result<i32, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "ScrollBoxObject" , js_name = scrolledHeight ) ]
    ///Getter for the `scrolledHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrolledHeight)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn scrolled_height(this: &ScrollBoxObject) -> Result<i32, JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ScrollBoxObject" , js_name = ensureElementIsVisible ) ]
    ///The `ensureElementIsVisible()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/ensureElementIsVisible)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*
    pub fn ensure_element_is_visible(
        this: &ScrollBoxObject,
        child: &Element,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ScrollBoxObject" , js_name = scrollBy ) ]
    ///The `scrollBy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollBy)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn scroll_by(this: &ScrollBoxObject, dx: i32, dy: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ScrollBoxObject" , js_name = scrollByIndex ) ]
    ///The `scrollByIndex()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollByIndex)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn scroll_by_index(this: &ScrollBoxObject, dindexes: i32) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ScrollBoxObject" , js_name = scrollTo ) ]
    ///The `scrollTo()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollTo)
    ///
    ///*This API requires the following crate features to be activated: `ScrollBoxObject`*
    pub fn scroll_to(this: &ScrollBoxObject, x: i32, y: i32) -> Result<(), JsValue>;

    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( catch , method , structural , js_class = "ScrollBoxObject" , js_name = scrollToElement ) ]
    ///The `scrollToElement()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollBoxObject/scrollToElement)
    ///
    ///*This API requires the following crate features to be activated: `Element`, `ScrollBoxObject`*
    pub fn scroll_to_element(this: &ScrollBoxObject, child: &Element) -> Result<(), JsValue>;

}

use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLProgressElement , typescript_name = HTMLProgressElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlProgressElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub type HtmlProgressElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLProgressElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub fn value(this: &HtmlProgressElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLProgressElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub fn set_value(this: &HtmlProgressElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLProgressElement" , js_name = max ) ]
    ///Getter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub fn max(this: &HtmlProgressElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLProgressElement" , js_name = max ) ]
    ///Setter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub fn set_max(this: &HtmlProgressElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLProgressElement" , js_name = position ) ]
    ///Getter for the `position` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`*
    pub fn position(this: &HtmlProgressElement) -> f64;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLProgressElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlProgressElement`, `NodeList`*
    pub fn labels(this: &HtmlProgressElement) -> NodeList;

}

use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLMeterElement , typescript_type = "HTMLMeterElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlMeterElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub type HtmlMeterElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn value(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_value(this: &HtmlMeterElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = min ) ]
    ///Getter for the `min` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn min(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = min ) ]
    ///Setter for the `min` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_min(this: &HtmlMeterElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = max ) ]
    ///Getter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn max(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = max ) ]
    ///Setter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_max(this: &HtmlMeterElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = low ) ]
    ///Getter for the `low` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn low(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = low ) ]
    ///Setter for the `low` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_low(this: &HtmlMeterElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = high ) ]
    ///Getter for the `high` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn high(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = high ) ]
    ///Setter for the `high` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_high(this: &HtmlMeterElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = optimum ) ]
    ///Getter for the `optimum` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn optimum(this: &HtmlMeterElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLMeterElement" , js_name = optimum ) ]
    ///Setter for the `optimum` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`*
    pub fn set_optimum(this: &HtmlMeterElement, value: f64);

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLMeterElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlMeterElement`, `NodeList`*
    pub fn labels(this: &HtmlMeterElement) -> NodeList;

}

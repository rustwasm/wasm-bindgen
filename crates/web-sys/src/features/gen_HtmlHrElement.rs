use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLHRElement , typescript_name = HTMLHRElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlHrElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub type HtmlHrElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHRElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn align(this: &HtmlHrElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHRElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn set_align(this: &HtmlHrElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHRElement" , js_name = color ) ]
    ///Getter for the `color` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn color(this: &HtmlHrElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHRElement" , js_name = color ) ]
    ///Setter for the `color` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn set_color(this: &HtmlHrElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHRElement" , js_name = noShade ) ]
    ///Getter for the `noShade` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn no_shade(this: &HtmlHrElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHRElement" , js_name = noShade ) ]
    ///Setter for the `noShade` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn set_no_shade(this: &HtmlHrElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHRElement" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn size(this: &HtmlHrElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHRElement" , js_name = size ) ]
    ///Setter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn set_size(this: &HtmlHrElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLHRElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn width(this: &HtmlHrElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLHRElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlHrElement`*
    pub fn set_width(this: &HtmlHrElement, value: &str);

}

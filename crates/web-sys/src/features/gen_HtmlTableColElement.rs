use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLTableColElement , typescript_name = HTMLTableColElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlTableColElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub type HtmlTableColElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = span ) ]
    ///Getter for the `span` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn span(this: &HtmlTableColElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = span ) ]
    ///Setter for the `span` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_span(this: &HtmlTableColElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn align(this: &HtmlTableColElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_align(this: &HtmlTableColElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = ch ) ]
    ///Getter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn ch(this: &HtmlTableColElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = ch ) ]
    ///Setter for the `ch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_ch(this: &HtmlTableColElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = chOff ) ]
    ///Getter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn ch_off(this: &HtmlTableColElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = chOff ) ]
    ///Setter for the `chOff` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_ch_off(this: &HtmlTableColElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = vAlign ) ]
    ///Getter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn v_align(this: &HtmlTableColElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = vAlign ) ]
    ///Setter for the `vAlign` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_v_align(this: &HtmlTableColElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLTableColElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn width(this: &HtmlTableColElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLTableColElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlTableColElement`*
    pub fn set_width(this: &HtmlTableColElement, value: &str);

}

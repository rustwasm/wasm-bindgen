use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFontElement , typescript_name = HTMLFontElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlFontElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub type HtmlFontElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFontElement" , js_name = color ) ]
    ///Getter for the `color` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn color(this: &HtmlFontElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFontElement" , js_name = color ) ]
    ///Setter for the `color` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/color)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn set_color(this: &HtmlFontElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFontElement" , js_name = face ) ]
    ///Getter for the `face` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn face(this: &HtmlFontElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFontElement" , js_name = face ) ]
    ///Setter for the `face` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/face)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn set_face(this: &HtmlFontElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFontElement" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn size(this: &HtmlFontElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFontElement" , js_name = size ) ]
    ///Setter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFontElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFontElement`*
    pub fn set_size(this: &HtmlFontElement, value: &str);

}

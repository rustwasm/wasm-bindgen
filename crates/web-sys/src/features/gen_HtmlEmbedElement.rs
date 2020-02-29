use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLEmbedElement , typescript_name = HTMLEmbedElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlEmbedElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub type HtmlEmbedElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn src(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_src(this: &HtmlEmbedElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn type_(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_type(this: &HtmlEmbedElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn width(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_width(this: &HtmlEmbedElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn height(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_height(this: &HtmlEmbedElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn align(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_align(this: &HtmlEmbedElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLEmbedElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn name(this: &HtmlEmbedElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLEmbedElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlEmbedElement`*
    pub fn set_name(this: &HtmlEmbedElement, value: &str);

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLEmbedElement" , js_name = getSVGDocument ) ]
    ///The `getSVGDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLEmbedElement/getSVGDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlEmbedElement`*
    pub fn get_svg_document(this: &HtmlEmbedElement) -> Option<Document>;

}

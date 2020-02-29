use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLSourceElement , typescript_name = HTMLSourceElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlSourceElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub type HtmlSourceElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn src(this: &HtmlSourceElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn set_src(this: &HtmlSourceElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn type_(this: &HtmlSourceElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn set_type(this: &HtmlSourceElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = srcset ) ]
    ///Getter for the `srcset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn srcset(this: &HtmlSourceElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = srcset ) ]
    ///Setter for the `srcset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/srcset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn set_srcset(this: &HtmlSourceElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = sizes ) ]
    ///Getter for the `sizes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn sizes(this: &HtmlSourceElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = sizes ) ]
    ///Setter for the `sizes` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/sizes)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn set_sizes(this: &HtmlSourceElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLSourceElement" , js_name = media ) ]
    ///Getter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn media(this: &HtmlSourceElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLSourceElement" , js_name = media ) ]
    ///Setter for the `media` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSourceElement/media)
    ///
    ///*This API requires the following crate features to be activated: `HtmlSourceElement`*
    pub fn set_media(this: &HtmlSourceElement, value: &str);

}

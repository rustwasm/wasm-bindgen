use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLFrameElement , typescript_type = "HTMLFrameElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlFrameElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub type HtmlFrameElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn name(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_name(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = scrolling ) ]
    ///Getter for the `scrolling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn scrolling(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = scrolling ) ]
    ///Setter for the `scrolling` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/scrolling)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_scrolling(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn src(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_src(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = frameBorder ) ]
    ///Getter for the `frameBorder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn frame_border(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = frameBorder ) ]
    ///Setter for the `frameBorder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/frameBorder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_frame_border(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = longDesc ) ]
    ///Getter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn long_desc(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = longDesc ) ]
    ///Setter for the `longDesc` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/longDesc)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_long_desc(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = noResize ) ]
    ///Getter for the `noResize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn no_resize(this: &HtmlFrameElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = noResize ) ]
    ///Setter for the `noResize` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/noResize)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_no_resize(this: &HtmlFrameElement, value: bool);

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = contentDocument ) ]
    ///Getter for the `contentDocument` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlFrameElement`*
    pub fn content_document(this: &HtmlFrameElement) -> Option<Document>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = contentWindow ) ]
    ///Getter for the `contentWindow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/contentWindow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`, `Window`*
    pub fn content_window(this: &HtmlFrameElement) -> Option<Window>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = marginHeight ) ]
    ///Getter for the `marginHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn margin_height(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = marginHeight ) ]
    ///Setter for the `marginHeight` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginHeight)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_margin_height(this: &HtmlFrameElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLFrameElement" , js_name = marginWidth ) ]
    ///Getter for the `marginWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn margin_width(this: &HtmlFrameElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLFrameElement" , js_name = marginWidth ) ]
    ///Setter for the `marginWidth` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameElement/marginWidth)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFrameElement`*
    pub fn set_margin_width(this: &HtmlFrameElement, value: &str);

}

use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLScriptElement , typescript_type = "HTMLScriptElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlScriptElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub type HtmlScriptElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn src(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_src(this: &HtmlScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn type_(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_type(this: &HtmlScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = noModule ) ]
    ///Getter for the `noModule` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn no_module(this: &HtmlScriptElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = noModule ) ]
    ///Setter for the `noModule` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_no_module(this: &HtmlScriptElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = charset ) ]
    ///Getter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn charset(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = charset ) ]
    ///Setter for the `charset` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_charset(this: &HtmlScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = async ) ]
    ///Getter for the `async` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn r#async(this: &HtmlScriptElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = async ) ]
    ///Setter for the `async` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_async(this: &HtmlScriptElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = defer ) ]
    ///Getter for the `defer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn defer(this: &HtmlScriptElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = defer ) ]
    ///Setter for the `defer` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_defer(this: &HtmlScriptElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = crossOrigin ) ]
    ///Getter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn cross_origin(this: &HtmlScriptElement) -> Option<String>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = crossOrigin ) ]
    ///Setter for the `crossOrigin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_cross_origin(this: &HtmlScriptElement, value: Option<&str>);

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLScriptElement" , js_name = text ) ]
    ///Getter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn text(this: &HtmlScriptElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLScriptElement" , js_name = text ) ]
    ///Setter for the `text` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_text(this: &HtmlScriptElement, value: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = event ) ]
    ///Getter for the `event` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn event(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = event ) ]
    ///Setter for the `event` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_event(this: &HtmlScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = htmlFor ) ]
    ///Getter for the `htmlFor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn html_for(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = htmlFor ) ]
    ///Setter for the `htmlFor` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_html_for(this: &HtmlScriptElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLScriptElement" , js_name = integrity ) ]
    ///Getter for the `integrity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn integrity(this: &HtmlScriptElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLScriptElement" , js_name = integrity ) ]
    ///Setter for the `integrity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlScriptElement`*
    pub fn set_integrity(this: &HtmlScriptElement, value: &str);

}

use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLHttpRequestEventTarget , typescript_name = XMLHttpRequestEventTarget ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `XmlHttpRequestEventTarget` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub type XmlHttpRequestEventTarget;

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadstart ) ]
    ///Getter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onloadstart(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadstart ) ]
    ///Setter for the `onloadstart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onloadstart(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onprogress ) ]
    ///Getter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onprogress(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onprogress ) ]
    ///Setter for the `onprogress` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onprogress(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onabort ) ]
    ///Getter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onabort(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onabort ) ]
    ///Setter for the `onabort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onabort(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onerror(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onerror(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onload ) ]
    ///Getter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onload(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onload ) ]
    ///Setter for the `onload` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onload(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = ontimeout ) ]
    ///Getter for the `ontimeout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn ontimeout(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = ontimeout ) ]
    ///Setter for the `ontimeout` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_ontimeout(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadend ) ]
    ///Getter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn onloadend(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadend ) ]
    ///Setter for the `onloadend` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)
    ///
    ///*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*
    pub fn set_onloadend(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);

}

use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = XMLHttpRequestEventTarget , typescript_name = XMLHttpRequestEventTarget ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlHttpRequestEventTarget` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub type XmlHttpRequestEventTarget;
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadstart ) ]
    #[doc = "Getter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onloadstart(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadstart ) ]
    #[doc = "Setter for the `onloadstart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onloadstart(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onprogress ) ]
    #[doc = "Getter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onprogress(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onprogress ) ]
    #[doc = "Setter for the `onprogress` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onprogress(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onabort ) ]
    #[doc = "Getter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onabort(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onabort ) ]
    #[doc = "Setter for the `onabort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onabort(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onerror(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onerror(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onload ) ]
    #[doc = "Getter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onload(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onload ) ]
    #[doc = "Setter for the `onload` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onload(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = ontimeout ) ]
    #[doc = "Getter for the `ontimeout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn ontimeout(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = ontimeout ) ]
    #[doc = "Setter for the `ontimeout` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_ontimeout(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadend ) ]
    #[doc = "Getter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn onloadend(this: &XmlHttpRequestEventTarget) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_class = "XMLHttpRequestEventTarget" , js_name = onloadend ) ]
    #[doc = "Setter for the `onloadend` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    pub fn set_onloadend(this: &XmlHttpRequestEventTarget, value: Option<&::js_sys::Function>);
}

use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLObjectElement , typescript_name = HTMLObjectElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlObjectElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub type HtmlObjectElement;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn data(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = data ) ]
    #[doc = "Setter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_data(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn type_(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_type(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = typeMustMatch ) ]
    #[doc = "Getter for the `typeMustMatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn type_must_match(this: &HtmlObjectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = typeMustMatch ) ]
    #[doc = "Setter for the `typeMustMatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_type_must_match(this: &HtmlObjectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn name(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_name(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = useMap ) ]
    #[doc = "Getter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn use_map(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = useMap ) ]
    #[doc = "Setter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_use_map(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlObjectElement`*"]
    pub fn form(this: &HtmlObjectElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn width(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_width(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn height(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_height(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = contentDocument ) ]
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `contentDocument` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    pub fn content_document(this: &HtmlObjectElement) -> Option<Document>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = contentWindow ) ]
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `contentWindow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`, `Window`*"]
    pub fn content_window(this: &HtmlObjectElement) -> Option<Window>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn will_validate(this: &HtmlObjectElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlObjectElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLObjectElement" , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn validation_message(this: &HtmlObjectElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn align(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_align(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = archive ) ]
    #[doc = "Getter for the `archive` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn archive(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = archive ) ]
    #[doc = "Setter for the `archive` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_archive(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = code ) ]
    #[doc = "Getter for the `code` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = code ) ]
    #[doc = "Setter for the `code` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = declare ) ]
    #[doc = "Getter for the `declare` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn declare(this: &HtmlObjectElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = declare ) ]
    #[doc = "Setter for the `declare` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_declare(this: &HtmlObjectElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = hspace ) ]
    #[doc = "Getter for the `hspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn hspace(this: &HtmlObjectElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = hspace ) ]
    #[doc = "Setter for the `hspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_hspace(this: &HtmlObjectElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = standby ) ]
    #[doc = "Getter for the `standby` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn standby(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = standby ) ]
    #[doc = "Setter for the `standby` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_standby(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = vspace ) ]
    #[doc = "Getter for the `vspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn vspace(this: &HtmlObjectElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = vspace ) ]
    #[doc = "Setter for the `vspace` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_vspace(this: &HtmlObjectElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = codeBase ) ]
    #[doc = "Getter for the `codeBase` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code_base(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = codeBase ) ]
    #[doc = "Setter for the `codeBase` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code_base(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = codeType ) ]
    #[doc = "Getter for the `codeType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn code_type(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = codeType ) ]
    #[doc = "Setter for the `codeType` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_code_type(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = border ) ]
    #[doc = "Getter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn border(this: &HtmlObjectElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = border ) ]
    #[doc = "Setter for the `border` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_border(this: &HtmlObjectElement, value: &str);
    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn check_validity(this: &HtmlObjectElement) -> bool;
    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = getSVGDocument ) ]
    #[doc = "The `getSVGDocument()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    pub fn get_svg_document(this: &HtmlObjectElement) -> Option<Document>;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn report_validity(this: &HtmlObjectElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    pub fn set_custom_validity(this: &HtmlObjectElement, error: &str);
}

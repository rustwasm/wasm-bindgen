use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLObjectElement , typescript_type = "HTMLObjectElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlObjectElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub type HtmlObjectElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn data(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = data ) ]
    ///Setter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_data(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn type_(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_type(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = typeMustMatch ) ]
    ///Getter for the `typeMustMatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn type_must_match(this: &HtmlObjectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = typeMustMatch ) ]
    ///Setter for the `typeMustMatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_type_must_match(this: &HtmlObjectElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn name(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_name(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = useMap ) ]
    ///Getter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn use_map(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = useMap ) ]
    ///Setter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_use_map(this: &HtmlObjectElement, value: &str);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlObjectElement`*
    pub fn form(this: &HtmlObjectElement) -> Option<HtmlFormElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn width(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_width(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn height(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_height(this: &HtmlObjectElement, value: &str);

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = contentDocument ) ]
    ///Getter for the `contentDocument` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*
    pub fn content_document(this: &HtmlObjectElement) -> Option<Document>;

    #[cfg(feature = "Window")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = contentWindow ) ]
    ///Getter for the `contentWindow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`, `Window`*
    pub fn content_window(this: &HtmlObjectElement) -> Option<Window>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn will_validate(this: &HtmlObjectElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`, `ValidityState`*
    pub fn validity(this: &HtmlObjectElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLObjectElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn validation_message(this: &HtmlObjectElement) -> Result<String, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn align(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_align(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = archive ) ]
    ///Getter for the `archive` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn archive(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = archive ) ]
    ///Setter for the `archive` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_archive(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn code(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = code ) ]
    ///Setter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_code(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = declare ) ]
    ///Getter for the `declare` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn declare(this: &HtmlObjectElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = declare ) ]
    ///Setter for the `declare` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_declare(this: &HtmlObjectElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = hspace ) ]
    ///Getter for the `hspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn hspace(this: &HtmlObjectElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = hspace ) ]
    ///Setter for the `hspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_hspace(this: &HtmlObjectElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = standby ) ]
    ///Getter for the `standby` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn standby(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = standby ) ]
    ///Setter for the `standby` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_standby(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = vspace ) ]
    ///Getter for the `vspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn vspace(this: &HtmlObjectElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = vspace ) ]
    ///Setter for the `vspace` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_vspace(this: &HtmlObjectElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = codeBase ) ]
    ///Getter for the `codeBase` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn code_base(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = codeBase ) ]
    ///Setter for the `codeBase` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_code_base(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = codeType ) ]
    ///Getter for the `codeType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn code_type(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = codeType ) ]
    ///Setter for the `codeType` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_code_type(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLObjectElement" , js_name = border ) ]
    ///Getter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn border(this: &HtmlObjectElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLObjectElement" , js_name = border ) ]
    ///Setter for the `border` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_border(this: &HtmlObjectElement, value: &str);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn check_validity(this: &HtmlObjectElement) -> bool;

    #[cfg(feature = "Document")]
    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = getSVGDocument ) ]
    ///The `getSVGDocument()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)
    ///
    ///*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*
    pub fn get_svg_document(this: &HtmlObjectElement) -> Option<Document>;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn report_validity(this: &HtmlObjectElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLObjectElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlObjectElement`*
    pub fn set_custom_validity(this: &HtmlObjectElement, error: &str);

}

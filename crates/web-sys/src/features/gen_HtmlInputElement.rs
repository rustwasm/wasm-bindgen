use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLInputElement , typescript_name = HTMLInputElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlInputElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub type HtmlInputElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = accept ) ]
    #[doc = "Getter for the `accept` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn accept(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = accept ) ]
    #[doc = "Setter for the `accept` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_accept(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = alt ) ]
    #[doc = "Getter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn alt(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = alt ) ]
    #[doc = "Setter for the `alt` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_alt(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = autocomplete ) ]
    #[doc = "Getter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn autocomplete(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = autocomplete ) ]
    #[doc = "Setter for the `autocomplete` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_autocomplete(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = autofocus ) ]
    #[doc = "Getter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn autofocus(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = autofocus ) ]
    #[doc = "Setter for the `autofocus` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_autofocus(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultChecked ) ]
    #[doc = "Getter for the `defaultChecked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn default_checked(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = defaultChecked ) ]
    #[doc = "Setter for the `defaultChecked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_default_checked(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = checked ) ]
    #[doc = "Getter for the `checked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn checked(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = checked ) ]
    #[doc = "Setter for the `checked` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_checked(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = disabled ) ]
    #[doc = "Getter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn disabled(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = disabled ) ]
    #[doc = "Setter for the `disabled` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_disabled(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = form ) ]
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlInputElement`*"]
    pub fn form(this: &HtmlInputElement) -> Option<HtmlFormElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = files ) ]
    #[cfg(feature = "FileList")]
    #[doc = "Getter for the `files` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)\n\n*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*"]
    pub fn files(this: &HtmlInputElement) -> Option<FileList>;
    # [ wasm_bindgen ( structural , method , setter , js_name = files ) ]
    #[cfg(feature = "FileList")]
    #[doc = "Setter for the `files` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)\n\n*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*"]
    pub fn set_files(this: &HtmlInputElement, value: Option<&FileList>);
    # [ wasm_bindgen ( structural , method , getter , js_name = formAction ) ]
    #[doc = "Getter for the `formAction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn form_action(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formAction ) ]
    #[doc = "Setter for the `formAction` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_form_action(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formEnctype ) ]
    #[doc = "Getter for the `formEnctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn form_enctype(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formEnctype ) ]
    #[doc = "Setter for the `formEnctype` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_form_enctype(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formMethod ) ]
    #[doc = "Getter for the `formMethod` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn form_method(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formMethod ) ]
    #[doc = "Setter for the `formMethod` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_form_method(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = formNoValidate ) ]
    #[doc = "Getter for the `formNoValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn form_no_validate(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = formNoValidate ) ]
    #[doc = "Setter for the `formNoValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_form_no_validate(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = formTarget ) ]
    #[doc = "Getter for the `formTarget` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn form_target(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = formTarget ) ]
    #[doc = "Setter for the `formTarget` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_form_target(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn height(this: &HtmlInputElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = height ) ]
    #[doc = "Setter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_height(this: &HtmlInputElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = indeterminate ) ]
    #[doc = "Getter for the `indeterminate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn indeterminate(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = indeterminate ) ]
    #[doc = "Setter for the `indeterminate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_indeterminate(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = inputMode ) ]
    #[doc = "Getter for the `inputMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn input_mode(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = inputMode ) ]
    #[doc = "Setter for the `inputMode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_input_mode(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = list ) ]
    #[cfg(feature = "HtmlElement")]
    #[doc = "Getter for the `list` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/list)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlInputElement`*"]
    pub fn list(this: &HtmlInputElement) -> Option<HtmlElement>;
    # [ wasm_bindgen ( structural , method , getter , js_name = max ) ]
    #[doc = "Getter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn max(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = max ) ]
    #[doc = "Setter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_max(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = maxLength ) ]
    #[doc = "Getter for the `maxLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn max_length(this: &HtmlInputElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = maxLength ) ]
    #[doc = "Setter for the `maxLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_max_length(this: &HtmlInputElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = min ) ]
    #[doc = "Getter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn min(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = min ) ]
    #[doc = "Setter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_min(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = minLength ) ]
    #[doc = "Getter for the `minLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn min_length(this: &HtmlInputElement) -> i32;
    # [ wasm_bindgen ( structural , method , setter , js_name = minLength ) ]
    #[doc = "Setter for the `minLength` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_min_length(this: &HtmlInputElement, value: i32);
    # [ wasm_bindgen ( structural , method , getter , js_name = multiple ) ]
    #[doc = "Getter for the `multiple` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn multiple(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = multiple ) ]
    #[doc = "Setter for the `multiple` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_multiple(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn name(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = name ) ]
    #[doc = "Setter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_name(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = pattern ) ]
    #[doc = "Getter for the `pattern` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn pattern(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = pattern ) ]
    #[doc = "Setter for the `pattern` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_pattern(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = placeholder ) ]
    #[doc = "Getter for the `placeholder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn placeholder(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = placeholder ) ]
    #[doc = "Setter for the `placeholder` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_placeholder(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = readOnly ) ]
    #[doc = "Getter for the `readOnly` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn read_only(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = readOnly ) ]
    #[doc = "Setter for the `readOnly` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_read_only(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = required ) ]
    #[doc = "Getter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn required(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = required ) ]
    #[doc = "Setter for the `required` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_required(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( structural , method , getter , js_name = size ) ]
    #[doc = "Getter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn size(this: &HtmlInputElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = size ) ]
    #[doc = "Setter for the `size` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_size(this: &HtmlInputElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = src ) ]
    #[doc = "Getter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn src(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = src ) ]
    #[doc = "Setter for the `src` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_src(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = step ) ]
    #[doc = "Getter for the `step` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn step(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = step ) ]
    #[doc = "Setter for the `step` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_step(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = type ) ]
    #[doc = "Getter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn type_(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = type ) ]
    #[doc = "Setter for the `type` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_type(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = defaultValue ) ]
    #[doc = "Getter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn default_value(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = defaultValue ) ]
    #[doc = "Setter for the `defaultValue` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_default_value(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = value ) ]
    #[doc = "Getter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn value(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = value ) ]
    #[doc = "Setter for the `value` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_value(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = valueAsNumber ) ]
    #[doc = "Getter for the `valueAsNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn value_as_number(this: &HtmlInputElement) -> f64;
    # [ wasm_bindgen ( structural , method , setter , js_name = valueAsNumber ) ]
    #[doc = "Setter for the `valueAsNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_value_as_number(this: &HtmlInputElement, value: f64);
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn width(this: &HtmlInputElement) -> u32;
    # [ wasm_bindgen ( structural , method , setter , js_name = width ) ]
    #[doc = "Setter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_width(this: &HtmlInputElement, value: u32);
    # [ wasm_bindgen ( structural , method , getter , js_name = willValidate ) ]
    #[doc = "Getter for the `willValidate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn will_validate(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = validity ) ]
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`, `ValidityState`*"]
    pub fn validity(this: &HtmlInputElement) -> ValidityState;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = validationMessage ) ]
    #[doc = "Getter for the `validationMessage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn validation_message(this: &HtmlInputElement) -> Result<String, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = labels ) ]
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`, `NodeList`*"]
    pub fn labels(this: &HtmlInputElement) -> Option<NodeList>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = selectionStart ) ]
    #[doc = "Getter for the `selectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn selection_start(this: &HtmlInputElement) -> Result<Option<u32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = selectionStart ) ]
    #[doc = "Setter for the `selectionStart` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_selection_start(this: &HtmlInputElement, value: Option<u32>) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = selectionEnd ) ]
    #[doc = "Getter for the `selectionEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn selection_end(this: &HtmlInputElement) -> Result<Option<u32>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = selectionEnd ) ]
    #[doc = "Setter for the `selectionEnd` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_selection_end(this: &HtmlInputElement, value: Option<u32>) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_name = selectionDirection ) ]
    #[doc = "Getter for the `selectionDirection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn selection_direction(this: &HtmlInputElement) -> Result<Option<String>, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , setter , js_name = selectionDirection ) ]
    #[doc = "Setter for the `selectionDirection` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_selection_direction(
        this: &HtmlInputElement,
        value: Option<&str>,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_name = align ) ]
    #[doc = "Getter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn align(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = align ) ]
    #[doc = "Setter for the `align` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_align(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = useMap ) ]
    #[doc = "Getter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn use_map(this: &HtmlInputElement) -> String;
    # [ wasm_bindgen ( structural , method , setter , js_name = useMap ) ]
    #[doc = "Setter for the `useMap` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_use_map(this: &HtmlInputElement, value: &str);
    # [ wasm_bindgen ( structural , method , getter , js_name = webkitEntries ) ]
    #[doc = "Getter for the `webkitEntries` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitEntries)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn webkit_entries(this: &HtmlInputElement) -> ::js_sys::Array;
    # [ wasm_bindgen ( structural , method , getter , js_name = webkitdirectory ) ]
    #[doc = "Getter for the `webkitdirectory` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn webkitdirectory(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( structural , method , setter , js_name = webkitdirectory ) ]
    #[doc = "Setter for the `webkitdirectory` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_webkitdirectory(this: &HtmlInputElement, value: bool);
    # [ wasm_bindgen ( method , structural , js_name = checkValidity ) ]
    #[doc = "The `checkValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn check_validity(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = reportValidity ) ]
    #[doc = "The `reportValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn report_validity(this: &HtmlInputElement) -> bool;
    # [ wasm_bindgen ( method , structural , js_name = select ) ]
    #[doc = "The `select()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn select(this: &HtmlInputElement);
    # [ wasm_bindgen ( method , structural , js_name = setCustomValidity ) ]
    #[doc = "The `setCustomValidity()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_custom_validity(this: &HtmlInputElement, error: &str);
    # [ wasm_bindgen ( catch , method , structural , js_name = setRangeText ) ]
    #[doc = "The `setRangeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_range_text(this: &HtmlInputElement, replacement: &str) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setRangeText ) ]
    #[doc = "The `setRangeText()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_range_text_with_start_and_end(
        this: &HtmlInputElement,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setSelectionRange ) ]
    #[doc = "The `setSelectionRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_selection_range(
        this: &HtmlInputElement,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = setSelectionRange ) ]
    #[doc = "The `setSelectionRange()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    pub fn set_selection_range_with_direction(
        this: &HtmlInputElement,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), JsValue>;
}

use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLInputElement , typescript_type = "HTMLInputElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `HtmlInputElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub type HtmlInputElement;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = accept ) ]
    ///Getter for the `accept` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn accept(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = accept ) ]
    ///Setter for the `accept` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_accept(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = alt ) ]
    ///Getter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn alt(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = alt ) ]
    ///Setter for the `alt` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_alt(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = autocomplete ) ]
    ///Getter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn autocomplete(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = autocomplete ) ]
    ///Setter for the `autocomplete` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_autocomplete(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = autofocus ) ]
    ///Getter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn autofocus(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = autofocus ) ]
    ///Setter for the `autofocus` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_autofocus(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = defaultChecked ) ]
    ///Getter for the `defaultChecked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn default_checked(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = defaultChecked ) ]
    ///Setter for the `defaultChecked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_default_checked(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = checked ) ]
    ///Getter for the `checked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn checked(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = checked ) ]
    ///Setter for the `checked` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_checked(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = disabled ) ]
    ///Getter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn disabled(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = disabled ) ]
    ///Setter for the `disabled` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_disabled(this: &HtmlInputElement, value: bool);

    #[cfg(feature = "HtmlFormElement")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = form ) ]
    ///Getter for the `form` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/form)
    ///
    ///*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlInputElement`*
    pub fn form(this: &HtmlInputElement) -> Option<HtmlFormElement>;

    #[cfg(feature = "FileList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = files ) ]
    ///Getter for the `files` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)
    ///
    ///*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*
    pub fn files(this: &HtmlInputElement) -> Option<FileList>;

    #[cfg(feature = "FileList")]
    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = files ) ]
    ///Setter for the `files` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)
    ///
    ///*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*
    pub fn set_files(this: &HtmlInputElement, value: Option<&FileList>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = formAction ) ]
    ///Getter for the `formAction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn form_action(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = formAction ) ]
    ///Setter for the `formAction` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_form_action(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = formEnctype ) ]
    ///Getter for the `formEnctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn form_enctype(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = formEnctype ) ]
    ///Setter for the `formEnctype` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_form_enctype(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = formMethod ) ]
    ///Getter for the `formMethod` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn form_method(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = formMethod ) ]
    ///Setter for the `formMethod` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_form_method(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = formNoValidate ) ]
    ///Getter for the `formNoValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn form_no_validate(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = formNoValidate ) ]
    ///Setter for the `formNoValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_form_no_validate(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = formTarget ) ]
    ///Getter for the `formTarget` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn form_target(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = formTarget ) ]
    ///Setter for the `formTarget` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_form_target(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn height(this: &HtmlInputElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = height ) ]
    ///Setter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_height(this: &HtmlInputElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = indeterminate ) ]
    ///Getter for the `indeterminate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn indeterminate(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = indeterminate ) ]
    ///Setter for the `indeterminate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_indeterminate(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = inputMode ) ]
    ///Getter for the `inputMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn input_mode(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = inputMode ) ]
    ///Setter for the `inputMode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_input_mode(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = list ) ]
    ///Getter for the `list` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/list)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn list(this: &HtmlInputElement) -> Option<HtmlElement>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = max ) ]
    ///Getter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn max(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = max ) ]
    ///Setter for the `max` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_max(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = maxLength ) ]
    ///Getter for the `maxLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn max_length(this: &HtmlInputElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = maxLength ) ]
    ///Setter for the `maxLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_max_length(this: &HtmlInputElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = min ) ]
    ///Getter for the `min` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn min(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = min ) ]
    ///Setter for the `min` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_min(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = minLength ) ]
    ///Getter for the `minLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn min_length(this: &HtmlInputElement) -> i32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = minLength ) ]
    ///Setter for the `minLength` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_min_length(this: &HtmlInputElement, value: i32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = multiple ) ]
    ///Getter for the `multiple` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn multiple(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = multiple ) ]
    ///Setter for the `multiple` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_multiple(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn name(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = name ) ]
    ///Setter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_name(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = pattern ) ]
    ///Getter for the `pattern` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn pattern(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = pattern ) ]
    ///Setter for the `pattern` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_pattern(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = placeholder ) ]
    ///Getter for the `placeholder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn placeholder(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = placeholder ) ]
    ///Setter for the `placeholder` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_placeholder(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = readOnly ) ]
    ///Getter for the `readOnly` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn read_only(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = readOnly ) ]
    ///Setter for the `readOnly` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_read_only(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = required ) ]
    ///Getter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn required(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = required ) ]
    ///Setter for the `required` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_required(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn size(this: &HtmlInputElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = size ) ]
    ///Setter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_size(this: &HtmlInputElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = src ) ]
    ///Getter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn src(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = src ) ]
    ///Setter for the `src` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_src(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = step ) ]
    ///Getter for the `step` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn step(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = step ) ]
    ///Setter for the `step` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_step(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn type_(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = type ) ]
    ///Setter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_type(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = defaultValue ) ]
    ///Getter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn default_value(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = defaultValue ) ]
    ///Setter for the `defaultValue` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_default_value(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = value ) ]
    ///Getter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn value(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = value ) ]
    ///Setter for the `value` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_value(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = valueAsNumber ) ]
    ///Getter for the `valueAsNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn value_as_number(this: &HtmlInputElement) -> f64;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = valueAsNumber ) ]
    ///Setter for the `valueAsNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_value_as_number(this: &HtmlInputElement, value: f64);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn width(this: &HtmlInputElement) -> u32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = width ) ]
    ///Setter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_width(this: &HtmlInputElement, value: u32);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = willValidate ) ]
    ///Getter for the `willValidate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/willValidate)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn will_validate(this: &HtmlInputElement) -> bool;

    #[cfg(feature = "ValidityState")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = validity ) ]
    ///Getter for the `validity` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`, `ValidityState`*
    pub fn validity(this: &HtmlInputElement) -> ValidityState;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLInputElement" , js_name = validationMessage ) ]
    ///Getter for the `validationMessage` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validationMessage)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn validation_message(this: &HtmlInputElement) -> Result<String, JsValue>;

    #[cfg(feature = "NodeList")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = labels ) ]
    ///Getter for the `labels` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`, `NodeList`*
    pub fn labels(this: &HtmlInputElement) -> Option<NodeList>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLInputElement" , js_name = selectionStart ) ]
    ///Getter for the `selectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn selection_start(this: &HtmlInputElement) -> Result<Option<u32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLInputElement" , js_name = selectionStart ) ]
    ///Setter for the `selectionStart` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_selection_start(this: &HtmlInputElement, value: Option<u32>) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLInputElement" , js_name = selectionEnd ) ]
    ///Getter for the `selectionEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn selection_end(this: &HtmlInputElement) -> Result<Option<u32>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLInputElement" , js_name = selectionEnd ) ]
    ///Setter for the `selectionEnd` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_selection_end(this: &HtmlInputElement, value: Option<u32>) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "HTMLInputElement" , js_name = selectionDirection ) ]
    ///Getter for the `selectionDirection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn selection_direction(this: &HtmlInputElement) -> Result<Option<String>, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , setter , js_class = "HTMLInputElement" , js_name = selectionDirection ) ]
    ///Setter for the `selectionDirection` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_selection_direction(
        this: &HtmlInputElement,
        value: Option<&str>,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = align ) ]
    ///Getter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn align(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = align ) ]
    ///Setter for the `align` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_align(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = useMap ) ]
    ///Getter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn use_map(this: &HtmlInputElement) -> String;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = useMap ) ]
    ///Setter for the `useMap` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_use_map(this: &HtmlInputElement, value: &str);

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = webkitEntries ) ]
    ///Getter for the `webkitEntries` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitEntries)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn webkit_entries(this: &HtmlInputElement) -> ::js_sys::Array;

    # [ wasm_bindgen ( structural , method , getter , js_class = "HTMLInputElement" , js_name = webkitdirectory ) ]
    ///Getter for the `webkitdirectory` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn webkitdirectory(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( structural , method , setter , js_class = "HTMLInputElement" , js_name = webkitdirectory ) ]
    ///Setter for the `webkitdirectory` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_webkitdirectory(this: &HtmlInputElement, value: bool);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLInputElement" , js_name = checkValidity ) ]
    ///The `checkValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn check_validity(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLInputElement" , js_name = reportValidity ) ]
    ///The `reportValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn report_validity(this: &HtmlInputElement) -> bool;

    # [ wasm_bindgen ( method , structural , js_class = "HTMLInputElement" , js_name = select ) ]
    ///The `select()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn select(this: &HtmlInputElement);

    # [ wasm_bindgen ( method , structural , js_class = "HTMLInputElement" , js_name = setCustomValidity ) ]
    ///The `setCustomValidity()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setCustomValidity)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_custom_validity(this: &HtmlInputElement, error: &str);

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLInputElement" , js_name = setRangeText ) ]
    ///The `setRangeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_range_text(this: &HtmlInputElement, replacement: &str) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLInputElement" , js_name = setRangeText ) ]
    ///The `setRangeText()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_range_text_with_start_and_end(
        this: &HtmlInputElement,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLInputElement" , js_name = setSelectionRange ) ]
    ///The `setSelectionRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_selection_range(
        this: &HtmlInputElement,
        start: u32,
        end: u32,
    ) -> Result<(), JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "HTMLInputElement" , js_name = setSelectionRange ) ]
    ///The `setSelectionRange()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)
    ///
    ///*This API requires the following crate features to be activated: `HtmlInputElement`*
    pub fn set_selection_range_with_direction(
        this: &HtmlInputElement,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), JsValue>;

}

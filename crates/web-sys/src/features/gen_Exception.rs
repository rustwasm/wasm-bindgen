use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Exception , typescript_name = Exception ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Exception` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub type Exception;
    # [ wasm_bindgen ( structural , method , getter , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/name)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn name(this: &Exception) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/message)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn message(this: &Exception) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = result ) ]
    #[doc = "Getter for the `result` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/result)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn result(this: &Exception) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = filename ) ]
    #[doc = "Getter for the `filename` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/filename)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn filename(this: &Exception) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_name = lineNumber ) ]
    #[doc = "Getter for the `lineNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/lineNumber)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn line_number(this: &Exception) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = columnNumber ) ]
    #[doc = "Getter for the `columnNumber` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/columnNumber)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn column_number(this: &Exception) -> u32;
    # [ wasm_bindgen ( structural , method , getter , js_name = data ) ]
    #[doc = "Getter for the `data` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/data)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn data(this: &Exception) -> Option<::js_sys::Object>;
    # [ wasm_bindgen ( structural , method , getter , js_name = stack ) ]
    #[doc = "Getter for the `stack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/stack)\n\n*This API requires the following crate features to be activated: `Exception`*"]
    pub fn stack(this: &Exception) -> String;
}

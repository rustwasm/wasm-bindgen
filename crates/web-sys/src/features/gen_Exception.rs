use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = Exception , typescript_name = Exception ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Exception` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub type Exception;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/name)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn name(this: &Exception) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/message)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn message(this: &Exception) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = result ) ]
    ///Getter for the `result` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/result)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn result(this: &Exception) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = filename ) ]
    ///Getter for the `filename` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/filename)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn filename(this: &Exception) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = lineNumber ) ]
    ///Getter for the `lineNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/lineNumber)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn line_number(this: &Exception) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = columnNumber ) ]
    ///Getter for the `columnNumber` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/columnNumber)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn column_number(this: &Exception) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = data ) ]
    ///Getter for the `data` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/data)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn data(this: &Exception) -> Option<::js_sys::Object>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "Exception" , js_name = stack ) ]
    ///Getter for the `stack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/stack)
    ///
    ///*This API requires the following crate features to be activated: `Exception`*
    pub fn stack(this: &Exception) -> String;

}

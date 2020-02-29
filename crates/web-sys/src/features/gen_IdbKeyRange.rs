use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBKeyRange , typescript_type = "IDBKeyRange" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `IdbKeyRange` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub type IdbKeyRange;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBKeyRange" , js_name = lower ) ]
    ///Getter for the `lower` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn lower(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBKeyRange" , js_name = upper ) ]
    ///Getter for the `upper` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn upper(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBKeyRange" , js_name = lowerOpen ) ]
    ///Getter for the `lowerOpen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn lower_open(this: &IdbKeyRange) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBKeyRange" , js_name = upperOpen ) ]
    ///Getter for the `upperOpen` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn upper_open(this: &IdbKeyRange) -> bool;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    ///The `bound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    ///The `bound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    ///The `bound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBKeyRange" , js_name = includes ) ]
    ///The `includes()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn includes(this: &IdbKeyRange, key: &::wasm_bindgen::JsValue) -> Result<bool, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = lowerBound ) ]
    ///The `lowerBound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn lower_bound(lower: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = lowerBound ) ]
    ///The `lowerBound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn lower_bound_with_open(
        lower: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = only ) ]
    ///The `only()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn only(value: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = upperBound ) ]
    ///The `upperBound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn upper_bound(upper: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;

    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = upperBound ) ]
    ///The `upperBound()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)
    ///
    ///*This API requires the following crate features to be activated: `IdbKeyRange`*
    pub fn upper_bound_with_open(
        upper: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;

}

use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = IDBKeyRange , typescript_name = IDBKeyRange ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbKeyRange` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub type IdbKeyRange;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBKeyRange" , js_name = lower ) ]
    #[doc = "Getter for the `lower` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , catch , method , getter , js_class = "IDBKeyRange" , js_name = upper ) ]
    #[doc = "Getter for the `upper` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper(this: &IdbKeyRange) -> Result<::wasm_bindgen::JsValue, JsValue>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBKeyRange" , js_name = lowerOpen ) ]
    #[doc = "Getter for the `lowerOpen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_open(this: &IdbKeyRange) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_class = "IDBKeyRange" , js_name = upperOpen ) ]
    #[doc = "Getter for the `upperOpen` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_open(this: &IdbKeyRange) -> bool;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
    ) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound_with_lower_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = bound ) ]
    #[doc = "The `bound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn bound_with_lower_open_and_upper_open(
        lower: &::wasm_bindgen::JsValue,
        upper: &::wasm_bindgen::JsValue,
        lower_open: bool,
        upper_open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_class = "IDBKeyRange" , js_name = includes ) ]
    #[doc = "The `includes()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn includes(this: &IdbKeyRange, key: &::wasm_bindgen::JsValue) -> Result<bool, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = lowerBound ) ]
    #[doc = "The `lowerBound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_bound(lower: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = lowerBound ) ]
    #[doc = "The `lowerBound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn lower_bound_with_open(
        lower: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = only ) ]
    #[doc = "The `only()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn only(value: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = upperBound ) ]
    #[doc = "The `upperBound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_bound(upper: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue>;
    # [ wasm_bindgen ( catch , static_method_of = IdbKeyRange , js_class = "IDBKeyRange" , js_name = upperBound ) ]
    #[doc = "The `upperBound()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound)\n\n*This API requires the following crate features to be activated: `IdbKeyRange`*"]
    pub fn upper_bound_with_open(
        upper: &::wasm_bindgen::JsValue,
        open: bool,
    ) -> Result<IdbKeyRange, JsValue>;
}

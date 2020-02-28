use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ValidityState , typescript_name = ValidityState ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ValidityState` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub type ValidityState;
    # [ wasm_bindgen ( structural , method , getter , js_name = valueMissing ) ]
    #[doc = "Getter for the `valueMissing` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn value_missing(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = typeMismatch ) ]
    #[doc = "Getter for the `typeMismatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn type_mismatch(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = patternMismatch ) ]
    #[doc = "Getter for the `patternMismatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn pattern_mismatch(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = tooLong ) ]
    #[doc = "Getter for the `tooLong` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn too_long(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = tooShort ) ]
    #[doc = "Getter for the `tooShort` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn too_short(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = rangeUnderflow ) ]
    #[doc = "Getter for the `rangeUnderflow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn range_underflow(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = rangeOverflow ) ]
    #[doc = "Getter for the `rangeOverflow` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn range_overflow(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = stepMismatch ) ]
    #[doc = "Getter for the `stepMismatch` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn step_mismatch(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = badInput ) ]
    #[doc = "Getter for the `badInput` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn bad_input(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = customError ) ]
    #[doc = "Getter for the `customError` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn custom_error(this: &ValidityState) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = valid ) ]
    #[doc = "Getter for the `valid` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid)\n\n*This API requires the following crate features to be activated: `ValidityState`*"]
    pub fn valid(this: &ValidityState) -> bool;
}

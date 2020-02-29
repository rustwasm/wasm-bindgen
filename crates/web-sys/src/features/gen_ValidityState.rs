use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ValidityState , typescript_name = ValidityState ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ValidityState` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub type ValidityState;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = valueMissing ) ]
    ///Getter for the `valueMissing` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valueMissing)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn value_missing(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = typeMismatch ) ]
    ///Getter for the `typeMismatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/typeMismatch)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn type_mismatch(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = patternMismatch ) ]
    ///Getter for the `patternMismatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/patternMismatch)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn pattern_mismatch(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = tooLong ) ]
    ///Getter for the `tooLong` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooLong)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn too_long(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = tooShort ) ]
    ///Getter for the `tooShort` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/tooShort)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn too_short(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = rangeUnderflow ) ]
    ///Getter for the `rangeUnderflow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeUnderflow)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn range_underflow(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = rangeOverflow ) ]
    ///Getter for the `rangeOverflow` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/rangeOverflow)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn range_overflow(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = stepMismatch ) ]
    ///Getter for the `stepMismatch` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/stepMismatch)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn step_mismatch(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = badInput ) ]
    ///Getter for the `badInput` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/badInput)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn bad_input(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = customError ) ]
    ///Getter for the `customError` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/customError)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn custom_error(this: &ValidityState) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ValidityState" , js_name = valid ) ]
    ///Getter for the `valid` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ValidityState/valid)
    ///
    ///*This API requires the following crate features to be activated: `ValidityState`*
    pub fn valid(this: &ValidityState) -> bool;

}

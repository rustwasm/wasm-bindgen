use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = PerformanceNavigation , typescript_name = PerformanceNavigation ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PerformanceNavigation` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*
    pub type PerformanceNavigation;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigation" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/type)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*
    pub fn type_(this: &PerformanceNavigation) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "PerformanceNavigation" , js_name = redirectCount ) ]
    ///Getter for the `redirectCount` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/redirectCount)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*
    pub fn redirect_count(this: &PerformanceNavigation) -> u16;

    # [ wasm_bindgen ( method , structural , js_class = "PerformanceNavigation" , js_name = toJSON ) ]
    ///The `toJSON()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/toJSON)
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*
    pub fn to_json(this: &PerformanceNavigation) -> ::js_sys::Object;

}

impl PerformanceNavigation {
    ///The `PerformanceNavigation.TYPE_NAVIGATE` const.
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*

    pub const TYPE_NAVIGATE: u16 = 0i64 as u16;

    ///The `PerformanceNavigation.TYPE_RELOAD` const.
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*

    pub const TYPE_RELOAD: u16 = 1u64 as u16;

    ///The `PerformanceNavigation.TYPE_BACK_FORWARD` const.
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*

    pub const TYPE_BACK_FORWARD: u16 = 2u64 as u16;

    ///The `PerformanceNavigation.TYPE_RESERVED` const.
    ///
    ///*This API requires the following crate features to be activated: `PerformanceNavigation`*

    pub const TYPE_RESERVED: u16 = 255u64 as u16;
}

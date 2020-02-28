use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = ScreenLuminance , typescript_name = ScreenLuminance ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenLuminance` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub type ScreenLuminance;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ScreenLuminance" , js_name = min ) ]
    #[doc = "Getter for the `min` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/min)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn min(this: &ScreenLuminance) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ScreenLuminance" , js_name = max ) ]
    #[doc = "Getter for the `max` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/max)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn max(this: &ScreenLuminance) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_class = "ScreenLuminance" , js_name = maxAverage ) ]
    #[doc = "Getter for the `maxAverage` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/maxAverage)\n\n*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn max_average(this: &ScreenLuminance) -> f64;
}

use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = BatteryManager , typescript_name = BatteryManager ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BatteryManager` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub type BatteryManager;
    # [ wasm_bindgen ( structural , method , getter , js_name = charging ) ]
    #[doc = "Getter for the `charging` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn charging(this: &BatteryManager) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = chargingTime ) ]
    #[doc = "Getter for the `chargingTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn charging_time(this: &BatteryManager) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = dischargingTime ) ]
    #[doc = "Getter for the `dischargingTime` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn discharging_time(this: &BatteryManager) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = level ) ]
    #[doc = "Getter for the `level` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn level(this: &BatteryManager) -> f64;
    # [ wasm_bindgen ( structural , method , getter , js_name = onchargingchange ) ]
    #[doc = "Getter for the `onchargingchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onchargingchange(this: &BatteryManager) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchargingchange ) ]
    #[doc = "Setter for the `onchargingchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onchargingchange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onchargingtimechange ) ]
    #[doc = "Getter for the `onchargingtimechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onchargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onchargingtimechange ) ]
    #[doc = "Setter for the `onchargingtimechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onchargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = ondischargingtimechange ) ]
    #[doc = "Getter for the `ondischargingtimechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn ondischargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = ondischargingtimechange ) ]
    #[doc = "Setter for the `ondischargingtimechange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_ondischargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onlevelchange ) ]
    #[doc = "Getter for the `onlevelchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onlevelchange(this: &BatteryManager) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onlevelchange ) ]
    #[doc = "Setter for the `onlevelchange` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onlevelchange(this: &BatteryManager, value: Option<&::js_sys::Function>);
}

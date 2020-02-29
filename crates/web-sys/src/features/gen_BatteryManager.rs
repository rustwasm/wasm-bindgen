use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = BatteryManager , typescript_name = BatteryManager ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BatteryManager` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub type BatteryManager;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = charging ) ]
    ///Getter for the `charging` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn charging(this: &BatteryManager) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = chargingTime ) ]
    ///Getter for the `chargingTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn charging_time(this: &BatteryManager) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = dischargingTime ) ]
    ///Getter for the `dischargingTime` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn discharging_time(this: &BatteryManager) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = level ) ]
    ///Getter for the `level` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn level(this: &BatteryManager) -> f64;

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = onchargingchange ) ]
    ///Getter for the `onchargingchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn onchargingchange(this: &BatteryManager) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BatteryManager" , js_name = onchargingchange ) ]
    ///Setter for the `onchargingchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn set_onchargingchange(this: &BatteryManager, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = onchargingtimechange ) ]
    ///Getter for the `onchargingtimechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn onchargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BatteryManager" , js_name = onchargingtimechange ) ]
    ///Setter for the `onchargingtimechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn set_onchargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = ondischargingtimechange ) ]
    ///Getter for the `ondischargingtimechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn ondischargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BatteryManager" , js_name = ondischargingtimechange ) ]
    ///Setter for the `ondischargingtimechange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn set_ondischargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "BatteryManager" , js_name = onlevelchange ) ]
    ///Getter for the `onlevelchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn onlevelchange(this: &BatteryManager) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "BatteryManager" , js_name = onlevelchange ) ]
    ///Setter for the `onlevelchange` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)
    ///
    ///*This API requires the following crate features to be activated: `BatteryManager`*
    pub fn set_onlevelchange(this: &BatteryManager, value: Option<&::js_sys::Function>);

}

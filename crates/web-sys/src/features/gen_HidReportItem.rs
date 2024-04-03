#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDReportItem)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidReportItem` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidReportItem;
    #[wasm_bindgen(method, setter = "hasNull")]
    fn has_null_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "hasPreferredState")]
    fn has_preferred_state_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isAbsolute")]
    fn is_absolute_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isArray")]
    fn is_array_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isBufferedBytes")]
    fn is_buffered_bytes_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isConstant")]
    fn is_constant_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isLinear")]
    fn is_linear_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isRange")]
    fn is_range_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "isVolatile")]
    fn is_volatile_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, setter = "logicalMaximum")]
    fn logical_maximum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, setter = "logicalMinimum")]
    fn logical_minimum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, setter = "physicalMaximum")]
    fn physical_maximum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, setter = "physicalMinimum")]
    fn physical_minimum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, setter = "reportCount")]
    fn report_count_shim(this: &HidReportItem, val: u16);
    #[wasm_bindgen(method, setter = "reportSize")]
    fn report_size_shim(this: &HidReportItem, val: u16);
    #[wasm_bindgen(method, setter = "strings")]
    fn strings_shim(this: &HidReportItem, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "unitExponent")]
    fn unit_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorCurrentExponent")]
    fn unit_factor_current_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorLengthExponent")]
    fn unit_factor_length_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorLuminousIntensityExponent")]
    fn unit_factor_luminous_intensity_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorMassExponent")]
    fn unit_factor_mass_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorTemperatureExponent")]
    fn unit_factor_temperature_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, setter = "unitFactorTimeExponent")]
    fn unit_factor_time_exponent_shim(this: &HidReportItem, val: i8);
    #[cfg(feature = "HidUnitSystem")]
    #[wasm_bindgen(method, setter = "unitSystem")]
    fn unit_system_shim(this: &HidReportItem, val: HidUnitSystem);
    #[wasm_bindgen(method, setter = "usageMaximum")]
    fn usage_maximum_shim(this: &HidReportItem, val: u32);
    #[wasm_bindgen(method, setter = "usageMinimum")]
    fn usage_minimum_shim(this: &HidReportItem, val: u32);
    #[wasm_bindgen(method, setter = "usages")]
    fn usages_shim(this: &HidReportItem, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "wrap")]
    fn wrap_shim(this: &HidReportItem, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl HidReportItem {
    #[doc = "Construct a new `HidReportItem`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasNull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has_null(&mut self, val: bool) -> &mut Self {
        self.has_null_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasPreferredState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has_preferred_state(&mut self, val: bool) -> &mut Self {
        self.has_preferred_state_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isAbsolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_absolute(&mut self, val: bool) -> &mut Self {
        self.is_absolute_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isArray` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_array(&mut self, val: bool) -> &mut Self {
        self.is_array_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isBufferedBytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_buffered_bytes(&mut self, val: bool) -> &mut Self {
        self.is_buffered_bytes_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_constant(&mut self, val: bool) -> &mut Self {
        self.is_constant_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isLinear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_linear(&mut self, val: bool) -> &mut Self {
        self.is_linear_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_range(&mut self, val: bool) -> &mut Self {
        self.is_range_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isVolatile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_volatile(&mut self, val: bool) -> &mut Self {
        self.is_volatile_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `logicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn logical_maximum(&mut self, val: i32) -> &mut Self {
        self.logical_maximum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `logicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn logical_minimum(&mut self, val: i32) -> &mut Self {
        self.logical_minimum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `physicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn physical_maximum(&mut self, val: i32) -> &mut Self {
        self.physical_maximum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `physicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn physical_minimum(&mut self, val: i32) -> &mut Self {
        self.physical_minimum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn report_count(&mut self, val: u16) -> &mut Self {
        self.report_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn report_size(&mut self, val: u16) -> &mut Self {
        self.report_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `strings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn strings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.strings_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorCurrentExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_current_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_current_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorLengthExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_length_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_length_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorLuminousIntensityExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_luminous_intensity_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_luminous_intensity_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorMassExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_mass_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_mass_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorTemperatureExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_temperature_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_temperature_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorTimeExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_factor_time_exponent(&mut self, val: i8) -> &mut Self {
        self.unit_factor_time_exponent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    #[doc = "Change the `unitSystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`, `HidUnitSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unit_system(&mut self, val: HidUnitSystem) -> &mut Self {
        self.unit_system_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usageMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage_maximum(&mut self, val: u32) -> &mut Self {
        self.usage_maximum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usageMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage_minimum(&mut self, val: u32) -> &mut Self {
        self.usage_minimum_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usages` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usages(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.usages_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn wrap(&mut self, val: bool) -> &mut Self {
        self.wrap_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidReportItem {
    fn default() -> Self {
        Self::new()
    }
}

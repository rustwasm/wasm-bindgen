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
    #[wasm_bindgen(method, getter = "hasNull")]
    fn has_null_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "hasNull")]
    fn set_has_null_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "hasPreferredState")]
    fn has_preferred_state_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "hasPreferredState")]
    fn set_has_preferred_state_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isAbsolute")]
    fn is_absolute_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isAbsolute")]
    fn set_is_absolute_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isArray")]
    fn is_array_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isArray")]
    fn set_is_array_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isBufferedBytes")]
    fn is_buffered_bytes_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isBufferedBytes")]
    fn set_is_buffered_bytes_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isConstant")]
    fn is_constant_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isConstant")]
    fn set_is_constant_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isLinear")]
    fn is_linear_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isLinear")]
    fn set_is_linear_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isRange")]
    fn is_range_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isRange")]
    fn set_is_range_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "isVolatile")]
    fn is_volatile_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "isVolatile")]
    fn set_is_volatile_shim(this: &HidReportItem, val: bool);
    #[wasm_bindgen(method, getter = "logicalMaximum")]
    fn logical_maximum_shim(this: &HidReportItem) -> i32;
    #[wasm_bindgen(method, setter = "logicalMaximum")]
    fn set_logical_maximum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, getter = "logicalMinimum")]
    fn logical_minimum_shim(this: &HidReportItem) -> i32;
    #[wasm_bindgen(method, setter = "logicalMinimum")]
    fn set_logical_minimum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, getter = "physicalMaximum")]
    fn physical_maximum_shim(this: &HidReportItem) -> i32;
    #[wasm_bindgen(method, setter = "physicalMaximum")]
    fn set_physical_maximum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, getter = "physicalMinimum")]
    fn physical_minimum_shim(this: &HidReportItem) -> i32;
    #[wasm_bindgen(method, setter = "physicalMinimum")]
    fn set_physical_minimum_shim(this: &HidReportItem, val: i32);
    #[wasm_bindgen(method, getter = "reportCount")]
    fn report_count_shim(this: &HidReportItem) -> u16;
    #[wasm_bindgen(method, setter = "reportCount")]
    fn set_report_count_shim(this: &HidReportItem, val: u16);
    #[wasm_bindgen(method, getter = "reportSize")]
    fn report_size_shim(this: &HidReportItem) -> u16;
    #[wasm_bindgen(method, setter = "reportSize")]
    fn set_report_size_shim(this: &HidReportItem, val: u16);
    #[wasm_bindgen(method, getter = "strings")]
    fn strings_shim(this: &HidReportItem) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "strings")]
    fn set_strings_shim(this: &HidReportItem, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "unitExponent")]
    fn unit_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitExponent")]
    fn set_unit_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorCurrentExponent")]
    fn unit_factor_current_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorCurrentExponent")]
    fn set_unit_factor_current_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorLengthExponent")]
    fn unit_factor_length_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorLengthExponent")]
    fn set_unit_factor_length_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorLuminousIntensityExponent")]
    fn unit_factor_luminous_intensity_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorLuminousIntensityExponent")]
    fn set_unit_factor_luminous_intensity_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorMassExponent")]
    fn unit_factor_mass_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorMassExponent")]
    fn set_unit_factor_mass_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorTemperatureExponent")]
    fn unit_factor_temperature_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorTemperatureExponent")]
    fn set_unit_factor_temperature_exponent_shim(this: &HidReportItem, val: i8);
    #[wasm_bindgen(method, getter = "unitFactorTimeExponent")]
    fn unit_factor_time_exponent_shim(this: &HidReportItem) -> i8;
    #[wasm_bindgen(method, setter = "unitFactorTimeExponent")]
    fn set_unit_factor_time_exponent_shim(this: &HidReportItem, val: i8);
    #[cfg(feature = "HidUnitSystem")]
    #[wasm_bindgen(method, getter = "unitSystem")]
    fn unit_system_shim(this: &HidReportItem) -> HidUnitSystem;
    #[cfg(feature = "HidUnitSystem")]
    #[wasm_bindgen(method, setter = "unitSystem")]
    fn set_unit_system_shim(this: &HidReportItem, val: HidUnitSystem);
    #[wasm_bindgen(method, getter = "usageMaximum")]
    fn usage_maximum_shim(this: &HidReportItem) -> u32;
    #[wasm_bindgen(method, setter = "usageMaximum")]
    fn set_usage_maximum_shim(this: &HidReportItem, val: u32);
    #[wasm_bindgen(method, getter = "usageMinimum")]
    fn usage_minimum_shim(this: &HidReportItem) -> u32;
    #[wasm_bindgen(method, setter = "usageMinimum")]
    fn set_usage_minimum_shim(this: &HidReportItem, val: u32);
    #[wasm_bindgen(method, getter = "usages")]
    fn usages_shim(this: &HidReportItem) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "usages")]
    fn set_usages_shim(this: &HidReportItem, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "wrap")]
    fn wrap_shim(this: &HidReportItem) -> bool;
    #[wasm_bindgen(method, setter = "wrap")]
    fn set_wrap_shim(this: &HidReportItem, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidReportItem` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
pub trait HidReportItemGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasNull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn has_null(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasPreferredState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn has_preferred_state(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isAbsolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_absolute(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isArray` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_array(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isBufferedBytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_buffered_bytes(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_constant(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isLinear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_linear(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_range(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isVolatile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn is_volatile(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `logicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn logical_maximum(&self) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `logicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn logical_minimum(&self) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `physicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn physical_maximum(&self) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `physicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn physical_minimum(&self) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn report_count(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn report_size(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `strings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn strings(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorCurrentExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_current_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorLengthExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_length_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorLuminousIntensityExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_luminous_intensity_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorMassExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_mass_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorTemperatureExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_temperature_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorTimeExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_factor_time_exponent(&self) -> i8;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    #[doc = "Get the `unitSystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`, `HidUnitSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn unit_system(&self) -> HidUnitSystem;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usageMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage_maximum(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usageMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage_minimum(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usages` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usages(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn wrap(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl HidReportItemGetters for HidReportItem {
    #[cfg(web_sys_unstable_apis)]
    fn has_null(&self) -> bool {
        self.has_null_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn has_preferred_state(&self) -> bool {
        self.has_preferred_state_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_absolute(&self) -> bool {
        self.is_absolute_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_array(&self) -> bool {
        self.is_array_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_buffered_bytes(&self) -> bool {
        self.is_buffered_bytes_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_constant(&self) -> bool {
        self.is_constant_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_linear(&self) -> bool {
        self.is_linear_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_range(&self) -> bool {
        self.is_range_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn is_volatile(&self) -> bool {
        self.is_volatile_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn logical_maximum(&self) -> i32 {
        self.logical_maximum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn logical_minimum(&self) -> i32 {
        self.logical_minimum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn physical_maximum(&self) -> i32 {
        self.physical_maximum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn physical_minimum(&self) -> i32 {
        self.physical_minimum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn report_count(&self) -> u16 {
        self.report_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn report_size(&self) -> u16 {
        self.report_size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn strings(&self) -> ::js_sys::Array {
        self.strings_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_exponent(&self) -> i8 {
        self.unit_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_current_exponent(&self) -> i8 {
        self.unit_factor_current_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_length_exponent(&self) -> i8 {
        self.unit_factor_length_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_luminous_intensity_exponent(&self) -> i8 {
        self.unit_factor_luminous_intensity_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_mass_exponent(&self) -> i8 {
        self.unit_factor_mass_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_temperature_exponent(&self) -> i8 {
        self.unit_factor_temperature_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn unit_factor_time_exponent(&self) -> i8 {
        self.unit_factor_time_exponent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    fn unit_system(&self) -> HidUnitSystem {
        self.unit_system_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage_maximum(&self) -> u32 {
        self.usage_maximum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage_minimum(&self) -> u32 {
        self.usage_minimum_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usages(&self) -> ::js_sys::Array {
        self.usages_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn wrap(&self) -> bool {
        self.wrap_shim()
    }
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
        self.set_has_null_shim(val);
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
        self.set_has_preferred_state_shim(val);
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
        self.set_is_absolute_shim(val);
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
        self.set_is_array_shim(val);
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
        self.set_is_buffered_bytes_shim(val);
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
        self.set_is_constant_shim(val);
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
        self.set_is_linear_shim(val);
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
        self.set_is_range_shim(val);
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
        self.set_is_volatile_shim(val);
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
        self.set_logical_maximum_shim(val);
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
        self.set_logical_minimum_shim(val);
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
        self.set_physical_maximum_shim(val);
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
        self.set_physical_minimum_shim(val);
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
        self.set_report_count_shim(val);
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
        self.set_report_size_shim(val);
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
        self.set_strings_shim(val);
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
        self.set_unit_exponent_shim(val);
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
        self.set_unit_factor_current_exponent_shim(val);
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
        self.set_unit_factor_length_exponent_shim(val);
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
        self.set_unit_factor_luminous_intensity_exponent_shim(val);
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
        self.set_unit_factor_mass_exponent_shim(val);
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
        self.set_unit_factor_temperature_exponent_shim(val);
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
        self.set_unit_factor_time_exponent_shim(val);
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
        self.set_unit_system_shim(val);
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
        self.set_usage_maximum_shim(val);
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
        self.set_usage_minimum_shim(val);
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
        self.set_usages_shim(val);
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
        self.set_wrap_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidReportItem {
    fn default() -> Self {
        Self::new()
    }
}

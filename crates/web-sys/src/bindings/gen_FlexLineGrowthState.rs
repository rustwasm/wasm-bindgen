use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum FlexLineGrowthState {
    Unchanged = 0,
    Shrinking = 1,
    Growing = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl FlexLineGrowthState {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<FlexLineGrowthState> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "unchanged" => Some(FlexLineGrowthState::Unchanged),
            "shrinking" => Some(FlexLineGrowthState::Shrinking),
            "growing" => Some(FlexLineGrowthState::Growing),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for FlexLineGrowthState {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for FlexLineGrowthState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for FlexLineGrowthState {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        FlexLineGrowthState::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(FlexLineGrowthState::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for FlexLineGrowthState {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for FlexLineGrowthState {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<FlexLineGrowthState> for wasm_bindgen::JsValue {
    fn from(obj: FlexLineGrowthState) -> wasm_bindgen::JsValue {
        match obj {
            FlexLineGrowthState::Unchanged => wasm_bindgen::JsValue::from_str("unchanged"),
            FlexLineGrowthState::Shrinking => wasm_bindgen::JsValue::from_str("shrinking"),
            FlexLineGrowthState::Growing => wasm_bindgen::JsValue::from_str("growing"),
            FlexLineGrowthState::__Nonexhaustive => {
                panic!("attempted to convert invalid FlexLineGrowthState into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_909be965ad952fce: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

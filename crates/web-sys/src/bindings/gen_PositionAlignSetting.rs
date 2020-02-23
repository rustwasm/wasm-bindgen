use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum PositionAlignSetting {
    LineLeft = 0,
    Center = 1,
    LineRight = 2,
    Auto = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl PositionAlignSetting {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<PositionAlignSetting> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "line-left" => Some(PositionAlignSetting::LineLeft),
            "center" => Some(PositionAlignSetting::Center),
            "line-right" => Some(PositionAlignSetting::LineRight),
            "auto" => Some(PositionAlignSetting::Auto),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for PositionAlignSetting {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for PositionAlignSetting {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for PositionAlignSetting {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        PositionAlignSetting::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(PositionAlignSetting::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for PositionAlignSetting {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for PositionAlignSetting {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<PositionAlignSetting> for wasm_bindgen::JsValue {
    fn from(obj: PositionAlignSetting) -> wasm_bindgen::JsValue {
        match obj {
            PositionAlignSetting::LineLeft => wasm_bindgen::JsValue::from_str("line-left"),
            PositionAlignSetting::Center => wasm_bindgen::JsValue::from_str("center"),
            PositionAlignSetting::LineRight => wasm_bindgen::JsValue::from_str("line-right"),
            PositionAlignSetting::Auto => wasm_bindgen::JsValue::from_str("auto"),
            PositionAlignSetting::__Nonexhaustive => {
                panic!("attempted to convert invalid PositionAlignSetting into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_34811a2393be88fe: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

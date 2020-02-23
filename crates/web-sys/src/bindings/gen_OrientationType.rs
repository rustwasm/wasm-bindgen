use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum OrientationType {
    PortraitPrimary = 0,
    PortraitSecondary = 1,
    LandscapePrimary = 2,
    LandscapeSecondary = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl OrientationType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<OrientationType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "portrait-primary" => Some(OrientationType::PortraitPrimary),
            "portrait-secondary" => Some(OrientationType::PortraitSecondary),
            "landscape-primary" => Some(OrientationType::LandscapePrimary),
            "landscape-secondary" => Some(OrientationType::LandscapeSecondary),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for OrientationType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for OrientationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for OrientationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        OrientationType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(OrientationType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for OrientationType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for OrientationType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<OrientationType> for wasm_bindgen::JsValue {
    fn from(obj: OrientationType) -> wasm_bindgen::JsValue {
        match obj {
            OrientationType::PortraitPrimary => wasm_bindgen::JsValue::from_str("portrait-primary"),
            OrientationType::PortraitSecondary => {
                wasm_bindgen::JsValue::from_str("portrait-secondary")
            }
            OrientationType::LandscapePrimary => {
                wasm_bindgen::JsValue::from_str("landscape-primary")
            }
            OrientationType::LandscapeSecondary => {
                wasm_bindgen::JsValue::from_str("landscape-secondary")
            }
            OrientationType::__Nonexhaustive => {
                panic!("attempted to convert invalid OrientationType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_526ec26f8c79cd4c: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

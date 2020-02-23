use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum AttestationConveyancePreference {
    None = 0,
    Indirect = 1,
    Direct = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl AttestationConveyancePreference {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<AttestationConveyancePreference> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "none" => Some(AttestationConveyancePreference::None),
            "indirect" => Some(AttestationConveyancePreference::Indirect),
            "direct" => Some(AttestationConveyancePreference::Direct),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for AttestationConveyancePreference {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for AttestationConveyancePreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for AttestationConveyancePreference {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        AttestationConveyancePreference::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(AttestationConveyancePreference::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for AttestationConveyancePreference {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for AttestationConveyancePreference {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<AttestationConveyancePreference> for wasm_bindgen::JsValue {
    fn from(obj: AttestationConveyancePreference) -> wasm_bindgen::JsValue {
        match obj {
            AttestationConveyancePreference::None => wasm_bindgen::JsValue::from_str("none"),
            AttestationConveyancePreference::Indirect => {
                wasm_bindgen::JsValue::from_str("indirect")
            }
            AttestationConveyancePreference::Direct => wasm_bindgen::JsValue::from_str("direct"),
            AttestationConveyancePreference::__Nonexhaustive => {
                panic!("attempted to convert invalid AttestationConveyancePreference into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8ad416a52f07a393: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

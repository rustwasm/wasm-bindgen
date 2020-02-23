use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum UserVerificationRequirement {
    Required = 0,
    Preferred = 1,
    Discouraged = 2,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl UserVerificationRequirement {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<UserVerificationRequirement> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "required" => Some(UserVerificationRequirement::Required),
            "preferred" => Some(UserVerificationRequirement::Preferred),
            "discouraged" => Some(UserVerificationRequirement::Discouraged),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for UserVerificationRequirement {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for UserVerificationRequirement {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for UserVerificationRequirement {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        UserVerificationRequirement::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(UserVerificationRequirement::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for UserVerificationRequirement {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for UserVerificationRequirement {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<UserVerificationRequirement> for wasm_bindgen::JsValue {
    fn from(obj: UserVerificationRequirement) -> wasm_bindgen::JsValue {
        match obj {
            UserVerificationRequirement::Required => wasm_bindgen::JsValue::from_str("required"),
            UserVerificationRequirement::Preferred => wasm_bindgen::JsValue::from_str("preferred"),
            UserVerificationRequirement::Discouraged => {
                wasm_bindgen::JsValue::from_str("discouraged")
            }
            UserVerificationRequirement::__Nonexhaustive => {
                panic!("attempted to convert invalid UserVerificationRequirement into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6e5a4e29d8608698: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

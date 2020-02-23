use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum ProfileTimelineMessagePortOperationType {
    SerializeData = 0,
    DeserializeData = 1,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl ProfileTimelineMessagePortOperationType {
    pub fn from_js_value(
        obj: &wasm_bindgen::JsValue,
    ) -> Option<ProfileTimelineMessagePortOperationType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "serializeData" => Some(ProfileTimelineMessagePortOperationType::SerializeData),
            "deserializeData" => Some(ProfileTimelineMessagePortOperationType::DeserializeData),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for ProfileTimelineMessagePortOperationType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for ProfileTimelineMessagePortOperationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for ProfileTimelineMessagePortOperationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        ProfileTimelineMessagePortOperationType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(ProfileTimelineMessagePortOperationType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for ProfileTimelineMessagePortOperationType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for ProfileTimelineMessagePortOperationType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<ProfileTimelineMessagePortOperationType> for wasm_bindgen::JsValue {
    fn from(obj: ProfileTimelineMessagePortOperationType) -> wasm_bindgen::JsValue {
        match obj {
            ProfileTimelineMessagePortOperationType::SerializeData => {
                wasm_bindgen::JsValue::from_str("serializeData")
            }
            ProfileTimelineMessagePortOperationType::DeserializeData => {
                wasm_bindgen::JsValue::from_str("deserializeData")
            }
            ProfileTimelineMessagePortOperationType::__Nonexhaustive => panic!(
                "attempted to convert invalid ProfileTimelineMessagePortOperationType into JSValue"
            ),
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_78f0b09cc37a6211: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum NavigationType {
    Navigate = 0,
    Reload = 1,
    BackForward = 2,
    Prerender = 3,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl NavigationType {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<NavigationType> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "navigate" => Some(NavigationType::Navigate),
            "reload" => Some(NavigationType::Reload),
            "back_forward" => Some(NavigationType::BackForward),
            "prerender" => Some(NavigationType::Prerender),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for NavigationType {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for NavigationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for NavigationType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        NavigationType::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(NavigationType::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for NavigationType {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for NavigationType {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<NavigationType> for wasm_bindgen::JsValue {
    fn from(obj: NavigationType) -> wasm_bindgen::JsValue {
        match obj {
            NavigationType::Navigate => wasm_bindgen::JsValue::from_str("navigate"),
            NavigationType::Reload => wasm_bindgen::JsValue::from_str("reload"),
            NavigationType::BackForward => wasm_bindgen::JsValue::from_str("back_forward"),
            NavigationType::Prerender => wasm_bindgen::JsValue::from_str("prerender"),
            NavigationType::__Nonexhaustive => {
                panic!("attempted to convert invalid NavigationType into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8e53717a0dc1fd48: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

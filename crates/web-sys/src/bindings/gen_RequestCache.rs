use super::*;
#[allow(bad_style)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(clippy::all)]
pub enum RequestCache {
    Default = 0,
    NoStore = 1,
    Reload = 2,
    NoCache = 3,
    ForceCache = 4,
    OnlyIfCached = 5,
    #[doc(hidden)]
    __Nonexhaustive,
}
#[allow(clippy::all)]
impl RequestCache {
    pub fn from_js_value(obj: &wasm_bindgen::JsValue) -> Option<RequestCache> {
        obj.as_string().and_then(|obj_str| match obj_str.as_str() {
            "default" => Some(RequestCache::Default),
            "no-store" => Some(RequestCache::NoStore),
            "reload" => Some(RequestCache::Reload),
            "no-cache" => Some(RequestCache::NoCache),
            "force-cache" => Some(RequestCache::ForceCache),
            "only-if-cached" => Some(RequestCache::OnlyIfCached),
            _ => None,
        })
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::describe::WasmDescribe for RequestCache {
    fn describe() {
        wasm_bindgen::JsValue::describe()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::IntoWasmAbi for RequestCache {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;
    #[inline]
    fn into_abi(self) -> Self::Abi {
        wasm_bindgen::JsValue::from(self).into_abi()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::FromWasmAbi for RequestCache {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
    unsafe fn from_abi(js: Self::Abi) -> Self {
        RequestCache::from_js_value(&wasm_bindgen::JsValue::from_abi(js))
            .unwrap_or(RequestCache::__Nonexhaustive)
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionIntoWasmAbi for RequestCache {
    #[inline]
    fn none() -> Self::Abi {
        Object::none()
    }
}
#[allow(clippy::all)]
impl wasm_bindgen::convert::OptionFromWasmAbi for RequestCache {
    #[inline]
    fn is_none(abi: &Self::Abi) -> bool {
        Object::is_none(abi)
    }
}
#[allow(clippy::all)]
impl From<RequestCache> for wasm_bindgen::JsValue {
    fn from(obj: RequestCache) -> wasm_bindgen::JsValue {
        match obj {
            RequestCache::Default => wasm_bindgen::JsValue::from_str("default"),
            RequestCache::NoStore => wasm_bindgen::JsValue::from_str("no-store"),
            RequestCache::Reload => wasm_bindgen::JsValue::from_str("reload"),
            RequestCache::NoCache => wasm_bindgen::JsValue::from_str("no-cache"),
            RequestCache::ForceCache => wasm_bindgen::JsValue::from_str("force-cache"),
            RequestCache::OnlyIfCached => wasm_bindgen::JsValue::from_str("only-if-cached"),
            RequestCache::__Nonexhaustive => {
                panic!("attempted to convert invalid RequestCache into JSValue")
            }
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a24b1815721363a4: [u8; 108usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}*\0\0\0\0\0\x01\0\0\x03\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

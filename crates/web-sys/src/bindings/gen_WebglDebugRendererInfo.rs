use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_debug_renderer_info` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_renderer_info)\n\n*This API requires the following crate features to be activated: `WebglDebugRendererInfo`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglDebugRendererInfo {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglDebugRendererInfo: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglDebugRendererInfo {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(87u32);
            inform(69u32);
            inform(66u32);
            inform(71u32);
            inform(76u32);
            inform(95u32);
            inform(100u32);
            inform(101u32);
            inform(98u32);
            inform(117u32);
            inform(103u32);
            inform(95u32);
            inform(114u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(101u32);
            inform(114u32);
            inform(95u32);
            inform(105u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for WebglDebugRendererInfo {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglDebugRendererInfo {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglDebugRendererInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglDebugRendererInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglDebugRendererInfo {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglDebugRendererInfo {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglDebugRendererInfo {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglDebugRendererInfo {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglDebugRendererInfo {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglDebugRendererInfo>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglDebugRendererInfo {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglDebugRendererInfo {
        #[inline]
        fn from(obj: JsValue) -> WebglDebugRendererInfo {
            WebglDebugRendererInfo { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglDebugRendererInfo {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglDebugRendererInfo> for WebglDebugRendererInfo {
        #[inline]
        fn as_ref(&self) -> &WebglDebugRendererInfo {
            self
        }
    }
    impl From<WebglDebugRendererInfo> for JsValue {
        #[inline]
        fn from(obj: WebglDebugRendererInfo) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglDebugRendererInfo {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_debug_renderer_info(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_debug_renderer_info(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_debug_renderer_info(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglDebugRendererInfo { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglDebugRendererInfo) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglDebugRendererInfo> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglDebugRendererInfo) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglDebugRendererInfo {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl WebglDebugRendererInfo {
    pub const UNMASKED_VENDOR_WEBGL: u32 = 37445u64 as u32;
}
impl WebglDebugRendererInfo {
    pub const UNMASKED_RENDERER_WEBGL: u32 = 37446u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_26f543707f68a24f: [u8; 179usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}q\0\0\0\0\0\x01\0\0\x02\x19WEBGL_debug_renderer_info+__widl_instanceof_WEBGL_debug_renderer_info\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

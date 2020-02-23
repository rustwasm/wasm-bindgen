use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebGLFramebuffer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebGlFramebuffer {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebGlFramebuffer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebGlFramebuffer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(71u32);
            inform(76u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(98u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for WebGlFramebuffer {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebGlFramebuffer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebGlFramebuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebGlFramebuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebGlFramebuffer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebGlFramebuffer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebGlFramebuffer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebGlFramebuffer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebGlFramebuffer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebGlFramebuffer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebGlFramebuffer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebGlFramebuffer {
        #[inline]
        fn from(obj: JsValue) -> WebGlFramebuffer {
            WebGlFramebuffer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebGlFramebuffer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebGlFramebuffer> for WebGlFramebuffer {
        #[inline]
        fn as_ref(&self) -> &WebGlFramebuffer {
            self
        }
    }
    impl From<WebGlFramebuffer> for JsValue {
        #[inline]
        fn from(obj: WebGlFramebuffer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebGlFramebuffer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebGLFramebuffer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebGLFramebuffer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebGLFramebuffer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebGlFramebuffer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebGlFramebuffer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebGlFramebuffer> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebGlFramebuffer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebGlFramebuffer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5f015bc9cb53d9ad: [u8; 161usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\0\0\0\0\0\x01\0\0\x02\x10WebGLFramebuffer\"__widl_instanceof_WebGLFramebuffer\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

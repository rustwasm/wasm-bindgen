use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_lose_context` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglLoseContext {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglLoseContext: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglLoseContext {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(87u32);
            inform(69u32);
            inform(66u32);
            inform(71u32);
            inform(76u32);
            inform(95u32);
            inform(108u32);
            inform(111u32);
            inform(115u32);
            inform(101u32);
            inform(95u32);
            inform(99u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WebglLoseContext {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglLoseContext {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglLoseContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglLoseContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglLoseContext {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglLoseContext {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglLoseContext {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglLoseContext {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglLoseContext {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglLoseContext>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglLoseContext {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglLoseContext {
        #[inline]
        fn from(obj: JsValue) -> WebglLoseContext {
            WebglLoseContext { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglLoseContext {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglLoseContext> for WebglLoseContext {
        #[inline]
        fn as_ref(&self) -> &WebglLoseContext {
            self
        }
    }
    impl From<WebglLoseContext> for JsValue {
        #[inline]
        fn from(obj: WebglLoseContext) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglLoseContext {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_lose_context(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_lose_context(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_lose_context(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglLoseContext { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglLoseContext) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglLoseContext> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglLoseContext) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglLoseContext {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebglLoseContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lose_context_WEBGL_lose_context() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebglLoseContext as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebglLoseContext {
    #[cfg(all(feature = "WebglLoseContext",))]
    #[allow(bad_style)]
    #[doc = "The `loseContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/loseContext)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    #[allow(clippy::all)]
    pub fn lose_context(&self) {
        #[cfg(all(feature = "WebglLoseContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lose_context_WEBGL_lose_context(
                self_: <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lose_context_WEBGL_lose_context(
            self_: <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lose_context_WEBGL_lose_context(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebglLoseContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_restore_context_WEBGL_lose_context() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebglLoseContext as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebglLoseContext {
    #[cfg(all(feature = "WebglLoseContext",))]
    #[allow(bad_style)]
    #[doc = "The `restoreContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/restoreContext)\n\n*This API requires the following crate features to be activated: `WebglLoseContext`*"]
    #[allow(clippy::all)]
    pub fn restore_context(&self) {
        #[cfg(all(feature = "WebglLoseContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_restore_context_WEBGL_lose_context(
                self_: <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_restore_context_WEBGL_lose_context(
            self_: <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebglLoseContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_restore_context_WEBGL_lose_context(self_)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2097ab7e27090e12: [u8; 351usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1D\x01\0\0\0\0\x03\0\0\x02\x12WEBGL_lose_context$__widl_instanceof_WEBGL_lose_context\0\0\0\0(__widl_f_lose_context_WEBGL_lose_context\0\0\0\x01\x12WEBGL_lose_context\x01\0\0\x01\x01\x05self_\x0BloseContext\0\0\0+__widl_f_restore_context_WEBGL_lose_context\0\0\0\x01\x12WEBGL_lose_context\x01\0\0\x01\x01\x05self_\x0ErestoreContext\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebGLActiveInfo` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebGlActiveInfo {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebGlActiveInfo: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebGlActiveInfo {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(71u32);
            inform(76u32);
            inform(65u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(118u32);
            inform(101u32);
            inform(73u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for WebGlActiveInfo {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebGlActiveInfo {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebGlActiveInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebGlActiveInfo {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebGlActiveInfo {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebGlActiveInfo {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebGlActiveInfo {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebGlActiveInfo {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebGlActiveInfo {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebGlActiveInfo>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebGlActiveInfo {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebGlActiveInfo {
        #[inline]
        fn from(obj: JsValue) -> WebGlActiveInfo {
            WebGlActiveInfo { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebGlActiveInfo {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebGlActiveInfo> for WebGlActiveInfo {
        #[inline]
        fn as_ref(&self) -> &WebGlActiveInfo {
            self
        }
    }
    impl From<WebGlActiveInfo> for JsValue {
        #[inline]
        fn from(obj: WebGlActiveInfo) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebGlActiveInfo {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebGLActiveInfo(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebGLActiveInfo(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebGLActiveInfo(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebGlActiveInfo { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebGlActiveInfo) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebGlActiveInfo> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebGlActiveInfo) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebGlActiveInfo {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebGlActiveInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_WebGLActiveInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlActiveInfo as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlActiveInfo {
    #[cfg(all(feature = "WebGlActiveInfo",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/size)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> i32 {
        #[cfg(all(feature = "WebGlActiveInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_WebGLActiveInfo(
                self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_WebGLActiveInfo(
            self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_WebGLActiveInfo(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlActiveInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_WebGLActiveInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlActiveInfo as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl WebGlActiveInfo {
    #[cfg(all(feature = "WebGlActiveInfo",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/type)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> u32 {
        #[cfg(all(feature = "WebGlActiveInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_WebGLActiveInfo(
                self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_WebGLActiveInfo(
            self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_WebGLActiveInfo(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlActiveInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_WebGLActiveInfo() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlActiveInfo as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebGlActiveInfo {
    #[cfg(all(feature = "WebGlActiveInfo",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/name)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "WebGlActiveInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_WebGLActiveInfo(
                self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_WebGLActiveInfo(
            self_: <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlActiveInfo as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_WebGLActiveInfo(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5595c6e3dbd24b89: [u8; 381usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"};\x01\0\0\0\0\x04\0\0\x02\x0FWebGLActiveInfo!__widl_instanceof_WebGLActiveInfo\0\0\0\0\x1D__widl_f_size_WebGLActiveInfo\0\0\0\x01\x0FWebGLActiveInfo\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\x1D__widl_f_type_WebGLActiveInfo\0\0\0\x01\x0FWebGLActiveInfo\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x1D__widl_f_name_WebGLActiveInfo\0\0\0\x01\x0FWebGLActiveInfo\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

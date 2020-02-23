use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebGLShaderPrecisionFormat` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebGlShaderPrecisionFormat {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebGlShaderPrecisionFormat: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebGlShaderPrecisionFormat {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(26u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(71u32);
            inform(76u32);
            inform(83u32);
            inform(104u32);
            inform(97u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(99u32);
            inform(105u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(70u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WebGlShaderPrecisionFormat {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebGlShaderPrecisionFormat {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebGlShaderPrecisionFormat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebGlShaderPrecisionFormat {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebGlShaderPrecisionFormat {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebGlShaderPrecisionFormat {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebGlShaderPrecisionFormat {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebGlShaderPrecisionFormat {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebGlShaderPrecisionFormat {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebGlShaderPrecisionFormat>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebGlShaderPrecisionFormat {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebGlShaderPrecisionFormat {
        #[inline]
        fn from(obj: JsValue) -> WebGlShaderPrecisionFormat {
            WebGlShaderPrecisionFormat { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebGlShaderPrecisionFormat {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebGlShaderPrecisionFormat> for WebGlShaderPrecisionFormat {
        #[inline]
        fn as_ref(&self) -> &WebGlShaderPrecisionFormat {
            self
        }
    }
    impl From<WebGlShaderPrecisionFormat> for JsValue {
        #[inline]
        fn from(obj: WebGlShaderPrecisionFormat) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebGlShaderPrecisionFormat {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebGLShaderPrecisionFormat(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebGLShaderPrecisionFormat(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebGLShaderPrecisionFormat(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebGlShaderPrecisionFormat { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebGlShaderPrecisionFormat) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebGlShaderPrecisionFormat> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebGlShaderPrecisionFormat) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebGlShaderPrecisionFormat {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_min_WebGLShaderPrecisionFormat() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlShaderPrecisionFormat as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlShaderPrecisionFormat {
    #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
    #[allow(bad_style)]
    #[doc = "The `rangeMin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    #[allow(clippy::all)]
    pub fn range_min(&self) -> i32 {
        #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_min_WebGLShaderPrecisionFormat(
                self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_min_WebGLShaderPrecisionFormat(
            self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_range_min_WebGLShaderPrecisionFormat(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_max_WebGLShaderPrecisionFormat() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlShaderPrecisionFormat as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlShaderPrecisionFormat {
    #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
    #[allow(bad_style)]
    #[doc = "The `rangeMax` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    #[allow(clippy::all)]
    pub fn range_max(&self) -> i32 {
        #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_max_WebGLShaderPrecisionFormat(
                self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_max_WebGLShaderPrecisionFormat(
            self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_range_max_WebGLShaderPrecisionFormat(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_precision_WebGLShaderPrecisionFormat() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlShaderPrecisionFormat as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlShaderPrecisionFormat {
    #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
    #[allow(bad_style)]
    #[doc = "The `precision` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    #[allow(clippy::all)]
    pub fn precision(&self) -> i32 {
        #[cfg(all(feature = "WebGlShaderPrecisionFormat",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_precision_WebGLShaderPrecisionFormat(
                self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_precision_WebGLShaderPrecisionFormat(
            self_: <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlShaderPrecisionFormat as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_precision_WebGLShaderPrecisionFormat(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_2663ece02fcc3681: [u8; 510usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBC\x01\0\0\0\0\x04\0\0\x02\x1AWebGLShaderPrecisionFormat,__widl_instanceof_WebGLShaderPrecisionFormat\0\0\0\0-__widl_f_range_min_WebGLShaderPrecisionFormat\0\0\0\x01\x1AWebGLShaderPrecisionFormat\x01\0\x01\x08rangeMin\x01\x01\x05self_\x08rangeMin\0\0\0-__widl_f_range_max_WebGLShaderPrecisionFormat\0\0\0\x01\x1AWebGLShaderPrecisionFormat\x01\0\x01\x08rangeMax\x01\x01\x05self_\x08rangeMax\0\0\0-__widl_f_precision_WebGLShaderPrecisionFormat\0\0\0\x01\x1AWebGLShaderPrecisionFormat\x01\0\x01\tprecision\x01\x01\x05self_\tprecision\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

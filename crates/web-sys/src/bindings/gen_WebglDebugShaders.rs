use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_debug_shaders` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders)\n\n*This API requires the following crate features to be activated: `WebglDebugShaders`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglDebugShaders {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglDebugShaders: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglDebugShaders {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
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
            inform(115u32);
            inform(104u32);
            inform(97u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for WebglDebugShaders {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglDebugShaders {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglDebugShaders {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglDebugShaders {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglDebugShaders {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglDebugShaders {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglDebugShaders {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglDebugShaders {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglDebugShaders {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglDebugShaders>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglDebugShaders {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglDebugShaders {
        #[inline]
        fn from(obj: JsValue) -> WebglDebugShaders {
            WebglDebugShaders { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglDebugShaders {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglDebugShaders> for WebglDebugShaders {
        #[inline]
        fn as_ref(&self) -> &WebglDebugShaders {
            self
        }
    }
    impl From<WebglDebugShaders> for JsValue {
        #[inline]
        fn from(obj: WebglDebugShaders) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglDebugShaders {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_debug_shaders(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_debug_shaders(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_debug_shaders(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglDebugShaders { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglDebugShaders) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglDebugShaders> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglDebugShaders) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglDebugShaders {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebGlShader", feature = "WebglDebugShaders",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_translated_shader_source_WEBGL_debug_shaders() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebglDebugShaders as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WebglDebugShaders {
    #[cfg(all(feature = "WebGlShader", feature = "WebglDebugShaders",))]
    #[allow(bad_style)]
    #[doc = "The `getTranslatedShaderSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders/getTranslatedShaderSource)\n\n*This API requires the following crate features to be activated: `WebGlShader`, `WebglDebugShaders`*"]
    #[allow(clippy::all)]
    pub fn get_translated_shader_source(&self, shader: &WebGlShader) -> String {
        #[cfg(all(feature = "WebGlShader", feature = "WebglDebugShaders",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_translated_shader_source_WEBGL_debug_shaders(
                self_: <&WebglDebugShaders as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_translated_shader_source_WEBGL_debug_shaders(
            self_: <&WebglDebugShaders as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(shader);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebglDebugShaders as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_get_translated_shader_source_WEBGL_debug_shaders(self_, shader)
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
pub static __WASM_BINDGEN_GENERATED_f7395da9084435ba: [u8; 296usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE6\0\0\0\0\0\x02\0\0\x02\x13WEBGL_debug_shaders%__widl_instanceof_WEBGL_debug_shaders\0\0\0\09__widl_f_get_translated_shader_source_WEBGL_debug_shaders\0\0\0\x01\x13WEBGL_debug_shaders\x01\0\0\x01\x02\x05self_\x06shader\x19getTranslatedShaderSource\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

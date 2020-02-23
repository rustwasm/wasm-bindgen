use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WebGLRenderingContext` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebGlRenderingContext {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebGlRenderingContext: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebGlRenderingContext {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(87u32);
            inform(101u32);
            inform(98u32);
            inform(71u32);
            inform(76u32);
            inform(82u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WebGlRenderingContext {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebGlRenderingContext {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebGlRenderingContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebGlRenderingContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebGlRenderingContext {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebGlRenderingContext {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebGlRenderingContext {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebGlRenderingContext {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebGlRenderingContext {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebGlRenderingContext>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebGlRenderingContext {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebGlRenderingContext {
        #[inline]
        fn from(obj: JsValue) -> WebGlRenderingContext {
            WebGlRenderingContext { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebGlRenderingContext {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebGlRenderingContext> for WebGlRenderingContext {
        #[inline]
        fn as_ref(&self) -> &WebGlRenderingContext {
            self
        }
    }
    impl From<WebGlRenderingContext> for JsValue {
        #[inline]
        fn from(obj: WebGlRenderingContext) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebGlRenderingContext {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WebGLRenderingContext(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WebGLRenderingContext(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WebGLRenderingContext(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebGlRenderingContext { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebGlRenderingContext) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebGlRenderingContext> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebGlRenderingContext) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebGlRenderingContext {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_data_with_i32_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_data_with_i32(&self, target: u32, size: i32, usage: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_data_with_i32_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_data_with_i32_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(size);
            drop(usage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let size = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let usage = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(usage);
                __widl_f_buffer_data_with_i32_WebGLRenderingContext(self_, target, size, usage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_data_with_f64_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_data_with_f64(&self, target: u32, size: f64, usage: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_data_with_f64_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_data_with_f64_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(size);
            drop(usage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let usage = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(usage);
                __widl_f_buffer_data_with_f64_WebGLRenderingContext(self_, target, size, usage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&::js_sys::ArrayBuffer> as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_data_with_opt_array_buffer(
        &self,
        target: u32,
        data: Option<&::js_sys::ArrayBuffer>,
        usage: u32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&::js_sys::ArrayBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&::js_sys::ArrayBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(data);
            drop(usage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let data = < Option < & :: js_sys :: ArrayBuffer > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( data ) ;
                let usage = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(usage);
                __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext(
                    self_, target, data, usage,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_data_with_array_buffer_view(
        &self,
        target: u32,
        data: &::js_sys::Object,
        usage: u32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(data);
            drop(usage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                let usage = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(usage);
                __widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext(
                    self_, target, data, usage,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_data_with_u8_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&[u8] as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_data_with_u8_array(&self, target: u32, data: &[u8], usage: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_data_with_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_data_with_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            usage: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(data);
            drop(usage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let data = <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                let usage = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(usage);
                __widl_f_buffer_data_with_u8_array_WebGLRenderingContext(self_, target, data, usage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_i32_and_array_buffer_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_i32_and_array_buffer(
        &self,
        target: u32,
        offset: i32,
        data: &::js_sys::ArrayBuffer,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_i32_and_array_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_i32_and_array_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_i32_and_array_buffer_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_f64_and_array_buffer_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_f64_and_array_buffer(
        &self,
        target: u32,
        offset: f64,
        data: &::js_sys::ArrayBuffer,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_f64_and_array_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_f64_and_array_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_f64_and_array_buffer_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_i32_and_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_i32_and_array_buffer_view(
        &self,
        target: u32,
        offset: i32,
        data: &::js_sys::Object,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_i32_and_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_i32_and_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_i32_and_array_buffer_view_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_f64_and_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_f64_and_array_buffer_view(
        &self,
        target: u32,
        offset: f64,
        data: &::js_sys::Object,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_f64_and_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_f64_and_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_f64_and_array_buffer_view_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_i32_and_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&[u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_i32_and_u8_array(&self, target: u32, offset: i32, data: &[u8]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_i32_and_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_i32_and_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data = <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_i32_and_u8_array_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_sub_data_with_f64_and_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <&[u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSubData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn buffer_sub_data_with_f64_and_u8_array(&self, target: u32, offset: f64, data: &[u8]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_sub_data_with_f64_and_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_sub_data_with_f64_and_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(offset);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                let data = <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_buffer_sub_data_with_f64_and_u8_array_WebGLRenderingContext(
                    self_, target, offset, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_commit_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `commit()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/commit)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn commit(&self) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_commit_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_commit_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_commit_WebGLRenderingContext(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compressed_tex_image_2d_with_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `compressedTexImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn compressed_tex_image_2d_with_array_buffer_view(
        &self,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        data: &::js_sys::Object,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compressed_tex_image_2d_with_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compressed_tex_image_2d_with_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(width);
            drop(height);
            drop(border);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let border = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_compressed_tex_image_2d_with_array_buffer_view_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    border,
                    data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compressed_tex_image_2d_with_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <&[u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `compressedTexImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn compressed_tex_image_2d_with_u8_array(
        &self,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        data: &[u8],
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compressed_tex_image_2d_with_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compressed_tex_image_2d_with_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(width);
            drop(height);
            drop(border);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let border = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                let data = <&[u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_compressed_tex_image_2d_with_u8_array_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    width,
                    height,
                    border,
                    data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compressed_tex_sub_image_2d_with_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `compressedTexSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn compressed_tex_sub_image_2d_with_array_buffer_view(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        data: &::js_sys::Object,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compressed_tex_sub_image_2d_with_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compressed_tex_sub_image_2d_with_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(width);
            drop(height);
            drop(format);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_compressed_tex_sub_image_2d_with_array_buffer_view_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, width, height, format, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compressed_tex_sub_image_2d_with_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `compressedTexSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn compressed_tex_sub_image_2d_with_u8_array(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        data: &mut [u8],
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compressed_tex_sub_image_2d_with_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compressed_tex_sub_image_2d_with_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(width);
            drop(height);
            drop(format);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_compressed_tex_sub_image_2d_with_u8_array_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, width, height, format, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_pixels_with_opt_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `readPixels()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn read_pixels_with_opt_array_buffer_view(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_pixels_with_opt_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_pixels_with_opt_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        pixels,
                    );
                __widl_f_read_pixels_with_opt_array_buffer_view_WebGLRenderingContext(
                    self_, x, y, width, height, format, type_, pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_pixels_with_opt_u8_array_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&mut [u8]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `readPixels()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn read_pixels_with_opt_u8_array(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&mut [u8]>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_pixels_with_opt_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_pixels_with_opt_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_read_pixels_with_opt_u8_array_WebGLRenderingContext(
                    self_, x, y, width, height, format, type_, pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(width);
            drop(height);
            drop(border);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let border = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        pixels,
                    );
                __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view_WebGLRenderingContext ( self_ , target , level , internalformat , width , height , border , format , type_ , pixels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&[u8]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(width);
            drop(height);
            drop(border);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let border = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext ( self_ , target , level , internalformat , width , height , border , format , type_ , pixels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_u32_and_u32_and_image_bitmap(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        pixels: &ImageBitmap,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    format,
                    type_,
                    pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_u32_and_u32_and_image_data(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        pixels: &ImageData,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    format,
                    type_,
                    pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_u32_and_u32_and_image(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        image: &HtmlImageElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(format);
            drop(type_);
            drop(image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                __widl_f_tex_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    format,
                    type_,
                    image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_u32_and_u32_and_canvas(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        canvas: &HtmlCanvasElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                canvas: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            canvas: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(format);
            drop(type_);
            drop(canvas);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let canvas =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(canvas);
                __widl_f_tex_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    format,
                    type_,
                    canvas,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_image_2d_with_u32_and_u32_and_video(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        format: u32,
        type_: u32,
        video: &HtmlVideoElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                video: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            video: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(format);
            drop(type_);
            drop(video);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let video =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(video);
                __widl_f_tex_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    format,
                    type_,
                    video,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(width);
            drop(height);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        pixels,
                    );
                __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view_WebGLRenderingContext ( self_ , target , level , xoffset , yoffset , width , height , format , type_ , pixels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&[u8]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(width);
            drop(height);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels =
                    <Option<&[u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array_WebGLRenderingContext ( self_ , target , level , xoffset , yoffset , width , height , format , type_ , pixels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&ImageBitmap as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_bitmap(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        pixels: &ImageBitmap,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels = <&ImageBitmap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, format, type_, pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&ImageData as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image_data(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        pixels: &ImageData,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageData", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pixels: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pixels: <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(format);
            drop(type_);
            drop(pixels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let pixels = <&ImageData as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pixels);
                __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, format, type_, pixels,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlImageElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_image(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        image: &HtmlImageElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlImageElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            image: <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(format);
            drop(type_);
            drop(image);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let image =
                    <&HtmlImageElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(image);
                __widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, format, type_, image,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_canvas(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        canvas: &HtmlCanvasElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                canvas: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            canvas: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(format);
            drop(type_);
            drop(canvas);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let canvas =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(canvas);
                __widl_f_tex_sub_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, format, type_, canvas,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_sub_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&HtmlVideoElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)\n\n*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_sub_image_2d_with_u32_and_u32_and_video(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        format: u32,
        type_: u32,
        video: &HtmlVideoElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlVideoElement", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                video: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_sub_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            video: <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(format);
            drop(type_);
            drop(video);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let format = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let video =
                    <&HtmlVideoElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(video);
                __widl_f_tex_sub_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, format, type_, video,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1fv_with_f32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform1fv_with_f32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1fv_with_f32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform1fv_with_f32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1iv_with_i32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[i32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1iv_with_i32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1iv_with_i32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform1iv_with_i32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1iv_with_i32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1iv_with_i32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1iv_with_i32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1iv_with_i32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform1iv_with_i32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2fv_with_f32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform2fv_with_f32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2fv_with_f32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform2fv_with_f32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2iv_with_i32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[i32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2iv_with_i32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2iv_with_i32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform2iv_with_i32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2iv_with_i32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2iv_with_i32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2iv_with_i32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2iv_with_i32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform2iv_with_i32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3fv_with_f32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform3fv_with_f32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3fv_with_f32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform3fv_with_f32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3iv_with_i32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[i32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3iv_with_i32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3iv_with_i32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform3iv_with_i32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3iv_with_i32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3iv_with_i32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3iv_with_i32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3iv_with_i32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform3iv_with_i32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4fv_with_f32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform4fv_with_f32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4fv_with_f32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform4fv_with_f32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4iv_with_i32_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&[i32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4iv_with_i32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4iv_with_i32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data = <&[i32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform4iv_with_i32_array_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4iv_with_i32_sequence_WebGLRenderingContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4iv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4iv_with_i32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4iv_with_i32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4iv_with_i32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform4iv_with_i32_sequence_WebGLRenderingContext(self_, location, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix2fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix2fv_with_f32_array(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix2fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix2fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform_matrix2fv_with_f32_array_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix2fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix2fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix2fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix2fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform_matrix2fv_with_f32_sequence_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix3fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix3fv_with_f32_array(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix3fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix3fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform_matrix3fv_with_f32_array_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix3fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix3fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix3fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix3fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform_matrix3fv_with_f32_sequence_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix4fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix4fv_with_f32_array(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &[f32],
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix4fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix4fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_uniform_matrix4fv_with_f32_array_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform_matrix4fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniformMatrix4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform_matrix4fv_with_f32_sequence(
        &self,
        location: Option<&WebGlUniformLocation>,
        transpose: bool,
        data: &::wasm_bindgen::JsValue,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform_matrix4fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform_matrix4fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            transpose: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(transpose);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let transpose = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(transpose);
                let data =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_uniform_matrix4fv_with_f32_sequence_WebGLRenderingContext(
                    self_, location, transpose, data,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_texture_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `activeTexture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/activeTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn active_texture(&self, texture: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_texture_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                texture: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_texture_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            texture: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(texture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let texture = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(texture);
                __widl_f_active_texture_WebGLRenderingContext(self_, texture)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
    feature = "WebGlShader",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attach_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
        feature = "WebGlShader",
    ))]
    #[allow(bad_style)]
    #[doc = "The `attachShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/attachShader)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
        #[cfg(all(
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
            feature = "WebGlShader",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attach_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attach_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            drop(shader);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_attach_shader_WebGLRenderingContext(self_, program, shader)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_attrib_location_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bindAttribLocation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindAttribLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn bind_attrib_location(&self, program: &WebGlProgram, index: u32, name: &str) {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_attrib_location_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_attrib_location_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            drop(index);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_bind_attrib_location_WebGLRenderingContext(self_, program, index, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_buffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlBuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bindBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn bind_buffer(&self, target: u32, buffer: Option<&WebGlBuffer>) {
        #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(buffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let buffer =
                    <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                __widl_f_bind_buffer_WebGLRenderingContext(self_, target, buffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_framebuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlFramebuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bindFramebuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&WebGlFramebuffer>) {
        #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_framebuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_framebuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(framebuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let framebuffer =
                    <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        framebuffer,
                    );
                __widl_f_bind_framebuffer_WebGLRenderingContext(self_, target, framebuffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_renderbuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlRenderbuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `bindRenderbuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&WebGlRenderbuffer>) {
        #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_renderbuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                renderbuffer : < Option < & WebGlRenderbuffer > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_renderbuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            renderbuffer: <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(renderbuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let renderbuffer =
                    <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        renderbuffer,
                    );
                __widl_f_bind_renderbuffer_WebGLRenderingContext(self_, target, renderbuffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_texture_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlTexture> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
    #[allow(bad_style)]
    #[doc = "The `bindTexture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    #[allow(clippy::all)]
    pub fn bind_texture(&self, target: u32, texture: Option<&WebGlTexture>) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_texture_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_texture_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(texture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let texture =
                    <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        texture,
                    );
                __widl_f_bind_texture_WebGLRenderingContext(self_, target, texture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blend_color_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `blendColor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendColor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blend_color_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                red: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                green: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blue: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blend_color_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            red: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            green: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blue: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(red);
            drop(green);
            drop(blue);
            drop(alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let red = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(red);
                let green = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(green);
                let blue = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blue);
                let alpha = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                __widl_f_blend_color_WebGLRenderingContext(self_, red, green, blue, alpha)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blend_equation_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `blendEquation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquation)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn blend_equation(&self, mode: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blend_equation_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blend_equation_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_blend_equation_WebGLRenderingContext(self_, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blend_equation_separate_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `blendEquationSeparate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquationSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blend_equation_separate_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blend_equation_separate_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode_rgb);
            drop(mode_alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode_rgb = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode_rgb);
                let mode_alpha = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode_alpha);
                __widl_f_blend_equation_separate_WebGLRenderingContext(self_, mode_rgb, mode_alpha)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blend_func_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `blendFunc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blend_func_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sfactor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dfactor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blend_func_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sfactor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dfactor: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(sfactor);
            drop(dfactor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let sfactor = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sfactor);
                let dfactor = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dfactor);
                __widl_f_blend_func_WebGLRenderingContext(self_, sfactor, dfactor)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blend_func_separate_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `blendFuncSeparate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFuncSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blend_func_separate_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dst_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dst_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blend_func_separate_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dst_rgb: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dst_alpha: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src_rgb);
            drop(dst_rgb);
            drop(src_alpha);
            drop(dst_alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src_rgb = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src_rgb);
                let dst_rgb = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dst_rgb);
                let src_alpha = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src_alpha);
                let dst_alpha = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dst_alpha);
                __widl_f_blend_func_separate_WebGLRenderingContext(
                    self_, src_rgb, dst_rgb, src_alpha, dst_alpha,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_framebuffer_status_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `checkFramebufferStatus()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/checkFramebufferStatus)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_framebuffer_status_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_framebuffer_status_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_check_framebuffer_status_WebGLRenderingContext(self_, target)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `clear()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clear)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn clear(&self, mask: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mask);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mask = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mask);
                __widl_f_clear_WebGLRenderingContext(self_, mask)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_color_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `clearColor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearColor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_color_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                red: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                green: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blue: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_color_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            red: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            green: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blue: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(red);
            drop(green);
            drop(blue);
            drop(alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let red = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(red);
                let green = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(green);
                let blue = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blue);
                let alpha = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                __widl_f_clear_color_WebGLRenderingContext(self_, red, green, blue, alpha)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_depth_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `clearDepth()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearDepth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn clear_depth(&self, depth: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_depth_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                depth: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_depth_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            depth: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(depth);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let depth = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(depth);
                __widl_f_clear_depth_WebGLRenderingContext(self_, depth)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_stencil_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `clearStencil()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearStencil)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn clear_stencil(&self, s: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_stencil_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                s: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_stencil_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            s: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(s);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let s = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
                __widl_f_clear_stencil_WebGLRenderingContext(self_, s)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_color_mask_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `colorMask()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/colorMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_color_mask_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                red: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                green: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                blue: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_color_mask_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            red: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            green: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            blue: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(red);
            drop(green);
            drop(blue);
            drop(alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let red = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(red);
                let green = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(green);
                let blue = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blue);
                let alpha = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                __widl_f_color_mask_WebGLRenderingContext(self_, red, green, blue, alpha)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_compile_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `compileShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compileShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn compile_shader(&self, shader: &WebGlShader) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_compile_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_compile_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_compile_shader_WebGLRenderingContext(self_, shader)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_tex_image_2d_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `copyTexImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn copy_tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internalformat: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        border: i32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_tex_image_2d_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_tex_image_2d_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(internalformat);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            drop(border);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let internalformat =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                let border = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                __widl_f_copy_tex_image_2d_WebGLRenderingContext(
                    self_,
                    target,
                    level,
                    internalformat,
                    x,
                    y,
                    width,
                    height,
                    border,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_tex_sub_image_2d_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `copyTexSubImage2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexSubImage2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn copy_tex_sub_image_2d(
        &self,
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_tex_sub_image_2d_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_tex_sub_image_2d_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            xoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            yoffset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(level);
            drop(xoffset);
            drop(yoffset);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                let xoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(xoffset);
                let yoffset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(yoffset);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_copy_tex_sub_image_2d_WebGLRenderingContext(
                    self_, target, level, xoffset, yoffset, x, y, width, height,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_buffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlBuffer> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn create_buffer(&self) -> Option<WebGlBuffer> {
        #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_buffer_WebGLRenderingContext(self_)
            };
            <Option<WebGlBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_framebuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlFramebuffer> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `createFramebuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn create_framebuffer(&self) -> Option<WebGlFramebuffer> {
        #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_framebuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlFramebuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_framebuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlFramebuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_framebuffer_WebGLRenderingContext(self_)
            };
            <Option<WebGlFramebuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlProgram> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `createProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn create_program(&self) -> Option<WebGlProgram> {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlProgram> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlProgram> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_program_WebGLRenderingContext(self_)
            };
            <Option<WebGlProgram> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_renderbuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlRenderbuffer> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `createRenderbuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn create_renderbuffer(&self) -> Option<WebGlRenderbuffer> {
        #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_renderbuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlRenderbuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_renderbuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlRenderbuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_renderbuffer_WebGLRenderingContext(self_)
            };
            <Option<WebGlRenderbuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<WebGlShader> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `createShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn create_shader(&self, type_: u32) -> Option<WebGlShader> {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlShader> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlShader> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_create_shader_WebGLRenderingContext(self_, type_)
            };
            <Option<WebGlShader> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_texture_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlTexture> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
    #[allow(bad_style)]
    #[doc = "The `createTexture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    #[allow(clippy::all)]
    pub fn create_texture(&self) -> Option<WebGlTexture> {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_texture_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlTexture> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_texture_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlTexture> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_texture_WebGLRenderingContext(self_)
            };
            <Option<WebGlTexture> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cull_face_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `cullFace()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/cullFace)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn cull_face(&self, mode: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cull_face_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cull_face_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_cull_face_WebGLRenderingContext(self_, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_buffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlBuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `deleteBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn delete_buffer(&self, buffer: Option<&WebGlBuffer>) {
        #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(buffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer =
                    <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                __widl_f_delete_buffer_WebGLRenderingContext(self_, buffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_framebuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlFramebuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `deleteFramebuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn delete_framebuffer(&self, framebuffer: Option<&WebGlFramebuffer>) {
        #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_framebuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_framebuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(framebuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let framebuffer =
                    <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        framebuffer,
                    );
                __widl_f_delete_framebuffer_WebGLRenderingContext(self_, framebuffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlProgram> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `deleteProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn delete_program(&self, program: Option<&WebGlProgram>) {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        program,
                    );
                __widl_f_delete_program_WebGLRenderingContext(self_, program)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_renderbuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlRenderbuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `deleteRenderbuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGlRenderbuffer>) {
        #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_renderbuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                renderbuffer : < Option < & WebGlRenderbuffer > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_renderbuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            renderbuffer: <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(renderbuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let renderbuffer =
                    <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        renderbuffer,
                    );
                __widl_f_delete_renderbuffer_WebGLRenderingContext(self_, renderbuffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlShader> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `deleteShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn delete_shader(&self, shader: Option<&WebGlShader>) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader =
                    <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_delete_shader_WebGLRenderingContext(self_, shader)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_texture_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlTexture> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
    #[allow(bad_style)]
    #[doc = "The `deleteTexture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    #[allow(clippy::all)]
    pub fn delete_texture(&self, texture: Option<&WebGlTexture>) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_texture_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_texture_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(texture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let texture =
                    <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        texture,
                    );
                __widl_f_delete_texture_WebGLRenderingContext(self_, texture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_depth_func_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `depthFunc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn depth_func(&self, func: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_depth_func_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_depth_func_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(func);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let func = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(func);
                __widl_f_depth_func_WebGLRenderingContext(self_, func)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_depth_mask_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `depthMask()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn depth_mask(&self, flag: bool) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_depth_mask_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_depth_mask_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            flag: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(flag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let flag = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(flag);
                __widl_f_depth_mask_WebGLRenderingContext(self_, flag)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_depth_range_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `depthRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthRange)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn depth_range(&self, z_near: f32, z_far: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_depth_range_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z_near: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z_far: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_depth_range_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z_near: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z_far: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(z_near);
            drop(z_far);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let z_near = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z_near);
                let z_far = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z_far);
                __widl_f_depth_range_WebGLRenderingContext(self_, z_near, z_far)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
    feature = "WebGlShader",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detach_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
        feature = "WebGlShader",
    ))]
    #[allow(bad_style)]
    #[doc = "The `detachShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/detachShader)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) {
        #[cfg(all(
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
            feature = "WebGlShader",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detach_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detach_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            drop(shader);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_detach_shader_WebGLRenderingContext(self_, program, shader)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disable_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `disable()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disable)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn disable(&self, cap: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disable_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disable_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cap = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cap);
                __widl_f_disable_WebGLRenderingContext(self_, cap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disable_vertex_attrib_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `disableVertexAttribArray()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disableVertexAttribArray)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn disable_vertex_attrib_array(&self, index: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disable_vertex_attrib_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disable_vertex_attrib_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_disable_vertex_attrib_array_WebGLRenderingContext(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_arrays_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `drawArrays()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_arrays_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                first: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_arrays_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            first: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(first);
            drop(count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let first = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(first);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                __widl_f_draw_arrays_WebGLRenderingContext(self_, mode, first, count)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_elements_with_i32_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `drawElements()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn draw_elements_with_i32(&self, mode: u32, count: i32, type_: u32, offset: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_elements_with_i32_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_elements_with_i32_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(count);
            drop(type_);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_draw_elements_with_i32_WebGLRenderingContext(
                    self_, mode, count, type_, offset,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_elements_with_f64_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `drawElements()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn draw_elements_with_f64(&self, mode: u32, count: i32, type_: u32, offset: f64) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_elements_with_f64_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_elements_with_f64_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            drop(count);
            drop(type_);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                let count = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_draw_elements_with_f64_WebGLRenderingContext(
                    self_, mode, count, type_, offset,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enable_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `enable()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn enable(&self, cap: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enable_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enable_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cap = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cap);
                __widl_f_enable_WebGLRenderingContext(self_, cap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enable_vertex_attrib_array_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `enableVertexAttribArray()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enableVertexAttribArray)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn enable_vertex_attrib_array(&self, index: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enable_vertex_attrib_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enable_vertex_attrib_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_enable_vertex_attrib_array_WebGLRenderingContext(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_finish_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `finish()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/finish)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn finish(&self) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_finish_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_finish_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_finish_WebGLRenderingContext(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flush_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `flush()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/flush)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn flush(&self) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flush_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flush_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flush_WebGLRenderingContext(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_framebuffer_renderbuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlRenderbuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `framebufferRenderbuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffertarget: u32,
        renderbuffer: Option<&WebGlRenderbuffer>,
    ) {
        #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_framebuffer_renderbuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                renderbuffertarget: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                renderbuffer : < Option < & WebGlRenderbuffer > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_framebuffer_renderbuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            renderbuffertarget: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            renderbuffer: <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(attachment);
            drop(renderbuffertarget);
            drop(renderbuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let attachment = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attachment);
                let renderbuffertarget =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(renderbuffertarget);
                let renderbuffer =
                    <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        renderbuffer,
                    );
                __widl_f_framebuffer_renderbuffer_WebGLRenderingContext(
                    self_,
                    target,
                    attachment,
                    renderbuffertarget,
                    renderbuffer,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_framebuffer_texture_2d_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&WebGlTexture> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
    #[allow(bad_style)]
    #[doc = "The `framebufferTexture2D()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferTexture2D)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    #[allow(clippy::all)]
    pub fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&WebGlTexture>,
        level: i32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_framebuffer_texture_2d_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                textarget: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_framebuffer_texture_2d_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            textarget: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            level: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(attachment);
            drop(textarget);
            drop(texture);
            drop(level);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let attachment = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attachment);
                let textarget = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(textarget);
                let texture =
                    <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        texture,
                    );
                let level = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(level);
                __widl_f_framebuffer_texture_2d_WebGLRenderingContext(
                    self_, target, attachment, textarget, texture, level,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_front_face_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `frontFace()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/frontFace)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn front_face(&self, mode: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_front_face_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_front_face_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_front_face_WebGLRenderingContext(self_, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_mipmap_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `generateMipmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/generateMipmap)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn generate_mipmap(&self, target: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_mipmap_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_mipmap_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_generate_mipmap_WebGLRenderingContext(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "WebGlActiveInfo",
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_active_attrib_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<WebGlActiveInfo> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlActiveInfo",
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getActiveAttrib()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveAttrib)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_active_attrib(&self, program: &WebGlProgram, index: u32) -> Option<WebGlActiveInfo> {
        #[cfg(all(
            feature = "WebGlActiveInfo",
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_active_attrib_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_active_attrib_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_active_attrib_WebGLRenderingContext(self_, program, index)
            };
            <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "WebGlActiveInfo",
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_active_uniform_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<WebGlActiveInfo> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlActiveInfo",
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getActiveUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveUniform)\n\n*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_active_uniform(
        &self,
        program: &WebGlProgram,
        index: u32,
    ) -> Option<WebGlActiveInfo> {
        #[cfg(all(
            feature = "WebGlActiveInfo",
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_active_uniform_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_active_uniform_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_active_uniform_WebGLRenderingContext(self_, program, index)
            };
            <Option<WebGlActiveInfo> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attached_shaders_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <Option<::js_sys::Array> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getAttachedShaders()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttachedShaders)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_attached_shaders(&self, program: &WebGlProgram) -> Option<::js_sys::Array> {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attached_shaders_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attached_shaders_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                __widl_f_get_attached_shaders_WebGLRenderingContext(self_, program)
            };
            <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_attrib_location_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getAttribLocation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_attrib_location(&self, program: &WebGlProgram, name: &str) -> i32 {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_attrib_location_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_attrib_location_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_attrib_location_WebGLRenderingContext(self_, program, name)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_buffer_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getBufferParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getBufferParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_buffer_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_buffer_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_buffer_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_buffer_parameter_WebGLRenderingContext(self_, target, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlContextAttributes", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_context_attributes_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<WebGlContextAttributes> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlContextAttributes", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getContextAttributes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getContextAttributes)\n\n*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_context_attributes(&self) -> Option<WebGlContextAttributes> {
        #[cfg(all(feature = "WebGlContextAttributes", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_context_attributes_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlContextAttributes> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_context_attributes_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlContextAttributes> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_context_attributes_WebGLRenderingContext(self_)
            };
            <Option<WebGlContextAttributes> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_error_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getError()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_error(&self) -> u32 {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_error_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_error_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_error_WebGLRenderingContext(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_extension_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getExtension()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getExtension)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_extension(
        &self,
        name: &str,
    ) -> Result<Option<::js_sys::Object>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_extension_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_extension_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_extension_WebGLRenderingContext(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_framebuffer_attachment_parameter_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getFramebufferAttachmentParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getFramebufferAttachmentParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_framebuffer_attachment_parameter(
        &self,
        target: u32,
        attachment: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_framebuffer_attachment_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_framebuffer_attachment_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            attachment: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(attachment);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let attachment = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(attachment);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_framebuffer_attachment_parameter_WebGLRenderingContext(
                    self_, target, attachment, pname,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_parameter(
        &self,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_parameter_WebGLRenderingContext(self_, pname)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_program_info_log_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getProgramInfoLog()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramInfoLog)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_program_info_log(&self, program: &WebGlProgram) -> Option<String> {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_program_info_log_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_program_info_log_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                __widl_f_get_program_info_log_WebGLRenderingContext(self_, program)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_program_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getProgramParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_program_parameter(
        &self,
        program: &WebGlProgram,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_program_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_program_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_program_parameter_WebGLRenderingContext(self_, program, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_renderbuffer_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getRenderbufferParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getRenderbufferParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_renderbuffer_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_renderbuffer_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_renderbuffer_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_renderbuffer_parameter_WebGLRenderingContext(self_, target, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_shader_info_log_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `getShaderInfoLog()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderInfoLog)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn get_shader_info_log(&self, shader: &WebGlShader) -> Option<String> {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_shader_info_log_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_shader_info_log_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_get_shader_info_log_WebGLRenderingContext(self_, shader)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_shader_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `getShaderParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn get_shader_parameter(
        &self,
        shader: &WebGlShader,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_shader_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_shader_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(shader);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_shader_parameter_WebGLRenderingContext(self_, shader, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "WebGlRenderingContext",
    feature = "WebGlShaderPrecisionFormat",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_shader_precision_format_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<WebGlShaderPrecisionFormat> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlRenderingContext",
        feature = "WebGlShaderPrecisionFormat",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getShaderPrecisionFormat()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderPrecisionFormat)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShaderPrecisionFormat`*"]
    #[allow(clippy::all)]
    pub fn get_shader_precision_format(
        &self,
        shadertype: u32,
        precisiontype: u32,
    ) -> Option<WebGlShaderPrecisionFormat> {
        #[cfg(all(
            feature = "WebGlRenderingContext",
            feature = "WebGlShaderPrecisionFormat",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_shader_precision_format_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shadertype: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                precisiontype: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlShaderPrecisionFormat> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_shader_precision_format_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shadertype: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            precisiontype: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlShaderPrecisionFormat> as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            drop(self_);
            drop(shadertype);
            drop(precisiontype);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shadertype = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shadertype);
                let precisiontype =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(precisiontype);
                __widl_f_get_shader_precision_format_WebGLRenderingContext(
                    self_,
                    shadertype,
                    precisiontype,
                )
            };
            <Option<WebGlShaderPrecisionFormat> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            )
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_shader_source_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `getShaderSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderSource)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn get_shader_source(&self, shader: &WebGlShader) -> Option<String> {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_shader_source_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_shader_source_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_get_shader_source_WebGLRenderingContext(self_, shader)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_supported_extensions_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<::js_sys::Array> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getSupportedExtensions()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getSupportedExtensions)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_supported_extensions(&self) -> Option<::js_sys::Array> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_supported_extensions_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_supported_extensions_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_supported_extensions_WebGLRenderingContext(self_)
            };
            <Option<::js_sys::Array> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_tex_parameter_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getTexParameter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getTexParameter)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_tex_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_tex_parameter_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_tex_parameter_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_tex_parameter_WebGLRenderingContext(self_, target, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
    feature = "WebGlUniformLocation",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_uniform_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <&WebGlUniformLocation as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
        feature = "WebGlUniformLocation",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getUniform()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniform)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn get_uniform(
        &self,
        program: &WebGlProgram,
        location: &WebGlUniformLocation,
    ) -> ::wasm_bindgen::JsValue {
        #[cfg(all(
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
            feature = "WebGlUniformLocation",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_uniform_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location: <&WebGlUniformLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_uniform_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <&WebGlUniformLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(location);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let location =
                    <&WebGlUniformLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                __widl_f_get_uniform_WebGLRenderingContext(self_, program, location)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "WebGlProgram",
    feature = "WebGlRenderingContext",
    feature = "WebGlUniformLocation",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_uniform_location_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<WebGlUniformLocation> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(
        feature = "WebGlProgram",
        feature = "WebGlRenderingContext",
        feature = "WebGlUniformLocation",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getUniformLocation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        #[cfg(all(
            feature = "WebGlProgram",
            feature = "WebGlRenderingContext",
            feature = "WebGlUniformLocation",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_uniform_location_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlUniformLocation> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_uniform_location_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlUniformLocation> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_uniform_location_WebGLRenderingContext(self_, program, name)
            };
            <Option<WebGlUniformLocation> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_vertex_attrib_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getVertexAttrib()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttrib)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_vertex_attrib(
        &self,
        index: u32,
        pname: u32,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_vertex_attrib_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_vertex_attrib_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_vertex_attrib_WebGLRenderingContext(self_, index, pname)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_vertex_attrib_offset_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `getVertexAttribOffset()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttribOffset)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn get_vertex_attrib_offset(&self, index: u32, pname: u32) -> f64 {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_vertex_attrib_offset_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_vertex_attrib_offset_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_vertex_attrib_offset_WebGLRenderingContext(self_, index, pname)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hint_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `hint()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/hint)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn hint(&self, target: u32, mode: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hint_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hint_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let mode = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_hint_WebGLRenderingContext(self_, target, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_buffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlBuffer> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isBuffer)\n\n*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_buffer(&self, buffer: Option<&WebGlBuffer>) -> bool {
        #[cfg(all(feature = "WebGlBuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_buffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_buffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer: <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer =
                    <Option<&WebGlBuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                __widl_f_is_buffer_WebGLRenderingContext(self_, buffer)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_context_lost_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isContextLost()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isContextLost)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_context_lost(&self) -> bool {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_context_lost_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_context_lost_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_context_lost_WebGLRenderingContext(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_enabled_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isEnabled)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_enabled(&self, cap: u32) -> bool {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_enabled_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_enabled_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cap: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(cap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cap = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cap);
                __widl_f_is_enabled_WebGLRenderingContext(self_, cap)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_framebuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlFramebuffer> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isFramebuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isFramebuffer)\n\n*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_framebuffer(&self, framebuffer: Option<&WebGlFramebuffer>) -> bool {
        #[cfg(all(feature = "WebGlFramebuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_framebuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_framebuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            framebuffer: <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(framebuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let framebuffer =
                    <Option<&WebGlFramebuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        framebuffer,
                    );
                __widl_f_is_framebuffer_WebGLRenderingContext(self_, framebuffer)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlProgram> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_program(&self, program: Option<&WebGlProgram>) -> bool {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        program,
                    );
                __widl_f_is_program_WebGLRenderingContext(self_, program)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_renderbuffer_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlRenderbuffer> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `isRenderbuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isRenderbuffer)\n\n*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGlRenderbuffer>) -> bool {
        #[cfg(all(feature = "WebGlRenderbuffer", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_renderbuffer_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                renderbuffer : < Option < & WebGlRenderbuffer > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_renderbuffer_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            renderbuffer: <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(renderbuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let renderbuffer =
                    <Option<&WebGlRenderbuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        renderbuffer,
                    );
                __widl_f_is_renderbuffer_WebGLRenderingContext(self_, renderbuffer)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_shader_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlShader> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `isShader()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isShader)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn is_shader(&self, shader: Option<&WebGlShader>) -> bool {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_shader_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_shader_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader =
                    <Option<&WebGlShader> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                __widl_f_is_shader_WebGLRenderingContext(self_, shader)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_texture_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlTexture> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
    #[allow(bad_style)]
    #[doc = "The `isTexture()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isTexture)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
    #[allow(clippy::all)]
    pub fn is_texture(&self, texture: Option<&WebGlTexture>) -> bool {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlTexture",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_texture_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_texture_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            texture: <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(texture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let texture =
                    <Option<&WebGlTexture> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        texture,
                    );
                __widl_f_is_texture_WebGLRenderingContext(self_, texture)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_line_width_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `lineWidth()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/lineWidth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn line_width(&self, width: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_line_width_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_line_width_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_line_width_WebGLRenderingContext(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_link_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `linkProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/linkProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn link_program(&self, program: &WebGlProgram) {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_link_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_link_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                __widl_f_link_program_WebGLRenderingContext(self_, program)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pixel_storei_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `pixelStorei()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/pixelStorei)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn pixel_storei(&self, pname: u32, param: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pixel_storei_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                param: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pixel_storei_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            param: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pname);
            drop(param);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                let param = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(param);
                __widl_f_pixel_storei_WebGLRenderingContext(self_, pname, param)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_polygon_offset_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `polygonOffset()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/polygonOffset)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn polygon_offset(&self, factor: f32, units: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_polygon_offset_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                factor: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_polygon_offset_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            factor: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            units: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(factor);
            drop(units);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let factor = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(factor);
                let units = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(units);
                __widl_f_polygon_offset_WebGLRenderingContext(self_, factor, units)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_renderbuffer_storage_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `renderbufferStorage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/renderbufferStorage)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn renderbuffer_storage(&self, target: u32, internalformat: u32, width: i32, height: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_renderbuffer_storage_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_renderbuffer_storage_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            internalformat: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(internalformat);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let internalformat =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(internalformat);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_renderbuffer_storage_WebGLRenderingContext(
                    self_,
                    target,
                    internalformat,
                    width,
                    height,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_coverage_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `sampleCoverage()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/sampleCoverage)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn sample_coverage(&self, value: f32, invert: bool) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_coverage_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                invert: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_coverage_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            invert: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            drop(invert);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let invert = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(invert);
                __widl_f_sample_coverage_WebGLRenderingContext(self_, value, invert)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scissor_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `scissor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/scissor)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scissor_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scissor_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_scissor_WebGLRenderingContext(self_, x, y, width, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shader_source_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlShader as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
    #[allow(bad_style)]
    #[doc = "The `shaderSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/shaderSource)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
    #[allow(clippy::all)]
    pub fn shader_source(&self, shader: &WebGlShader, source: &str) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlShader",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shader_source_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shader_source_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shader: <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shader);
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shader = <&WebGlShader as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shader);
                let source = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_shader_source_WebGLRenderingContext(self_, shader, source)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_func_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilFunc()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFunc)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_func_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_func_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(func);
            drop(ref_);
            drop(mask);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let func = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(func);
                let ref_ = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_);
                let mask = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mask);
                __widl_f_stencil_func_WebGLRenderingContext(self_, func, ref_, mask)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_func_separate_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilFuncSeparate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFuncSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_func_separate_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_func_separate_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            func: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(face);
            drop(func);
            drop(ref_);
            drop(mask);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let face = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(face);
                let func = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(func);
                let ref_ = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_);
                let mask = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mask);
                __widl_f_stencil_func_separate_WebGLRenderingContext(self_, face, func, ref_, mask)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_mask_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilMask()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMask)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_mask(&self, mask: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_mask_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_mask_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mask);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mask = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mask);
                __widl_f_stencil_mask_WebGLRenderingContext(self_, mask)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_mask_separate_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilMaskSeparate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMaskSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_mask_separate(&self, face: u32, mask: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_mask_separate_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_mask_separate_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mask: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(face);
            drop(mask);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let face = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(face);
                let mask = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mask);
                __widl_f_stencil_mask_separate_WebGLRenderingContext(self_, face, mask)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_op_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilOp()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOp)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_op_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zfail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zpass: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_op_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zfail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zpass: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fail);
            drop(zfail);
            drop(zpass);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fail = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fail);
                let zfail = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zfail);
                let zpass = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zpass);
                __widl_f_stencil_op_WebGLRenderingContext(self_, fail, zfail, zpass)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stencil_op_separate_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `stencilOpSeparate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOpSeparate)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn stencil_op_separate(&self, face: u32, fail: u32, zfail: u32, zpass: u32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stencil_op_separate_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zfail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                zpass: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stencil_op_separate_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            face: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zfail: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            zpass: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(face);
            drop(fail);
            drop(zfail);
            drop(zpass);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let face = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(face);
                let fail = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fail);
                let zfail = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zfail);
                let zpass = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(zpass);
                __widl_f_stencil_op_separate_WebGLRenderingContext(self_, face, fail, zfail, zpass)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_parameterf_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texParameterf()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameterf)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_parameterf(&self, target: u32, pname: u32, param: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_parameterf_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                param: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_parameterf_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            param: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(pname);
            drop(param);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                let param = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(param);
                __widl_f_tex_parameterf_WebGLRenderingContext(self_, target, pname, param)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tex_parameteri_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `texParameteri()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameteri)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tex_parameteri_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                param: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tex_parameteri_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            param: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(pname);
            drop(param);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                let param = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(param);
                __widl_f_tex_parameteri_WebGLRenderingContext(self_, target, pname, param)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1f(&self, location: Option<&WebGlUniformLocation>, x: f32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_uniform1f_WebGLRenderingContext(self_, location, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform1i_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform1i()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform1i(&self, location: Option<&WebGlUniformLocation>, x: i32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform1i_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform1i_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_uniform1i_WebGLRenderingContext(self_, location, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2f(&self, location: Option<&WebGlUniformLocation>, x: f32, y: f32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_uniform2f_WebGLRenderingContext(self_, location, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform2i_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform2i()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform2i(&self, location: Option<&WebGlUniformLocation>, x: i32, y: i32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform2i_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform2i_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_uniform2i_WebGLRenderingContext(self_, location, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3f(&self, location: Option<&WebGlUniformLocation>, x: f32, y: f32, z: f32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_uniform3f_WebGLRenderingContext(self_, location, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform3i_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform3i()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform3i(&self, location: Option<&WebGlUniformLocation>, x: i32, y: i32, z: i32) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform3i_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform3i_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_uniform3i_WebGLRenderingContext(self_, location, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4f(
        &self,
        location: Option<&WebGlUniformLocation>,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            drop(z);
            drop(w);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let w = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                __widl_f_uniform4f_WebGLRenderingContext(self_, location, x, y, z, w)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_uniform4i_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlUniformLocation> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
    #[allow(bad_style)]
    #[doc = "The `uniform4i()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4i)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
    #[allow(clippy::all)]
    pub fn uniform4i(
        &self,
        location: Option<&WebGlUniformLocation>,
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext", feature = "WebGlUniformLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_uniform4i_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location : < Option < & WebGlUniformLocation > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_uniform4i_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            drop(x);
            drop(y);
            drop(z);
            drop(w);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<&WebGlUniformLocation> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        location,
                    );
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let w = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                __widl_f_uniform4i_WebGLRenderingContext(self_, location, x, y, z, w)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_use_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<&WebGlProgram> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `useProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/useProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn use_program(&self, program: Option<&WebGlProgram>) {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_use_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_use_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <Option<&WebGlProgram> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        program,
                    );
                __widl_f_use_program_WebGLRenderingContext(self_, program)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validate_program_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <&WebGlProgram as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `validateProgram()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/validateProgram)\n\n*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn validate_program(&self, program: &WebGlProgram) {
        #[cfg(all(feature = "WebGlProgram", feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validate_program_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validate_program_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            program: <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(program);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let program =
                    <&WebGlProgram as wasm_bindgen::convert::IntoWasmAbi>::into_abi(program);
                __widl_f_validate_program_WebGLRenderingContext(self_, program)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib1f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib1f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib1f(&self, indx: u32, x: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib1f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib1f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_vertex_attrib1f_WebGLRenderingContext(self_, indx, x)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib1fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib1fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib1fv_with_f32_array(&self, indx: u32, values: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib1fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib1fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(values);
                __widl_f_vertex_attrib1fv_with_f32_array_WebGLRenderingContext(self_, indx, values)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib1fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib1fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib1fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib1fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib1fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        values,
                    );
                __widl_f_vertex_attrib1fv_with_f32_sequence_WebGLRenderingContext(
                    self_, indx, values,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib2f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib2f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib2f(&self, indx: u32, x: f32, y: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib2f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib2f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_vertex_attrib2f_WebGLRenderingContext(self_, indx, x, y)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib2fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib2fv_with_f32_array(&self, indx: u32, values: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib2fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib2fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(values);
                __widl_f_vertex_attrib2fv_with_f32_array_WebGLRenderingContext(self_, indx, values)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib2fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib2fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib2fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib2fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib2fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        values,
                    );
                __widl_f_vertex_attrib2fv_with_f32_sequence_WebGLRenderingContext(
                    self_, indx, values,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib3f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib3f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib3f(&self, indx: u32, x: f32, y: f32, z: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib3f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib3f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_vertex_attrib3f_WebGLRenderingContext(self_, indx, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib3fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib3fv_with_f32_array(&self, indx: u32, values: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib3fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib3fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(values);
                __widl_f_vertex_attrib3fv_with_f32_array_WebGLRenderingContext(self_, indx, values)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib3fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib3fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib3fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib3fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib3fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        values,
                    );
                __widl_f_vertex_attrib3fv_with_f32_sequence_WebGLRenderingContext(
                    self_, indx, values,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib4f_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib4f()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4f)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib4f(&self, indx: u32, x: f32, y: f32, z: f32, w: f32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib4f_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                w: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib4f_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            w: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(x);
            drop(y);
            drop(z);
            drop(w);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let w = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(w);
                __widl_f_vertex_attrib4f_WebGLRenderingContext(self_, indx, x, y, z, w)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib4fv_with_f32_array_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&[f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib4fv_with_f32_array(&self, indx: u32, values: &[f32]) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib4fv_with_f32_array_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib4fv_with_f32_array_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values = <&[f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(values);
                __widl_f_vertex_attrib4fv_with_f32_array_WebGLRenderingContext(self_, indx, values)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib4fv_with_f32_sequence_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttrib4fv()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib4fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib4fv_with_f32_sequence_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib4fv_with_f32_sequence_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(values);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let values =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        values,
                    );
                __widl_f_vertex_attrib4fv_with_f32_sequence_WebGLRenderingContext(
                    self_, indx, values,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttribPointer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib_pointer_with_i32(
        &self,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                normalized: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stride: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            normalized: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stride: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(size);
            drop(type_);
            drop(normalized);
            drop(stride);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let size = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let normalized = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(normalized);
                let stride = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stride);
                let offset = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext(
                    self_, indx, size, type_, normalized, stride, offset,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vertex_attrib_pointer_with_f64_WebGLRenderingContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `vertexAttribPointer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn vertex_attrib_pointer_with_f64(
        &self,
        indx: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: f64,
    ) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vertex_attrib_pointer_with_f64_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                normalized: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                stride: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vertex_attrib_pointer_with_f64_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indx: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            normalized: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            stride: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indx);
            drop(size);
            drop(type_);
            drop(normalized);
            drop(stride);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indx = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indx);
                let size = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let type_ = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let normalized = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(normalized);
                let stride = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stride);
                let offset = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_vertex_attrib_pointer_with_f64_WebGLRenderingContext(
                    self_, indx, size, type_, normalized, stride, offset,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_viewport_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `viewport()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/viewport)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_viewport_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_viewport_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_viewport_WebGLRenderingContext(self_, x, y, width, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_canvas_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `canvas` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/canvas)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn canvas(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_canvas_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_canvas_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_canvas_WebGLRenderingContext(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_drawing_buffer_width_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `drawingBufferWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferWidth)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn drawing_buffer_width(&self) -> i32 {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_drawing_buffer_width_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_drawing_buffer_width_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_drawing_buffer_width_WebGLRenderingContext(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WebGlRenderingContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_drawing_buffer_height_WebGLRenderingContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WebGlRenderingContext as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl WebGlRenderingContext {
    #[cfg(all(feature = "WebGlRenderingContext",))]
    #[allow(bad_style)]
    #[doc = "The `drawingBufferHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferHeight)\n\n*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[allow(clippy::all)]
    pub fn drawing_buffer_height(&self) -> i32 {
        #[cfg(all(feature = "WebGlRenderingContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_drawing_buffer_height_WebGLRenderingContext(
                self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_drawing_buffer_height_WebGLRenderingContext(
            self_: <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&WebGlRenderingContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_drawing_buffer_height_WebGLRenderingContext(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl WebGlRenderingContext {
    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;
}
impl WebGlRenderingContext {
    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;
}
impl WebGlRenderingContext {
    pub const POINTS: u32 = 0u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINES: u32 = 1u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINE_LOOP: u32 = 2u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINE_STRIP: u32 = 3u64 as u32;
}
impl WebGlRenderingContext {
    pub const TRIANGLES: u32 = 4u64 as u32;
}
impl WebGlRenderingContext {
    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;
}
impl WebGlRenderingContext {
    pub const TRIANGLE_FAN: u32 = 6u64 as u32;
}
impl WebGlRenderingContext {
    pub const ZERO: u32 = 0i64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE: u32 = 1u64 as u32;
}
impl WebGlRenderingContext {
    pub const SRC_COLOR: u32 = 768u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;
}
impl WebGlRenderingContext {
    pub const SRC_ALPHA: u32 = 770u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;
}
impl WebGlRenderingContext {
    pub const DST_ALPHA: u32 = 772u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;
}
impl WebGlRenderingContext {
    pub const DST_COLOR: u32 = 774u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;
}
impl WebGlRenderingContext {
    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;
}
impl WebGlRenderingContext {
    pub const FUNC_ADD: u32 = 32774u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_EQUATION: u32 = 32777u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;
}
impl WebGlRenderingContext {
    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;
}
impl WebGlRenderingContext {
    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;
}
impl WebGlRenderingContext {
    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;
}
impl WebGlRenderingContext {
    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;
}
impl WebGlRenderingContext {
    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND_COLOR: u32 = 32773u64 as u32;
}
impl WebGlRenderingContext {
    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;
}
impl WebGlRenderingContext {
    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;
}
impl WebGlRenderingContext {
    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;
}
impl WebGlRenderingContext {
    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;
}
impl WebGlRenderingContext {
    pub const STREAM_DRAW: u32 = 35040u64 as u32;
}
impl WebGlRenderingContext {
    pub const STATIC_DRAW: u32 = 35044u64 as u32;
}
impl WebGlRenderingContext {
    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;
}
impl WebGlRenderingContext {
    pub const BUFFER_SIZE: u32 = 34660u64 as u32;
}
impl WebGlRenderingContext {
    pub const BUFFER_USAGE: u32 = 34661u64 as u32;
}
impl WebGlRenderingContext {
    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRONT: u32 = 1028u64 as u32;
}
impl WebGlRenderingContext {
    pub const BACK: u32 = 1029u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;
}
impl WebGlRenderingContext {
    pub const CULL_FACE: u32 = 2884u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLEND: u32 = 3042u64 as u32;
}
impl WebGlRenderingContext {
    pub const DITHER: u32 = 3024u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_TEST: u32 = 2960u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_TEST: u32 = 2929u64 as u32;
}
impl WebGlRenderingContext {
    pub const SCISSOR_TEST: u32 = 3089u64 as u32;
}
impl WebGlRenderingContext {
    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;
}
impl WebGlRenderingContext {
    pub const NO_ERROR: u32 = 0i64 as u32;
}
impl WebGlRenderingContext {
    pub const INVALID_ENUM: u32 = 1280u64 as u32;
}
impl WebGlRenderingContext {
    pub const INVALID_VALUE: u32 = 1281u64 as u32;
}
impl WebGlRenderingContext {
    pub const INVALID_OPERATION: u32 = 1282u64 as u32;
}
impl WebGlRenderingContext {
    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;
}
impl WebGlRenderingContext {
    pub const CW: u32 = 2304u64 as u32;
}
impl WebGlRenderingContext {
    pub const CCW: u32 = 2305u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINE_WIDTH: u32 = 2849u64 as u32;
}
impl WebGlRenderingContext {
    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;
}
impl WebGlRenderingContext {
    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;
}
impl WebGlRenderingContext {
    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRONT_FACE: u32 = 2886u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_RANGE: u32 = 2928u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_FUNC: u32 = 2932u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_FUNC: u32 = 2962u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_FAIL: u32 = 2964u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_REF: u32 = 2967u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;
}
impl WebGlRenderingContext {
    pub const VIEWPORT: u32 = 2978u64 as u32;
}
impl WebGlRenderingContext {
    pub const SCISSOR_BOX: u32 = 3088u64 as u32;
}
impl WebGlRenderingContext {
    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;
}
impl WebGlRenderingContext {
    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;
}
impl WebGlRenderingContext {
    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;
}
impl WebGlRenderingContext {
    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;
}
impl WebGlRenderingContext {
    pub const RED_BITS: u32 = 3410u64 as u32;
}
impl WebGlRenderingContext {
    pub const GREEN_BITS: u32 = 3411u64 as u32;
}
impl WebGlRenderingContext {
    pub const BLUE_BITS: u32 = 3412u64 as u32;
}
impl WebGlRenderingContext {
    pub const ALPHA_BITS: u32 = 3413u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_BITS: u32 = 3414u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_BITS: u32 = 3415u64 as u32;
}
impl WebGlRenderingContext {
    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;
}
impl WebGlRenderingContext {
    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLES: u32 = 32937u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;
}
impl WebGlRenderingContext {
    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;
}
impl WebGlRenderingContext {
    pub const DONT_CARE: u32 = 4352u64 as u32;
}
impl WebGlRenderingContext {
    pub const FASTEST: u32 = 4353u64 as u32;
}
impl WebGlRenderingContext {
    pub const NICEST: u32 = 4354u64 as u32;
}
impl WebGlRenderingContext {
    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;
}
impl WebGlRenderingContext {
    pub const BYTE: u32 = 5120u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;
}
impl WebGlRenderingContext {
    pub const SHORT: u32 = 5122u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;
}
impl WebGlRenderingContext {
    pub const INT: u32 = 5124u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_INT: u32 = 5125u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT: u32 = 5126u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;
}
impl WebGlRenderingContext {
    pub const ALPHA: u32 = 6406u64 as u32;
}
impl WebGlRenderingContext {
    pub const RGB: u32 = 6407u64 as u32;
}
impl WebGlRenderingContext {
    pub const RGBA: u32 = 6408u64 as u32;
}
impl WebGlRenderingContext {
    pub const LUMINANCE: u32 = 6409u64 as u32;
}
impl WebGlRenderingContext {
    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_SHADER: u32 = 35633u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;
}
impl WebGlRenderingContext {
    pub const SHADER_TYPE: u32 = 35663u64 as u32;
}
impl WebGlRenderingContext {
    pub const DELETE_STATUS: u32 = 35712u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINK_STATUS: u32 = 35714u64 as u32;
}
impl WebGlRenderingContext {
    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;
}
impl WebGlRenderingContext {
    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;
}
impl WebGlRenderingContext {
    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;
}
impl WebGlRenderingContext {
    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;
}
impl WebGlRenderingContext {
    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;
}
impl WebGlRenderingContext {
    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;
}
impl WebGlRenderingContext {
    pub const NEVER: u32 = 512u64 as u32;
}
impl WebGlRenderingContext {
    pub const LESS: u32 = 513u64 as u32;
}
impl WebGlRenderingContext {
    pub const EQUAL: u32 = 514u64 as u32;
}
impl WebGlRenderingContext {
    pub const LEQUAL: u32 = 515u64 as u32;
}
impl WebGlRenderingContext {
    pub const GREATER: u32 = 516u64 as u32;
}
impl WebGlRenderingContext {
    pub const NOTEQUAL: u32 = 517u64 as u32;
}
impl WebGlRenderingContext {
    pub const GEQUAL: u32 = 518u64 as u32;
}
impl WebGlRenderingContext {
    pub const ALWAYS: u32 = 519u64 as u32;
}
impl WebGlRenderingContext {
    pub const KEEP: u32 = 7680u64 as u32;
}
impl WebGlRenderingContext {
    pub const REPLACE: u32 = 7681u64 as u32;
}
impl WebGlRenderingContext {
    pub const INCR: u32 = 7682u64 as u32;
}
impl WebGlRenderingContext {
    pub const DECR: u32 = 7683u64 as u32;
}
impl WebGlRenderingContext {
    pub const INVERT: u32 = 5386u64 as u32;
}
impl WebGlRenderingContext {
    pub const INCR_WRAP: u32 = 34055u64 as u32;
}
impl WebGlRenderingContext {
    pub const DECR_WRAP: u32 = 34056u64 as u32;
}
impl WebGlRenderingContext {
    pub const VENDOR: u32 = 7936u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERER: u32 = 7937u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERSION: u32 = 7938u64 as u32;
}
impl WebGlRenderingContext {
    pub const NEAREST: u32 = 9728u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINEAR: u32 = 9729u64 as u32;
}
impl WebGlRenderingContext {
    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;
}
impl WebGlRenderingContext {
    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;
}
impl WebGlRenderingContext {
    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_2D: u32 = 3553u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE: u32 = 5890u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE0: u32 = 33984u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE1: u32 = 33985u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE2: u32 = 33986u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE3: u32 = 33987u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE4: u32 = 33988u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE5: u32 = 33989u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE6: u32 = 33990u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE7: u32 = 33991u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE8: u32 = 33992u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE9: u32 = 33993u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE10: u32 = 33994u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE11: u32 = 33995u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE12: u32 = 33996u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE13: u32 = 33997u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE14: u32 = 33998u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE15: u32 = 33999u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE16: u32 = 34000u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE17: u32 = 34001u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE18: u32 = 34002u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE19: u32 = 34003u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE20: u32 = 34004u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE21: u32 = 34005u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE22: u32 = 34006u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE23: u32 = 34007u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE24: u32 = 34008u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE25: u32 = 34009u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE26: u32 = 34010u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE27: u32 = 34011u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE28: u32 = 34012u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE29: u32 = 34013u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE30: u32 = 34014u64 as u32;
}
impl WebGlRenderingContext {
    pub const TEXTURE31: u32 = 34015u64 as u32;
}
impl WebGlRenderingContext {
    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;
}
impl WebGlRenderingContext {
    pub const REPEAT: u32 = 10497u64 as u32;
}
impl WebGlRenderingContext {
    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;
}
impl WebGlRenderingContext {
    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_VEC2: u32 = 35664u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_VEC3: u32 = 35665u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_VEC4: u32 = 35666u64 as u32;
}
impl WebGlRenderingContext {
    pub const INT_VEC2: u32 = 35667u64 as u32;
}
impl WebGlRenderingContext {
    pub const INT_VEC3: u32 = 35668u64 as u32;
}
impl WebGlRenderingContext {
    pub const INT_VEC4: u32 = 35669u64 as u32;
}
impl WebGlRenderingContext {
    pub const BOOL: u32 = 35670u64 as u32;
}
impl WebGlRenderingContext {
    pub const BOOL_VEC2: u32 = 35671u64 as u32;
}
impl WebGlRenderingContext {
    pub const BOOL_VEC3: u32 = 35672u64 as u32;
}
impl WebGlRenderingContext {
    pub const BOOL_VEC4: u32 = 35673u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_MAT2: u32 = 35674u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_MAT3: u32 = 35675u64 as u32;
}
impl WebGlRenderingContext {
    pub const FLOAT_MAT4: u32 = 35676u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLER_2D: u32 = 35678u64 as u32;
}
impl WebGlRenderingContext {
    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;
}
impl WebGlRenderingContext {
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;
}
impl WebGlRenderingContext {
    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;
}
impl WebGlRenderingContext {
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;
}
impl WebGlRenderingContext {
    pub const COMPILE_STATUS: u32 = 35713u64 as u32;
}
impl WebGlRenderingContext {
    pub const LOW_FLOAT: u32 = 36336u64 as u32;
}
impl WebGlRenderingContext {
    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;
}
impl WebGlRenderingContext {
    pub const HIGH_FLOAT: u32 = 36338u64 as u32;
}
impl WebGlRenderingContext {
    pub const LOW_INT: u32 = 36339u64 as u32;
}
impl WebGlRenderingContext {
    pub const MEDIUM_INT: u32 = 36340u64 as u32;
}
impl WebGlRenderingContext {
    pub const HIGH_INT: u32 = 36341u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER: u32 = 36160u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER: u32 = 36161u64 as u32;
}
impl WebGlRenderingContext {
    pub const RGBA4: u32 = 32854u64 as u32;
}
impl WebGlRenderingContext {
    pub const RGB5_A1: u32 = 32855u64 as u32;
}
impl WebGlRenderingContext {
    pub const RGB565: u32 = 36194u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;
}
impl WebGlRenderingContext {
    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;
}
impl WebGlRenderingContext {
    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;
}
impl WebGlRenderingContext {
    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;
}
impl WebGlRenderingContext {
    pub const NONE: u32 = 0i64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;
}
impl WebGlRenderingContext {
    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;
}
impl WebGlRenderingContext {
    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;
}
impl WebGlRenderingContext {
    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;
}
impl WebGlRenderingContext {
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;
}
impl WebGlRenderingContext {
    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;
}
impl WebGlRenderingContext {
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;
}
impl WebGlRenderingContext {
    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bf44b25a98c712b2: [u8; 23051usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC9Y\0\0\0\0\xB6\x01\0\0\x02\x15WebGLRenderingContext'__widl_instanceof_WebGLRenderingContext\0\0\0\03__widl_f_buffer_data_with_i32_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x04size\x05usage\nbufferData\0\0\03__widl_f_buffer_data_with_f64_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x04size\x05usage\nbufferData\0\0\0@__widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x04data\x05usage\nbufferData\0\0\0A__widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x04data\x05usage\nbufferData\0\0\08__widl_f_buffer_data_with_u8_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x04data\x05usage\nbufferData\0\0\0H__widl_f_buffer_sub_data_with_i32_and_array_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0H__widl_f_buffer_sub_data_with_f64_and_array_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0M__widl_f_buffer_sub_data_with_i32_and_array_buffer_view_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0M__widl_f_buffer_sub_data_with_f64_and_array_buffer_view_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0D__widl_f_buffer_sub_data_with_i32_and_u8_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0D__widl_f_buffer_sub_data_with_f64_and_u8_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x06offset\x04data\rbufferSubData\0\0\0%__widl_f_commit_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x06commit\0\0\0M__widl_f_compressed_tex_image_2d_with_array_buffer_view_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x0Einternalformat\x05width\x06height\x06border\x04data\x14compressedTexImage2D\0\0\0D__widl_f_compressed_tex_image_2d_with_u8_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x0Einternalformat\x05width\x06height\x06border\x04data\x14compressedTexImage2D\0\0\0Q__widl_f_compressed_tex_sub_image_2d_with_array_buffer_view_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\t\x05self_\x06target\x05level\x07xoffset\x07yoffset\x05width\x06height\x06format\x04data\x17compressedTexSubImage2D\0\0\0H__widl_f_compressed_tex_sub_image_2d_with_u8_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\t\x05self_\x06target\x05level\x07xoffset\x07yoffset\x05width\x06height\x06format\x04data\x17compressedTexSubImage2D\0\0\0E__widl_f_read_pixels_with_opt_array_buffer_view_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x01x\x01y\x05width\x06height\x06format\x05type_\x06pixels\nreadPixels\0\0\0<__widl_f_read_pixels_with_opt_u8_array_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x01x\x01y\x05width\x06height\x06format\x05type_\x06pixels\nreadPixels\0\0\0r__widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\n\x05self_\x06target\x05level\x0Einternalformat\x05width\x06height\x06border\x06format\x05type_\x06pixels\ntexImage2D\0\0\0i__widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\n\x05self_\x06target\x05level\x0Einternalformat\x05width\x06height\x06border\x06format\x05type_\x06pixels\ntexImage2D\0\0\0M__widl_f_tex_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x06target\x05level\x0Einternalformat\x06format\x05type_\x06pixels\ntexImage2D\0\0\0K__widl_f_tex_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x06target\x05level\x0Einternalformat\x06format\x05type_\x06pixels\ntexImage2D\0\0\0F__widl_f_tex_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x06target\x05level\x0Einternalformat\x06format\x05type_\x05image\ntexImage2D\0\0\0G__widl_f_tex_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x06target\x05level\x0Einternalformat\x06format\x05type_\x06canvas\ntexImage2D\0\0\0F__widl_f_tex_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x06target\x05level\x0Einternalformat\x06format\x05type_\x05video\ntexImage2D\0\0\0k__widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\n\x05self_\x06target\x05level\x07xoffset\x07yoffset\x05width\x06height\x06format\x05type_\x06pixels\rtexSubImage2D\0\0\0b__widl_f_tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\n\x05self_\x06target\x05level\x07xoffset\x07yoffset\x05width\x06height\x06format\x05type_\x06pixels\rtexSubImage2D\0\0\0Q__widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_bitmap_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x07xoffset\x07yoffset\x06format\x05type_\x06pixels\rtexSubImage2D\0\0\0O__widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_data_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x07xoffset\x07yoffset\x06format\x05type_\x06pixels\rtexSubImage2D\0\0\0J__widl_f_tex_sub_image_2d_with_u32_and_u32_and_image_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x07xoffset\x07yoffset\x06format\x05type_\x05image\rtexSubImage2D\0\0\0K__widl_f_tex_sub_image_2d_with_u32_and_u32_and_canvas_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x07xoffset\x07yoffset\x06format\x05type_\x06canvas\rtexSubImage2D\0\0\0J__widl_f_tex_sub_image_2d_with_u32_and_u32_and_video_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x08\x05self_\x06target\x05level\x07xoffset\x07yoffset\x06format\x05type_\x05video\rtexSubImage2D\0\0\08__widl_f_uniform1fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform1fv\0\0\0;__widl_f_uniform1fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform1fv\0\0\08__widl_f_uniform1iv_with_i32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform1iv\0\0\0;__widl_f_uniform1iv_with_i32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform1iv\0\0\08__widl_f_uniform2fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform2fv\0\0\0;__widl_f_uniform2fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform2fv\0\0\08__widl_f_uniform2iv_with_i32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform2iv\0\0\0;__widl_f_uniform2iv_with_i32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform2iv\0\0\08__widl_f_uniform3fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform3fv\0\0\0;__widl_f_uniform3fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform3fv\0\0\08__widl_f_uniform3iv_with_i32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform3iv\0\0\0;__widl_f_uniform3iv_with_i32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform3iv\0\0\08__widl_f_uniform4fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform4fv\0\0\0;__widl_f_uniform4fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform4fv\0\0\08__widl_f_uniform4iv_with_i32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform4iv\0\0\0;__widl_f_uniform4iv_with_i32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x04data\nuniform4iv\0\0\0?__widl_f_uniform_matrix2fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix2fv\0\0\0B__widl_f_uniform_matrix2fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix2fv\0\0\0?__widl_f_uniform_matrix3fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix3fv\0\0\0B__widl_f_uniform_matrix3fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix3fv\0\0\0?__widl_f_uniform_matrix4fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix4fv\0\0\0B__widl_f_uniform_matrix4fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\ttranspose\x04data\x10uniformMatrix4fv\0\0\0-__widl_f_active_texture_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07texture\ractiveTexture\0\0\0,__widl_f_attach_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x06shader\x0CattachShader\0\0\03__widl_f_bind_attrib_location_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x07program\x05index\x04name\x12bindAttribLocation\0\0\0*__widl_f_bind_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x06buffer\nbindBuffer\0\0\0/__widl_f_bind_framebuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x0Bframebuffer\x0FbindFramebuffer\0\0\00__widl_f_bind_renderbuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x0Crenderbuffer\x10bindRenderbuffer\0\0\0+__widl_f_bind_texture_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x07texture\x0BbindTexture\0\0\0*__widl_f_blend_color_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x03red\x05green\x04blue\x05alpha\nblendColor\0\0\0-__widl_f_blend_equation_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04mode\rblendEquation\0\0\06__widl_f_blend_equation_separate_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08mode_rgb\nmode_alpha\x15blendEquationSeparate\0\0\0)__widl_f_blend_func_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07sfactor\x07dfactor\tblendFunc\0\0\02__widl_f_blend_func_separate_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x07src_rgb\x07dst_rgb\tsrc_alpha\tdst_alpha\x11blendFuncSeparate\0\0\07__widl_f_check_framebuffer_status_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06target\x16checkFramebufferStatus\0\0\0$__widl_f_clear_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04mask\x05clear\0\0\0*__widl_f_clear_color_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x03red\x05green\x04blue\x05alpha\nclearColor\0\0\0*__widl_f_clear_depth_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05depth\nclearDepth\0\0\0,__widl_f_clear_stencil_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x01s\x0CclearStencil\0\0\0)__widl_f_color_mask_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x03red\x05green\x04blue\x05alpha\tcolorMask\0\0\0-__widl_f_compile_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06shader\rcompileShader\0\0\00__widl_f_copy_tex_image_2d_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\t\x05self_\x06target\x05level\x0Einternalformat\x01x\x01y\x05width\x06height\x06border\x0EcopyTexImage2D\0\0\04__widl_f_copy_tex_sub_image_2d_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\t\x05self_\x06target\x05level\x07xoffset\x07yoffset\x01x\x01y\x05width\x06height\x11copyTexSubImage2D\0\0\0,__widl_f_create_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x0CcreateBuffer\0\0\01__widl_f_create_framebuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x11createFramebuffer\0\0\0-__widl_f_create_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\rcreateProgram\0\0\02__widl_f_create_renderbuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x12createRenderbuffer\0\0\0,__widl_f_create_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05type_\x0CcreateShader\0\0\0-__widl_f_create_texture_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\rcreateTexture\0\0\0(__widl_f_cull_face_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04mode\x08cullFace\0\0\0,__widl_f_delete_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06buffer\x0CdeleteBuffer\0\0\01__widl_f_delete_framebuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x0Bframebuffer\x11deleteFramebuffer\0\0\0-__widl_f_delete_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\rdeleteProgram\0\0\02__widl_f_delete_renderbuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x0Crenderbuffer\x12deleteRenderbuffer\0\0\0,__widl_f_delete_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06shader\x0CdeleteShader\0\0\0-__widl_f_delete_texture_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07texture\rdeleteTexture\0\0\0)__widl_f_depth_func_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04func\tdepthFunc\0\0\0)__widl_f_depth_mask_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04flag\tdepthMask\0\0\0*__widl_f_depth_range_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06z_near\x05z_far\ndepthRange\0\0\0,__widl_f_detach_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x06shader\x0CdetachShader\0\0\0&__widl_f_disable_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x03cap\x07disable\0\0\0:__widl_f_disable_vertex_attrib_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05index\x18disableVertexAttribArray\0\0\0*__widl_f_draw_arrays_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x04mode\x05first\x05count\ndrawArrays\0\0\05__widl_f_draw_elements_with_i32_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x04mode\x05count\x05type_\x06offset\x0CdrawElements\0\0\05__widl_f_draw_elements_with_f64_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x04mode\x05count\x05type_\x06offset\x0CdrawElements\0\0\0%__widl_f_enable_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x03cap\x06enable\0\0\09__widl_f_enable_vertex_attrib_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05index\x17enableVertexAttribArray\0\0\0%__widl_f_finish_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x06finish\0\0\0$__widl_f_flush_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x05flush\0\0\07__widl_f_framebuffer_renderbuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x06target\nattachment\x12renderbuffertarget\x0Crenderbuffer\x17framebufferRenderbuffer\0\0\05__widl_f_framebuffer_texture_2d_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x06\x05self_\x06target\nattachment\ttextarget\x07texture\x05level\x14framebufferTexture2D\0\0\0)__widl_f_front_face_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04mode\tfrontFace\0\0\0.__widl_f_generate_mipmap_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06target\x0EgenerateMipmap\0\0\00__widl_f_get_active_attrib_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x05index\x0FgetActiveAttrib\0\0\01__widl_f_get_active_uniform_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x05index\x10getActiveUniform\0\0\03__widl_f_get_attached_shaders_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\x12getAttachedShaders\0\0\02__widl_f_get_attrib_location_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x04name\x11getAttribLocation\0\0\03__widl_f_get_buffer_parameter_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x05pname\x12getBufferParameter\0\0\05__widl_f_get_context_attributes_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x14getContextAttributes\0\0\0(__widl_f_get_error_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x08getError\0\0\0,__widl_f_get_extension_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04name\x0CgetExtension\0\0\0C__widl_f_get_framebuffer_attachment_parameter_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\nattachment\x05pname!getFramebufferAttachmentParameter\0\0\0,__widl_f_get_parameter_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05pname\x0CgetParameter\0\0\03__widl_f_get_program_info_log_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\x11getProgramInfoLog\0\0\04__widl_f_get_program_parameter_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x05pname\x13getProgramParameter\0\0\09__widl_f_get_renderbuffer_parameter_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x05pname\x18getRenderbufferParameter\0\0\02__widl_f_get_shader_info_log_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06shader\x10getShaderInfoLog\0\0\03__widl_f_get_shader_parameter_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06shader\x05pname\x12getShaderParameter\0\0\0:__widl_f_get_shader_precision_format_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\nshadertype\rprecisiontype\x18getShaderPrecisionFormat\0\0\00__widl_f_get_shader_source_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06shader\x0FgetShaderSource\0\0\07__widl_f_get_supported_extensions_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\x16getSupportedExtensions\0\0\00__widl_f_get_tex_parameter_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x05pname\x0FgetTexParameter\0\0\0*__widl_f_get_uniform_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x08location\ngetUniform\0\0\03__widl_f_get_uniform_location_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x07program\x04name\x12getUniformLocation\0\0\00__widl_f_get_vertex_attrib_WebGLRenderingContext\x01\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x05index\x05pname\x0FgetVertexAttrib\0\0\07__widl_f_get_vertex_attrib_offset_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x05index\x05pname\x15getVertexAttribOffset\0\0\0#__widl_f_hint_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06target\x04mode\x04hint\0\0\0(__widl_f_is_buffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06buffer\x08isBuffer\0\0\0.__widl_f_is_context_lost_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x01\x05self_\risContextLost\0\0\0)__widl_f_is_enabled_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x03cap\tisEnabled\0\0\0-__widl_f_is_framebuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x0Bframebuffer\risFramebuffer\0\0\0)__widl_f_is_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\tisProgram\0\0\0.__widl_f_is_renderbuffer_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x0Crenderbuffer\x0EisRenderbuffer\0\0\0(__widl_f_is_shader_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x06shader\x08isShader\0\0\0)__widl_f_is_texture_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07texture\tisTexture\0\0\0)__widl_f_line_width_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x05width\tlineWidth\0\0\0+__widl_f_link_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\x0BlinkProgram\0\0\0+__widl_f_pixel_storei_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x05pname\x05param\x0BpixelStorei\0\0\0-__widl_f_polygon_offset_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06factor\x05units\rpolygonOffset\0\0\03__widl_f_renderbuffer_storage_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x06target\x0Einternalformat\x05width\x06height\x13renderbufferStorage\0\0\0.__widl_f_sample_coverage_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x05value\x06invert\x0EsampleCoverage\0\0\0&__widl_f_scissor_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x01x\x01y\x05width\x06height\x07scissor\0\0\0,__widl_f_shader_source_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x06shader\x06source\x0CshaderSource\0\0\0+__widl_f_stencil_func_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x04func\x04ref_\x04mask\x0BstencilFunc\0\0\04__widl_f_stencil_func_separate_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x04face\x04func\x04ref_\x04mask\x13stencilFuncSeparate\0\0\0+__widl_f_stencil_mask_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x04mask\x0BstencilMask\0\0\04__widl_f_stencil_mask_separate_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04face\x04mask\x13stencilMaskSeparate\0\0\0)__widl_f_stencil_op_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x04fail\x05zfail\x05zpass\tstencilOp\0\0\02__widl_f_stencil_op_separate_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x04face\x04fail\x05zfail\x05zpass\x11stencilOpSeparate\0\0\0-__widl_f_tex_parameterf_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x05pname\x05param\rtexParameterf\0\0\0-__widl_f_tex_parameteri_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x06target\x05pname\x05param\rtexParameteri\0\0\0(__widl_f_uniform1f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x01x\tuniform1f\0\0\0(__widl_f_uniform1i_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x08location\x01x\tuniform1i\0\0\0(__widl_f_uniform2f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\x01x\x01y\tuniform2f\0\0\0(__widl_f_uniform2i_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x08location\x01x\x01y\tuniform2i\0\0\0(__widl_f_uniform3f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x08location\x01x\x01y\x01z\tuniform3f\0\0\0(__widl_f_uniform3i_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x08location\x01x\x01y\x01z\tuniform3i\0\0\0(__widl_f_uniform4f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x06\x05self_\x08location\x01x\x01y\x01z\x01w\tuniform4f\0\0\0(__widl_f_uniform4i_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x06\x05self_\x08location\x01x\x01y\x01z\x01w\tuniform4i\0\0\0*__widl_f_use_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\nuseProgram\0\0\0/__widl_f_validate_program_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x02\x05self_\x07program\x0FvalidateProgram\0\0\0.__widl_f_vertex_attrib1f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x01x\x0EvertexAttrib1f\0\0\0>__widl_f_vertex_attrib1fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib1fv\0\0\0A__widl_f_vertex_attrib1fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib1fv\0\0\0.__widl_f_vertex_attrib2f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x04\x05self_\x04indx\x01x\x01y\x0EvertexAttrib2f\0\0\0>__widl_f_vertex_attrib2fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib2fv\0\0\0A__widl_f_vertex_attrib2fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib2fv\0\0\0.__widl_f_vertex_attrib3f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x04indx\x01x\x01y\x01z\x0EvertexAttrib3f\0\0\0>__widl_f_vertex_attrib3fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib3fv\0\0\0A__widl_f_vertex_attrib3fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib3fv\0\0\0.__widl_f_vertex_attrib4f_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x06\x05self_\x04indx\x01x\x01y\x01z\x01w\x0EvertexAttrib4f\0\0\0>__widl_f_vertex_attrib4fv_with_f32_array_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib4fv\0\0\0A__widl_f_vertex_attrib4fv_with_f32_sequence_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x03\x05self_\x04indx\x06values\x0FvertexAttrib4fv\0\0\0=__widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x04indx\x04size\x05type_\nnormalized\x06stride\x06offset\x13vertexAttribPointer\0\0\0=__widl_f_vertex_attrib_pointer_with_f64_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x07\x05self_\x04indx\x04size\x05type_\nnormalized\x06stride\x06offset\x13vertexAttribPointer\0\0\0'__widl_f_viewport_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\0\x01\x05\x05self_\x01x\x01y\x05width\x06height\x08viewport\0\0\0%__widl_f_canvas_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\x01\x06canvas\x01\x01\x05self_\x06canvas\0\0\03__widl_f_drawing_buffer_width_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\x01\x12drawingBufferWidth\x01\x01\x05self_\x12drawingBufferWidth\0\0\04__widl_f_drawing_buffer_height_WebGLRenderingContext\0\0\0\x01\x15WebGLRenderingContext\x01\0\x01\x13drawingBufferHeight\x01\x01\x05self_\x13drawingBufferHeight\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

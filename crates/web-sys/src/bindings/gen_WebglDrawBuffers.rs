use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WEBGL_draw_buffers` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers)\n\n*This API requires the following crate features to be activated: `WebglDrawBuffers`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WebglDrawBuffers {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WebglDrawBuffers: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WebglDrawBuffers {
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
            inform(100u32);
            inform(114u32);
            inform(97u32);
            inform(119u32);
            inform(95u32);
            inform(98u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for WebglDrawBuffers {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WebglDrawBuffers {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WebglDrawBuffers {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WebglDrawBuffers {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WebglDrawBuffers {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WebglDrawBuffers {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WebglDrawBuffers {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WebglDrawBuffers {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WebglDrawBuffers {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WebglDrawBuffers>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WebglDrawBuffers {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WebglDrawBuffers {
        #[inline]
        fn from(obj: JsValue) -> WebglDrawBuffers {
            WebglDrawBuffers { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WebglDrawBuffers {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WebglDrawBuffers> for WebglDrawBuffers {
        #[inline]
        fn as_ref(&self) -> &WebglDrawBuffers {
            self
        }
    }
    impl From<WebglDrawBuffers> for JsValue {
        #[inline]
        fn from(obj: WebglDrawBuffers) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WebglDrawBuffers {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WEBGL_draw_buffers(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WEBGL_draw_buffers(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WEBGL_draw_buffers(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WebglDrawBuffers { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WebglDrawBuffers) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WebglDrawBuffers> for ::js_sys::Object {
    #[inline]
    fn from(obj: WebglDrawBuffers) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WebglDrawBuffers {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WebglDrawBuffers",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draw_buffers_webgl_WEBGL_draw_buffers() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WebglDrawBuffers as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl WebglDrawBuffers {
    #[cfg(all(feature = "WebglDrawBuffers",))]
    #[allow(bad_style)]
    #[doc = "The `drawBuffersWEBGL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_buffers/drawBuffersWEBGL)\n\n*This API requires the following crate features to be activated: `WebglDrawBuffers`*"]
    #[allow(clippy::all)]
    pub fn draw_buffers_webgl(&self, buffers: &::wasm_bindgen::JsValue) {
        #[cfg(all(feature = "WebglDrawBuffers",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draw_buffers_webgl_WEBGL_draw_buffers(
                self_: <&WebglDrawBuffers as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffers: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draw_buffers_webgl_WEBGL_draw_buffers(
            self_: <&WebglDrawBuffers as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffers: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(buffers);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&WebglDrawBuffers as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffers =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        buffers,
                    );
                __widl_f_draw_buffers_webgl_WEBGL_draw_buffers(self_, buffers)
            };
            ()
        }
    }
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT0_WEBGL: u32 = 36064u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT1_WEBGL: u32 = 36065u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT2_WEBGL: u32 = 36066u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT3_WEBGL: u32 = 36067u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT4_WEBGL: u32 = 36068u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT5_WEBGL: u32 = 36069u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT6_WEBGL: u32 = 36070u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT7_WEBGL: u32 = 36071u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT8_WEBGL: u32 = 36072u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT9_WEBGL: u32 = 36073u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT10_WEBGL: u32 = 36074u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT11_WEBGL: u32 = 36075u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT12_WEBGL: u32 = 36076u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT13_WEBGL: u32 = 36077u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT14_WEBGL: u32 = 36078u64 as u32;
}
impl WebglDrawBuffers {
    pub const COLOR_ATTACHMENT15_WEBGL: u32 = 36079u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER0_WEBGL: u32 = 34853u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER1_WEBGL: u32 = 34854u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER2_WEBGL: u32 = 34855u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER3_WEBGL: u32 = 34856u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER4_WEBGL: u32 = 34857u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER5_WEBGL: u32 = 34858u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER6_WEBGL: u32 = 34859u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER7_WEBGL: u32 = 34860u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER8_WEBGL: u32 = 34861u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER9_WEBGL: u32 = 34862u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER10_WEBGL: u32 = 34863u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER11_WEBGL: u32 = 34864u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER12_WEBGL: u32 = 34865u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER13_WEBGL: u32 = 34866u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER14_WEBGL: u32 = 34867u64 as u32;
}
impl WebglDrawBuffers {
    pub const DRAW_BUFFER15_WEBGL: u32 = 34868u64 as u32;
}
impl WebglDrawBuffers {
    pub const MAX_COLOR_ATTACHMENTS_WEBGL: u32 = 36063u64 as u32;
}
impl WebglDrawBuffers {
    pub const MAX_DRAW_BUFFERS_WEBGL: u32 = 34852u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a47a640231bdd16c: [u8; 274usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD0\0\0\0\0\0\x02\0\0\x02\x12WEBGL_draw_buffers$__widl_instanceof_WEBGL_draw_buffers\0\0\0\0.__widl_f_draw_buffers_webgl_WEBGL_draw_buffers\0\0\0\x01\x12WEBGL_draw_buffers\x01\0\0\x01\x02\x05self_\x07buffers\x10drawBuffersWEBGL\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

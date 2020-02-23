use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CanvasCaptureMediaStream` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CanvasCaptureMediaStream {
    obj: MediaStream,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CanvasCaptureMediaStream: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CanvasCaptureMediaStream {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
            inform(67u32);
            inform(97u32);
            inform(112u32);
            inform(116u32);
            inform(117u32);
            inform(114u32);
            inform(101u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(109u32);
        }
    }
    impl core::ops::Deref for CanvasCaptureMediaStream {
        type Target = MediaStream;
        #[inline]
        fn deref(&self) -> &MediaStream {
            &self.obj
        }
    }
    impl IntoWasmAbi for CanvasCaptureMediaStream {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CanvasCaptureMediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CanvasCaptureMediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CanvasCaptureMediaStream {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CanvasCaptureMediaStream {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CanvasCaptureMediaStream {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CanvasCaptureMediaStream {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CanvasCaptureMediaStream {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CanvasCaptureMediaStream>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CanvasCaptureMediaStream {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CanvasCaptureMediaStream {
        #[inline]
        fn from(obj: JsValue) -> CanvasCaptureMediaStream {
            CanvasCaptureMediaStream { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CanvasCaptureMediaStream {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CanvasCaptureMediaStream> for CanvasCaptureMediaStream {
        #[inline]
        fn as_ref(&self) -> &CanvasCaptureMediaStream {
            self
        }
    }
    impl From<CanvasCaptureMediaStream> for JsValue {
        #[inline]
        fn from(obj: CanvasCaptureMediaStream) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CanvasCaptureMediaStream {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CanvasCaptureMediaStream(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CanvasCaptureMediaStream(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CanvasCaptureMediaStream(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CanvasCaptureMediaStream { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CanvasCaptureMediaStream) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CanvasCaptureMediaStream> for MediaStream {
    #[inline]
    fn from(obj: CanvasCaptureMediaStream) -> MediaStream {
        use wasm_bindgen::JsCast;
        MediaStream::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MediaStream> for CanvasCaptureMediaStream {
    #[inline]
    fn as_ref(&self) -> &MediaStream {
        use wasm_bindgen::JsCast;
        MediaStream::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CanvasCaptureMediaStream> for EventTarget {
    #[inline]
    fn from(obj: CanvasCaptureMediaStream) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for CanvasCaptureMediaStream {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CanvasCaptureMediaStream> for ::js_sys::Object {
    #[inline]
    fn from(obj: CanvasCaptureMediaStream) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CanvasCaptureMediaStream {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CanvasCaptureMediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_frame_CanvasCaptureMediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasCaptureMediaStream as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CanvasCaptureMediaStream {
    #[cfg(all(feature = "CanvasCaptureMediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `requestFrame()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/requestFrame)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`*"]
    #[allow(clippy::all)]
    pub fn request_frame(&self) {
        #[cfg(all(feature = "CanvasCaptureMediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_frame_CanvasCaptureMediaStream(
                self_: <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_frame_CanvasCaptureMediaStream(
            self_: <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_request_frame_CanvasCaptureMediaStream(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CanvasCaptureMediaStream", feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_canvas_CanvasCaptureMediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CanvasCaptureMediaStream as WasmDescribe>::describe();
    <HtmlCanvasElement as WasmDescribe>::describe();
}
impl CanvasCaptureMediaStream {
    #[cfg(all(feature = "CanvasCaptureMediaStream", feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `canvas` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStream/canvas)\n\n*This API requires the following crate features to be activated: `CanvasCaptureMediaStream`, `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn canvas(&self) -> HtmlCanvasElement {
        #[cfg(all(feature = "CanvasCaptureMediaStream", feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_canvas_CanvasCaptureMediaStream(
                self_: <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCanvasElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_canvas_CanvasCaptureMediaStream(
            self_: <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCanvasElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CanvasCaptureMediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_canvas_CanvasCaptureMediaStream(self_)
            };
            <HtmlCanvasElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8322d910bbd580be: [u8; 379usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}9\x01\0\0\0\0\x03\0\0\x02\x18CanvasCaptureMediaStream*__widl_instanceof_CanvasCaptureMediaStream\0\0\0\0/__widl_f_request_frame_CanvasCaptureMediaStream\0\0\0\x01\x18CanvasCaptureMediaStream\x01\0\0\x01\x01\x05self_\x0CrequestFrame\0\0\0(__widl_f_canvas_CanvasCaptureMediaStream\0\0\0\x01\x18CanvasCaptureMediaStream\x01\0\x01\x06canvas\x01\x01\x05self_\x06canvas\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

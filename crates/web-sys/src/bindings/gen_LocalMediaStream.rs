use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `LocalMediaStream` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream)\n\n*This API requires the following crate features to be activated: `LocalMediaStream`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct LocalMediaStream {
    obj: MediaStream,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_LocalMediaStream: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for LocalMediaStream {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(76u32);
            inform(111u32);
            inform(99u32);
            inform(97u32);
            inform(108u32);
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
    impl core::ops::Deref for LocalMediaStream {
        type Target = MediaStream;
        #[inline]
        fn deref(&self) -> &MediaStream {
            &self.obj
        }
    }
    impl IntoWasmAbi for LocalMediaStream {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for LocalMediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a LocalMediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for LocalMediaStream {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            LocalMediaStream {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for LocalMediaStream {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a LocalMediaStream {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for LocalMediaStream {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<LocalMediaStream>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(LocalMediaStream {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for LocalMediaStream {
        #[inline]
        fn from(obj: JsValue) -> LocalMediaStream {
            LocalMediaStream { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for LocalMediaStream {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<LocalMediaStream> for LocalMediaStream {
        #[inline]
        fn as_ref(&self) -> &LocalMediaStream {
            self
        }
    }
    impl From<LocalMediaStream> for JsValue {
        #[inline]
        fn from(obj: LocalMediaStream) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for LocalMediaStream {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_LocalMediaStream(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_LocalMediaStream(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_LocalMediaStream(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            LocalMediaStream { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const LocalMediaStream) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<LocalMediaStream> for MediaStream {
    #[inline]
    fn from(obj: LocalMediaStream) -> MediaStream {
        use wasm_bindgen::JsCast;
        MediaStream::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MediaStream> for LocalMediaStream {
    #[inline]
    fn as_ref(&self) -> &MediaStream {
        use wasm_bindgen::JsCast;
        MediaStream::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<LocalMediaStream> for EventTarget {
    #[inline]
    fn from(obj: LocalMediaStream) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for LocalMediaStream {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<LocalMediaStream> for ::js_sys::Object {
    #[inline]
    fn from(obj: LocalMediaStream) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for LocalMediaStream {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "LocalMediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_LocalMediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&LocalMediaStream as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl LocalMediaStream {
    #[cfg(all(feature = "LocalMediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream/stop)\n\n*This API requires the following crate features to be activated: `LocalMediaStream`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) {
        #[cfg(all(feature = "LocalMediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_LocalMediaStream(
                self_: <&LocalMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_LocalMediaStream(
            self_: <&LocalMediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&LocalMediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_LocalMediaStream(self_)
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
pub static __WASM_BINDGEN_GENERATED_1dcdd08a5eb80ca6: [u8; 232usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA6\0\0\0\0\0\x02\0\0\x02\x10LocalMediaStream\"__widl_instanceof_LocalMediaStream\0\0\0\0\x1E__widl_f_stop_LocalMediaStream\0\0\0\x01\x10LocalMediaStream\x01\0\0\x01\x01\x05self_\x04stop\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

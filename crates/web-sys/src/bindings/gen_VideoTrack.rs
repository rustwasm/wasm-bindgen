use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `VideoTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct VideoTrack {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_VideoTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for VideoTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(86u32);
            inform(105u32);
            inform(100u32);
            inform(101u32);
            inform(111u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for VideoTrack {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for VideoTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for VideoTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a VideoTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for VideoTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            VideoTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for VideoTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a VideoTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for VideoTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<VideoTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(VideoTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for VideoTrack {
        #[inline]
        fn from(obj: JsValue) -> VideoTrack {
            VideoTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for VideoTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<VideoTrack> for VideoTrack {
        #[inline]
        fn as_ref(&self) -> &VideoTrack {
            self
        }
    }
    impl From<VideoTrack> for JsValue {
        #[inline]
        fn from(obj: VideoTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for VideoTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_VideoTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_VideoTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_VideoTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            VideoTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const VideoTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<VideoTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: VideoTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for VideoTrack {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_VideoTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> String {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_VideoTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_VideoTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `language` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn language(&self) -> String {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_VideoTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&VideoTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `selected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn selected(&self) -> bool {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_VideoTrack(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "VideoTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selected_VideoTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&VideoTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl VideoTrack {
    #[cfg(all(feature = "VideoTrack",))]
    #[allow(bad_style)]
    #[doc = "The `selected` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)\n\n*This API requires the following crate features to be activated: `VideoTrack`*"]
    #[allow(clippy::all)]
    pub fn set_selected(&self, selected: bool) {
        #[cfg(all(feature = "VideoTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selected_VideoTrack(
                self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selected_VideoTrack(
            self_: <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selected);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&VideoTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selected = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selected);
                __widl_f_set_selected_VideoTrack(self_, selected)
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
pub static __WASM_BINDGEN_GENERATED_e17866f72f7f2ee0: [u8; 579usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x01\x02\0\0\0\0\x07\0\0\x02\nVideoTrack\x1C__widl_instanceof_VideoTrack\0\0\0\0\x16__widl_f_id_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x18__widl_f_kind_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x19__widl_f_label_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0\x1C__widl_f_language_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x01\x08language\x01\x01\x05self_\x08language\0\0\0\x1C__widl_f_selected_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x01\x08selected\x01\x01\x05self_\x08selected\0\0\0 __widl_f_set_selected_VideoTrack\0\0\0\x01\nVideoTrack\x01\0\x02\x08selected\x01\x02\x05self_\x08selected\x08selected\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

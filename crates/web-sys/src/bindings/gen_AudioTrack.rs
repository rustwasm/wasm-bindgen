use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioTrack {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for AudioTrack {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioTrack {
        #[inline]
        fn from(obj: JsValue) -> AudioTrack {
            AudioTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioTrack> for AudioTrack {
        #[inline]
        fn as_ref(&self) -> &AudioTrack {
            self
        }
    }
    impl From<AudioTrack> for JsValue {
        #[inline]
        fn from(obj: AudioTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioTrack {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_AudioTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> String {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_AudioTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_AudioTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `language` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn language(&self) -> String {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_AudioTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enabled_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `enabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn enabled(&self) -> bool {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enabled_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enabled_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_enabled_AudioTrack(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_enabled_AudioTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioTrack as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioTrack {
    #[cfg(all(feature = "AudioTrack",))]
    #[allow(bad_style)]
    #[doc = "The `enabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)\n\n*This API requires the following crate features to be activated: `AudioTrack`*"]
    #[allow(clippy::all)]
    pub fn set_enabled(&self, enabled: bool) {
        #[cfg(all(feature = "AudioTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_enabled_AudioTrack(
                self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_enabled_AudioTrack(
            self_: <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            enabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(enabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let enabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(enabled);
                __widl_f_set_enabled_AudioTrack(self_, enabled)
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
pub static __WASM_BINDGEN_GENERATED_bc8c9392c2c02827: [u8; 572usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xFA\x01\0\0\0\0\x07\0\0\x02\nAudioTrack\x1C__widl_instanceof_AudioTrack\0\0\0\0\x16__widl_f_id_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x18__widl_f_kind_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x19__widl_f_label_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0\x1C__widl_f_language_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x01\x08language\x01\x01\x05self_\x08language\0\0\0\x1B__widl_f_enabled_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x01\x07enabled\x01\x01\x05self_\x07enabled\0\0\0\x1F__widl_f_set_enabled_AudioTrack\0\0\0\x01\nAudioTrack\x01\0\x02\x07enabled\x01\x02\x05self_\x07enabled\x07enabled\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

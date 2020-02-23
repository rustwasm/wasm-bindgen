use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioWorkletGlobalScope` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioWorkletGlobalScope {
    obj: WorkletGlobalScope,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioWorkletGlobalScope: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioWorkletGlobalScope {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(108u32);
            inform(101u32);
            inform(116u32);
            inform(71u32);
            inform(108u32);
            inform(111u32);
            inform(98u32);
            inform(97u32);
            inform(108u32);
            inform(83u32);
            inform(99u32);
            inform(111u32);
            inform(112u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AudioWorkletGlobalScope {
        type Target = WorkletGlobalScope;
        #[inline]
        fn deref(&self) -> &WorkletGlobalScope {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioWorkletGlobalScope {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioWorkletGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioWorkletGlobalScope {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioWorkletGlobalScope {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioWorkletGlobalScope {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioWorkletGlobalScope {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioWorkletGlobalScope {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioWorkletGlobalScope {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioWorkletGlobalScope>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioWorkletGlobalScope {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioWorkletGlobalScope {
        #[inline]
        fn from(obj: JsValue) -> AudioWorkletGlobalScope {
            AudioWorkletGlobalScope { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioWorkletGlobalScope {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioWorkletGlobalScope> for AudioWorkletGlobalScope {
        #[inline]
        fn as_ref(&self) -> &AudioWorkletGlobalScope {
            self
        }
    }
    impl From<AudioWorkletGlobalScope> for JsValue {
        #[inline]
        fn from(obj: AudioWorkletGlobalScope) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioWorkletGlobalScope {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioWorkletGlobalScope(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioWorkletGlobalScope(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioWorkletGlobalScope(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioWorkletGlobalScope { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioWorkletGlobalScope) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioWorkletGlobalScope> for WorkletGlobalScope {
    #[inline]
    fn from(obj: AudioWorkletGlobalScope) -> WorkletGlobalScope {
        use wasm_bindgen::JsCast;
        WorkletGlobalScope::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<WorkletGlobalScope> for AudioWorkletGlobalScope {
    #[inline]
    fn as_ref(&self) -> &WorkletGlobalScope {
        use wasm_bindgen::JsCast;
        WorkletGlobalScope::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioWorkletGlobalScope> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioWorkletGlobalScope) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioWorkletGlobalScope {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioWorkletGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_processor_AudioWorkletGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioWorkletGlobalScope as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioWorkletGlobalScope {
    #[cfg(all(feature = "AudioWorkletGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `registerProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn register_processor(&self, name: &str, processor_ctor: &::js_sys::Function) {
        #[cfg(all(feature = "AudioWorkletGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_processor_AudioWorkletGlobalScope(
                self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                processor_ctor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_processor_AudioWorkletGlobalScope(
            self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            processor_ctor: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            drop(processor_ctor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let processor_ctor =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        processor_ctor,
                    );
                __widl_f_register_processor_AudioWorkletGlobalScope(self_, name, processor_ctor)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioWorkletGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_frame_AudioWorkletGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletGlobalScope as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioWorkletGlobalScope {
    #[cfg(all(feature = "AudioWorkletGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `currentFrame` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn current_frame(&self) -> f64 {
        #[cfg(all(feature = "AudioWorkletGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_frame_AudioWorkletGlobalScope(
                self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_frame_AudioWorkletGlobalScope(
            self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_current_frame_AudioWorkletGlobalScope(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioWorkletGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_AudioWorkletGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletGlobalScope as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioWorkletGlobalScope {
    #[cfg(all(feature = "AudioWorkletGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> f64 {
        #[cfg(all(feature = "AudioWorkletGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_AudioWorkletGlobalScope(
                self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_AudioWorkletGlobalScope(
            self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_current_time_AudioWorkletGlobalScope(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioWorkletGlobalScope",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_rate_AudioWorkletGlobalScope() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletGlobalScope as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioWorkletGlobalScope {
    #[cfg(all(feature = "AudioWorkletGlobalScope",))]
    #[allow(bad_style)]
    #[doc = "The `sampleRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
    #[allow(clippy::all)]
    pub fn sample_rate(&self) -> f32 {
        #[cfg(all(feature = "AudioWorkletGlobalScope",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_rate_AudioWorkletGlobalScope(
                self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_rate_AudioWorkletGlobalScope(
            self_: <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletGlobalScope as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_sample_rate_AudioWorkletGlobalScope(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_255a17e4942ce835: [u8; 643usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}A\x02\0\0\0\0\x05\0\0\x02\x17AudioWorkletGlobalScope)__widl_instanceof_AudioWorkletGlobalScope\0\0\0\03__widl_f_register_processor_AudioWorkletGlobalScope\0\0\0\x01\x17AudioWorkletGlobalScope\x01\0\0\x01\x03\x05self_\x04name\x0Eprocessor_ctor\x11registerProcessor\0\0\0.__widl_f_current_frame_AudioWorkletGlobalScope\0\0\0\x01\x17AudioWorkletGlobalScope\x01\0\x01\x0CcurrentFrame\x01\x01\x05self_\x0CcurrentFrame\0\0\0-__widl_f_current_time_AudioWorkletGlobalScope\0\0\0\x01\x17AudioWorkletGlobalScope\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0,__widl_f_sample_rate_AudioWorkletGlobalScope\0\0\0\x01\x17AudioWorkletGlobalScope\x01\0\x01\nsampleRate\x01\x01\x05self_\nsampleRate\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioProcessingEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)\n\n*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioProcessingEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioProcessingEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioProcessingEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for AudioProcessingEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioProcessingEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioProcessingEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioProcessingEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioProcessingEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioProcessingEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioProcessingEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioProcessingEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioProcessingEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioProcessingEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioProcessingEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioProcessingEvent {
        #[inline]
        fn from(obj: JsValue) -> AudioProcessingEvent {
            AudioProcessingEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioProcessingEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioProcessingEvent> for AudioProcessingEvent {
        #[inline]
        fn as_ref(&self) -> &AudioProcessingEvent {
            self
        }
    }
    impl From<AudioProcessingEvent> for JsValue {
        #[inline]
        fn from(obj: AudioProcessingEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioProcessingEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioProcessingEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioProcessingEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioProcessingEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioProcessingEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioProcessingEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioProcessingEvent> for Event {
    #[inline]
    fn from(obj: AudioProcessingEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for AudioProcessingEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioProcessingEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioProcessingEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioProcessingEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioProcessingEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_playback_time_AudioProcessingEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioProcessingEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioProcessingEvent {
    #[cfg(all(feature = "AudioProcessingEvent",))]
    #[allow(bad_style)]
    #[doc = "The `playbackTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)\n\n*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
    #[allow(clippy::all)]
    pub fn playback_time(&self) -> f64 {
        #[cfg(all(feature = "AudioProcessingEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_playback_time_AudioProcessingEvent(
                self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_playback_time_AudioProcessingEvent(
            self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_playback_time_AudioProcessingEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_input_buffer_AudioProcessingEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioProcessingEvent as WasmDescribe>::describe();
    <AudioBuffer as WasmDescribe>::describe();
}
impl AudioProcessingEvent {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
    #[allow(bad_style)]
    #[doc = "The `inputBuffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    #[allow(clippy::all)]
    pub fn input_buffer(&self) -> Result<AudioBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_input_buffer_AudioProcessingEvent(
                self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_input_buffer_AudioProcessingEvent(
            self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_input_buffer_AudioProcessingEvent(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_output_buffer_AudioProcessingEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioProcessingEvent as WasmDescribe>::describe();
    <AudioBuffer as WasmDescribe>::describe();
}
impl AudioProcessingEvent {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
    #[allow(bad_style)]
    #[doc = "The `outputBuffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
    #[allow(clippy::all)]
    pub fn output_buffer(&self) -> Result<AudioBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioProcessingEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_output_buffer_AudioProcessingEvent(
                self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_output_buffer_AudioProcessingEvent(
            self_: <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioProcessingEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_output_buffer_AudioProcessingEvent(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b83062f207b05662: [u8; 493usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAB\x01\0\0\0\0\x04\0\0\x02\x14AudioProcessingEvent&__widl_instanceof_AudioProcessingEvent\0\0\0\0+__widl_f_playback_time_AudioProcessingEvent\0\0\0\x01\x14AudioProcessingEvent\x01\0\x01\x0CplaybackTime\x01\x01\x05self_\x0CplaybackTime\0\0\0*__widl_f_input_buffer_AudioProcessingEvent\x01\0\0\x01\x14AudioProcessingEvent\x01\0\x01\x0BinputBuffer\x01\x01\x05self_\x0BinputBuffer\0\0\0+__widl_f_output_buffer_AudioProcessingEvent\x01\0\0\x01\x14AudioProcessingEvent\x01\0\x01\x0CoutputBuffer\x01\x01\x05self_\x0CoutputBuffer\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

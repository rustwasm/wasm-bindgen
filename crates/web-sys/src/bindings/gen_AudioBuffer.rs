use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioBuffer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioBuffer {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioBuffer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioBuffer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(66u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for AudioBuffer {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioBuffer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioBuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioBuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioBuffer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioBuffer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioBuffer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioBuffer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioBuffer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioBuffer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioBuffer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioBuffer {
        #[inline]
        fn from(obj: JsValue) -> AudioBuffer {
            AudioBuffer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioBuffer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioBuffer> for AudioBuffer {
        #[inline]
        fn as_ref(&self) -> &AudioBuffer {
            self
        }
    }
    impl From<AudioBuffer> for JsValue {
        #[inline]
        fn from(obj: AudioBuffer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioBuffer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioBuffer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioBuffer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioBuffer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioBuffer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioBuffer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioBuffer> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioBuffer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioBuffer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioBufferOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferOptions as WasmDescribe>::describe();
    <AudioBuffer as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new AudioBuffer(..)` constructor, creating a new instance of `AudioBuffer`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/AudioBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferOptions`*"]
    #[allow(clippy::all)]
    pub fn new(options: &AudioBufferOptions) -> Result<AudioBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AudioBuffer(
                options: <&AudioBufferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AudioBuffer(
            options: <&AudioBufferOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let options =
                    <&AudioBufferOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_AudioBuffer(options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_from_channel_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `copyFromChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn copy_from_channel(
        &self,
        destination: &mut [f32],
        channel_number: i32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_from_channel_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_from_channel_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(channel_number);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let channel_number =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel_number);
                __widl_f_copy_from_channel_AudioBuffer(self_, destination, channel_number)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_from_channel_with_start_in_channel_AudioBuffer()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `copyFromChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn copy_from_channel_with_start_in_channel(
        &self,
        destination: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_from_channel_with_start_in_channel_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_in_channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_from_channel_with_start_in_channel_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_in_channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(channel_number);
            drop(start_in_channel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let channel_number =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel_number);
                let start_in_channel =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_in_channel);
                __widl_f_copy_from_channel_with_start_in_channel_AudioBuffer(
                    self_,
                    destination,
                    channel_number,
                    start_in_channel,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_to_channel_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `copyToChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn copy_to_channel(
        &self,
        source: &mut [f32],
        channel_number: i32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_to_channel_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_to_channel_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(source);
            drop(channel_number);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let source = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let channel_number =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel_number);
                __widl_f_copy_to_channel_AudioBuffer(self_, source, channel_number)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_copy_to_channel_with_start_in_channel_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `copyToChannel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn copy_to_channel_with_start_in_channel(
        &self,
        source: &mut [f32],
        channel_number: i32,
        start_in_channel: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_copy_to_channel_with_start_in_channel_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_in_channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_copy_to_channel_with_start_in_channel_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_number: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_in_channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(source);
            drop(channel_number);
            drop(start_in_channel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let source = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                let channel_number =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel_number);
                let start_in_channel =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_in_channel);
                __widl_f_copy_to_channel_with_start_in_channel_AudioBuffer(
                    self_,
                    source,
                    channel_number,
                    start_in_channel,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_channel_data_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Vec<f32> as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `getChannelData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn get_channel_data(&self, channel: u32) -> Result<Vec<f32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_channel_data_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_channel_data_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(channel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let channel = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel);
                __widl_f_get_channel_data_AudioBuffer(self_, channel)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Vec<f32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_rate_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `sampleRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn sample_rate(&self) -> f32 {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_rate_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_rate_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sample_rate_AudioBuffer(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_AudioBuffer(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_duration_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `duration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn duration(&self) -> f64 {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_duration_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_duration_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_duration_AudioBuffer(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_of_channels_AudioBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBuffer as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AudioBuffer {
    #[cfg(all(feature = "AudioBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `numberOfChannels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)\n\n*This API requires the following crate features to be activated: `AudioBuffer`*"]
    #[allow(clippy::all)]
    pub fn number_of_channels(&self) -> u32 {
        #[cfg(all(feature = "AudioBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_of_channels_AudioBuffer(
                self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_of_channels_AudioBuffer(
            self_: <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_of_channels_AudioBuffer(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_eaf2942cd30f5e17: [u8; 1148usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}:\x04\0\0\0\0\x0B\0\0\x02\x0BAudioBuffer\x1D__widl_instanceof_AudioBuffer\0\0\0\0\x18__widl_f_new_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\0\x01\x01\x07options\x03new\0\0\0&__widl_f_copy_from_channel_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\x01\0\0\x01\x03\x05self_\x0Bdestination\x0Echannel_number\x0FcopyFromChannel\0\0\0<__widl_f_copy_from_channel_with_start_in_channel_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\x01\0\0\x01\x04\x05self_\x0Bdestination\x0Echannel_number\x10start_in_channel\x0FcopyFromChannel\0\0\0$__widl_f_copy_to_channel_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\x01\0\0\x01\x03\x05self_\x06source\x0Echannel_number\rcopyToChannel\0\0\0:__widl_f_copy_to_channel_with_start_in_channel_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\x01\0\0\x01\x04\x05self_\x06source\x0Echannel_number\x10start_in_channel\rcopyToChannel\0\0\0%__widl_f_get_channel_data_AudioBuffer\x01\0\0\x01\x0BAudioBuffer\x01\0\0\x01\x02\x05self_\x07channel\x0EgetChannelData\0\0\0 __widl_f_sample_rate_AudioBuffer\0\0\0\x01\x0BAudioBuffer\x01\0\x01\nsampleRate\x01\x01\x05self_\nsampleRate\0\0\0\x1B__widl_f_length_AudioBuffer\0\0\0\x01\x0BAudioBuffer\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\x1D__widl_f_duration_AudioBuffer\0\0\0\x01\x0BAudioBuffer\x01\0\x01\x08duration\x01\x01\x05self_\x08duration\0\0\0'__widl_f_number_of_channels_AudioBuffer\0\0\0\x01\x0BAudioBuffer\x01\0\x01\x10numberOfChannels\x01\x01\x05self_\x10numberOfChannels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

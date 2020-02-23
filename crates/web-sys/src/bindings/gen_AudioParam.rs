use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioParam` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioParam {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioParam: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioParam {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(80u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
        }
    }
    impl core::ops::Deref for AudioParam {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioParam {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioParam {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioParam {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioParam {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioParam {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioParam {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioParam {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioParam {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioParam>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioParam {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioParam {
        #[inline]
        fn from(obj: JsValue) -> AudioParam {
            AudioParam { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioParam {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioParam> for AudioParam {
        #[inline]
        fn as_ref(&self) -> &AudioParam {
            self
        }
    }
    impl From<AudioParam> for JsValue {
        #[inline]
        fn from(obj: AudioParam) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioParam {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioParam(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioParam(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioParam(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioParam { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioParam) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioParam> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioParam) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioParam {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_scheduled_values_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioParam as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `cancelScheduledValues()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn cancel_scheduled_values(
        &self,
        start_time: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancel_scheduled_values_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancel_scheduled_values_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                __widl_f_cancel_scheduled_values_AudioParam(self_, start_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_exponential_ramp_to_value_at_time_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `exponentialRampToValueAtTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn exponential_ramp_to_value_at_time(
        &self,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_exponential_ramp_to_value_at_time_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_exponential_ramp_to_value_at_time_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(end_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let end_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_time);
                __widl_f_exponential_ramp_to_value_at_time_AudioParam(self_, value, end_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_linear_ramp_to_value_at_time_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `linearRampToValueAtTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn linear_ramp_to_value_at_time(
        &self,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_linear_ramp_to_value_at_time_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_linear_ramp_to_value_at_time_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(end_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let end_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_time);
                __widl_f_linear_ramp_to_value_at_time_AudioParam(self_, value, end_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_at_time_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `setTargetAtTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn set_target_at_time(
        &self,
        target: f32,
        start_time: f64,
        time_constant: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_at_time_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                time_constant: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_at_time_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            time_constant: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(start_time);
            drop(time_constant);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                let time_constant =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(time_constant);
                __widl_f_set_target_at_time_AudioParam(self_, target, start_time, time_constant)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_at_time_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `setValueAtTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn set_value_at_time(
        &self,
        value: f32,
        start_time: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_at_time_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_at_time_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(start_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                __widl_f_set_value_at_time_AudioParam(self_, value, start_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_curve_at_time_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioParam as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `setValueCurveAtTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn set_value_curve_at_time(
        &self,
        values: &mut [f32],
        start_time: f64,
        duration: f64,
    ) -> Result<AudioParam, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_curve_at_time_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                values: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_curve_at_time_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            values: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(values);
            drop(start_time);
            drop(duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let values = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(values);
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                let duration = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                __widl_f_set_value_curve_at_time_AudioParam(self_, values, start_time, duration)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f32 {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_AudioParam(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f32) {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_AudioParam(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_value_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn default_value(&self) -> f32 {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_value_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_value_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_value_AudioParam(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_value_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `minValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn min_value(&self) -> f32 {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_value_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_value_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_value_AudioParam(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_value_AudioParam() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioParam as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioParam {
    #[cfg(all(feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `maxValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)\n\n*This API requires the following crate features to be activated: `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn max_value(&self) -> f32 {
        #[cfg(all(feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_value_AudioParam(
                self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_value_AudioParam(
            self_: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_value_AudioParam(self_)
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
pub static __WASM_BINDGEN_GENERATED_2adf8743c34eacc3: [u8; 1221usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x83\x04\0\0\0\0\x0C\0\0\x02\nAudioParam\x1C__widl_instanceof_AudioParam\0\0\0\0+__widl_f_cancel_scheduled_values_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x02\x05self_\nstart_time\x15cancelScheduledValues\0\0\05__widl_f_exponential_ramp_to_value_at_time_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x03\x05self_\x05value\x08end_time\x1CexponentialRampToValueAtTime\0\0\00__widl_f_linear_ramp_to_value_at_time_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x03\x05self_\x05value\x08end_time\x17linearRampToValueAtTime\0\0\0&__widl_f_set_target_at_time_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x04\x05self_\x06target\nstart_time\rtime_constant\x0FsetTargetAtTime\0\0\0%__widl_f_set_value_at_time_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x03\x05self_\x05value\nstart_time\x0EsetValueAtTime\0\0\0+__widl_f_set_value_curve_at_time_AudioParam\x01\0\0\x01\nAudioParam\x01\0\0\x01\x04\x05self_\x06values\nstart_time\x08duration\x13setValueCurveAtTime\0\0\0\x19__widl_f_value_AudioParam\0\0\0\x01\nAudioParam\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\x1D__widl_f_set_value_AudioParam\0\0\0\x01\nAudioParam\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0!__widl_f_default_value_AudioParam\0\0\0\x01\nAudioParam\x01\0\x01\x0CdefaultValue\x01\x01\x05self_\x0CdefaultValue\0\0\0\x1D__widl_f_min_value_AudioParam\0\0\0\x01\nAudioParam\x01\0\x01\x08minValue\x01\x01\x05self_\x08minValue\0\0\0\x1D__widl_f_max_value_AudioParam\0\0\0\x01\nAudioParam\x01\0\x01\x08maxValue\x01\x01\x05self_\x08maxValue\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

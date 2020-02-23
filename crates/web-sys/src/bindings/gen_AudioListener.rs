use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioListener` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioListener {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioListener: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioListener {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
            inform(101u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for AudioListener {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioListener {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioListener {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioListener {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioListener {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioListener {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioListener {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioListener {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioListener {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioListener>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioListener {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioListener {
        #[inline]
        fn from(obj: JsValue) -> AudioListener {
            AudioListener { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioListener {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioListener> for AudioListener {
        #[inline]
        fn as_ref(&self) -> &AudioListener {
            self
        }
    }
    impl From<AudioListener> for JsValue {
        #[inline]
        fn from(obj: AudioListener) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioListener {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioListener(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioListener(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioListener(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioListener { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioListener) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioListener> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioListener) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioListener {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_orientation_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `setOrientation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn set_orientation(&self, x: f64, y: f64, z: f64, x_up: f64, y_up: f64, z_up: f64) {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_orientation_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_orientation_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z_up: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            drop(x_up);
            drop(y_up);
            drop(z_up);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                let x_up = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x_up);
                let y_up = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y_up);
                let z_up = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z_up);
                __widl_f_set_orientation_AudioListener(self_, x, y, z, x_up, y_up, z_up)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `setPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn set_position(&self, x: f64, y: f64, z: f64) {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
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
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_position_AudioListener(self_, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_velocity_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `setVelocity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setVelocity)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn set_velocity(&self, x: f64, y: f64, z: f64) {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_velocity_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_velocity_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
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
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_velocity_AudioListener(self_, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_doppler_factor_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `dopplerFactor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn doppler_factor(&self) -> f64 {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_doppler_factor_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_doppler_factor_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_doppler_factor_AudioListener(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_doppler_factor_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `dopplerFactor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn set_doppler_factor(&self, doppler_factor: f64) {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_doppler_factor_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                doppler_factor: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_doppler_factor_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            doppler_factor: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(doppler_factor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let doppler_factor =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(doppler_factor);
                __widl_f_set_doppler_factor_AudioListener(self_, doppler_factor)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_speed_of_sound_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `speedOfSound` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn speed_of_sound(&self) -> f64 {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_speed_of_sound_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_speed_of_sound_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_speed_of_sound_AudioListener(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_speed_of_sound_AudioListener() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioListener as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioListener {
    #[cfg(all(feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `speedOfSound` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)\n\n*This API requires the following crate features to be activated: `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn set_speed_of_sound(&self, speed_of_sound: f64) {
        #[cfg(all(feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_speed_of_sound_AudioListener(
                self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                speed_of_sound: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_speed_of_sound_AudioListener(
            self_: <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            speed_of_sound: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(speed_of_sound);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioListener as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let speed_of_sound =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(speed_of_sound);
                __widl_f_set_speed_of_sound_AudioListener(self_, speed_of_sound)
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
pub static __WASM_BINDGEN_GENERATED_f81e6057b0494454: [u8; 860usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1A\x03\0\0\0\0\x08\0\0\x02\rAudioListener\x1F__widl_instanceof_AudioListener\0\0\0\0&__widl_f_set_orientation_AudioListener\0\0\0\x01\rAudioListener\x01\0\0\x01\x07\x05self_\x01x\x01y\x01z\x04x_up\x04y_up\x04z_up\x0EsetOrientation\0\0\0#__widl_f_set_position_AudioListener\0\0\0\x01\rAudioListener\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0BsetPosition\0\0\0#__widl_f_set_velocity_AudioListener\0\0\0\x01\rAudioListener\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0BsetVelocity\0\0\0%__widl_f_doppler_factor_AudioListener\0\0\0\x01\rAudioListener\x01\0\x01\rdopplerFactor\x01\x01\x05self_\rdopplerFactor\0\0\0)__widl_f_set_doppler_factor_AudioListener\0\0\0\x01\rAudioListener\x01\0\x02\rdopplerFactor\x01\x02\x05self_\x0Edoppler_factor\rdopplerFactor\0\0\0%__widl_f_speed_of_sound_AudioListener\0\0\0\x01\rAudioListener\x01\0\x01\x0CspeedOfSound\x01\x01\x05self_\x0CspeedOfSound\0\0\0)__widl_f_set_speed_of_sound_AudioListener\0\0\0\x01\rAudioListener\x01\0\x02\x0CspeedOfSound\x01\x02\x05self_\x0Espeed_of_sound\x0CspeedOfSound\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

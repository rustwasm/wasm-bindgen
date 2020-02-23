use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Navigator` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Navigator {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Navigator: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Navigator {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(78u32);
            inform(97u32);
            inform(118u32);
            inform(105u32);
            inform(103u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for Navigator {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Navigator {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Navigator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Navigator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Navigator {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Navigator {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Navigator {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Navigator {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Navigator {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Navigator>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Navigator {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Navigator {
        #[inline]
        fn from(obj: JsValue) -> Navigator {
            Navigator { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Navigator {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Navigator> for Navigator {
        #[inline]
        fn as_ref(&self) -> &Navigator {
            self
        }
    }
    impl From<Navigator> for JsValue {
        #[inline]
        fn from(obj: Navigator) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Navigator {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Navigator(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Navigator(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Navigator(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Navigator { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Navigator) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Navigator> for ::js_sys::Object {
    #[inline]
    fn from(obj: Navigator) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Navigator {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_gamepads_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `getGamepads()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn get_gamepads(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_gamepads_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_gamepads_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_gamepads_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_vr_displays_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `getVRDisplays()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getVRDisplays)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn get_vr_displays(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_vr_displays_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_vr_displays_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_vr_displays_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_gamepad_service_test_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <GamepadServiceTest as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "GamepadServiceTest", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `requestGamepadServiceTest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestGamepadServiceTest)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn request_gamepad_service_test(&self) -> GamepadServiceTest {
        #[cfg(all(feature = "GamepadServiceTest", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_gamepad_service_test_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadServiceTest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_gamepad_service_test_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadServiceTest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_gamepad_service_test_Navigator(self_)
            };
            <GamepadServiceTest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_midi_access_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `requestMIDIAccess()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn request_midi_access(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_midi_access_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_midi_access_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_midi_access_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MidiOptions", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_midi_access_with_options_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Navigator as WasmDescribe>::describe();
    <&MidiOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "MidiOptions", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `requestMIDIAccess()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)\n\n*This API requires the following crate features to be activated: `MidiOptions`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn request_midi_access_with_options(
        &self,
        options: &MidiOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MidiOptions", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_midi_access_with_options_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&MidiOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_midi_access_with_options_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MidiOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&MidiOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_request_midi_access_with_options_Navigator(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_media_key_system_access_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `requestMediaKeySystemAccess()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn request_media_key_system_access(
        &self,
        key_system: &str,
        supported_configurations: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_media_key_system_access_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_system: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                supported_configurations : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_media_key_system_access_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_system: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            supported_configurations : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_system);
            drop(supported_configurations);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_system = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_system);
                let supported_configurations =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        supported_configurations,
                    );
                __widl_f_request_media_key_system_access_Navigator(
                    self_,
                    key_system,
                    supported_configurations,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "VrServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_vr_service_test_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <VrServiceTest as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "VrServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `requestVRServiceTest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestVRServiceTest)\n\n*This API requires the following crate features to be activated: `Navigator`, `VrServiceTest`*"]
    #[allow(clippy::all)]
    pub fn request_vr_service_test(&self) -> VrServiceTest {
        #[cfg(all(feature = "Navigator", feature = "VrServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_vr_service_test_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VrServiceTest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_vr_service_test_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VrServiceTest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_vr_service_test_Navigator(self_)
            };
            <VrServiceTest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon(&self, url: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_send_beacon_Navigator(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_blob_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&Blob> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Blob", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Blob`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_blob(
        &self,
        url: &str,
        data: Option<&Blob>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_blob_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_blob_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data = <Option<&Blob> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_beacon_with_opt_blob_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_buffer_source_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_buffer_source(
        &self,
        url: &str,
        data: Option<&::js_sys::Object>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_buffer_source_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_buffer_source_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_send_beacon_with_opt_buffer_source_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_u8_array_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&mut [u8]> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_u8_array(
        &self,
        url: &str,
        data: Option<&mut [u8]>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_u8_array_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_u8_array_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data =
                    <Option<&mut [u8]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_beacon_with_opt_u8_array_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FormData", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_form_data_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&FormData> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "FormData", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `FormData`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_form_data(
        &self,
        url: &str,
        data: Option<&FormData>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FormData", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_form_data_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_form_data_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data =
                    <Option<&FormData> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_beacon_with_opt_form_data_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_url_search_params_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&UrlSearchParams> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`, `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_url_search_params(
        &self,
        url: &str,
        data: Option<&UrlSearchParams>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_url_search_params_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_url_search_params_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data =
                    <Option<&UrlSearchParams> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        data,
                    );
                __widl_f_send_beacon_with_opt_url_search_params_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_str_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_str(
        &self,
        url: &str,
        data: Option<&str>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_str_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_str_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data = <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_beacon_with_opt_str_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "ReadableStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_beacon_with_opt_readable_stream_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&ReadableStream> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "ReadableStream",))]
    #[allow(bad_style)]
    #[doc = "The `sendBeacon()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)\n\n*This API requires the following crate features to be activated: `Navigator`, `ReadableStream`*"]
    #[allow(clippy::all)]
    pub fn send_beacon_with_opt_readable_stream(
        &self,
        url: &str,
        data: Option<&ReadableStream>,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "ReadableStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_beacon_with_opt_readable_stream_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_beacon_with_opt_readable_stream_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let data =
                    <Option<&ReadableStream> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_beacon_with_opt_readable_stream_Navigator(self_, url, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vibrate_with_duration_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Navigator as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `vibrate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn vibrate_with_duration(&self, duration: u32) -> bool {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vibrate_with_duration_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vibrate_with_duration_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let duration = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                __widl_f_vibrate_with_duration_Navigator(self_, duration)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vibrate_with_pattern_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Navigator as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `vibrate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn vibrate_with_pattern(&self, pattern: &::wasm_bindgen::JsValue) -> bool {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vibrate_with_pattern_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pattern: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vibrate_with_pattern_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pattern: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(pattern);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pattern =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        pattern,
                    );
                __widl_f_vibrate_with_pattern_Navigator(self_, pattern)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "Permissions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_permissions_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <Permissions as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "Permissions",))]
    #[allow(bad_style)]
    #[doc = "The `permissions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)\n\n*This API requires the following crate features to be activated: `Navigator`, `Permissions`*"]
    #[allow(clippy::all)]
    pub fn permissions(&self) -> Result<Permissions, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "Permissions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_permissions_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Permissions as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_permissions_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Permissions as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_permissions_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Permissions as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MimeTypeArray", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mime_types_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <MimeTypeArray as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "MimeTypeArray", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `mimeTypes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn mime_types(&self) -> Result<MimeTypeArray, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MimeTypeArray", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mime_types_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MimeTypeArray as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mime_types_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MimeTypeArray as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mime_types_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MimeTypeArray as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_plugins_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <PluginArray as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `plugins` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)\n\n*This API requires the following crate features to be activated: `Navigator`, `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn plugins(&self) -> Result<PluginArray, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_plugins_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PluginArray as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_plugins_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PluginArray as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_plugins_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PluginArray as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_do_not_track_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `doNotTrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/doNotTrack)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn do_not_track(&self) -> String {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_do_not_track_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_do_not_track_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_do_not_track_Navigator(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_touch_points_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `maxTouchPoints` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn max_touch_points(&self) -> i32 {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_touch_points_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_touch_points_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_touch_points_Navigator(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaCapabilities", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_capabilities_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <MediaCapabilities as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "MediaCapabilities", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `mediaCapabilities` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)\n\n*This API requires the following crate features to be activated: `MediaCapabilities`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn media_capabilities(&self) -> MediaCapabilities {
        #[cfg(all(feature = "MediaCapabilities", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_capabilities_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_capabilities_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_capabilities_Navigator(self_)
            };
            <MediaCapabilities as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "NetworkInformation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connection_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <NetworkInformation as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "NetworkInformation",))]
    #[allow(bad_style)]
    #[doc = "The `connection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)\n\n*This API requires the following crate features to be activated: `Navigator`, `NetworkInformation`*"]
    #[allow(clippy::all)]
    pub fn connection(&self) -> Result<NetworkInformation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "NetworkInformation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connection_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connection_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_connection_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<NetworkInformation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_vr_displays_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `activeVRDisplays` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/activeVRDisplays)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn active_vr_displays(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_vr_displays_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_vr_displays_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_vr_displays_Navigator(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaDevices", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_devices_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <MediaDevices as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "MediaDevices", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `mediaDevices` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)\n\n*This API requires the following crate features to be activated: `MediaDevices`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn media_devices(&self) -> Result<MediaDevices, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaDevices", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_devices_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaDevices as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_devices_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaDevices as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_devices_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaDevices as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "ServiceWorkerContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_service_worker_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <ServiceWorkerContainer as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "ServiceWorkerContainer",))]
    #[allow(bad_style)]
    #[doc = "The `serviceWorker` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)\n\n*This API requires the following crate features to be activated: `Navigator`, `ServiceWorkerContainer`*"]
    #[allow(clippy::all)]
    pub fn service_worker(&self) -> ServiceWorkerContainer {
        #[cfg(all(feature = "Navigator", feature = "ServiceWorkerContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_service_worker_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ServiceWorkerContainer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_service_worker_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ServiceWorkerContainer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_service_worker_Navigator(self_)
            };
            <ServiceWorkerContainer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "Presentation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_presentation_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <Option<Presentation> as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "Presentation",))]
    #[allow(bad_style)]
    #[doc = "The `presentation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)\n\n*This API requires the following crate features to be activated: `Navigator`, `Presentation`*"]
    #[allow(clippy::all)]
    pub fn presentation(&self) -> Result<Option<Presentation>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator", feature = "Presentation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_presentation_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Presentation> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_presentation_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Presentation> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_presentation_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Presentation> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CredentialsContainer", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_credentials_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <CredentialsContainer as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "CredentialsContainer", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `credentials` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn credentials(&self) -> CredentialsContainer {
        #[cfg(all(feature = "CredentialsContainer", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_credentials_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CredentialsContainer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_credentials_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CredentialsContainer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_credentials_Navigator(self_)
            };
            <CredentialsContainer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hardware_concurrency_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `hardwareConcurrency` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn hardware_concurrency(&self) -> f64 {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hardware_concurrency_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hardware_concurrency_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hardware_concurrency_Navigator(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_content_handler_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `registerContentHandler()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerContentHandler)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn register_content_handler(
        &self,
        mime_type: &str,
        url: &str,
        title: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_content_handler_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mime_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_content_handler_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mime_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mime_type);
            drop(url);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mime_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mime_type);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_register_content_handler_Navigator(self_, mime_type, url, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_protocol_handler_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Navigator as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `registerProtocolHandler()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn register_protocol_handler(
        &self,
        scheme: &str,
        url: &str,
        title: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_protocol_handler_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scheme: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_protocol_handler_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scheme: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scheme);
            drop(url);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scheme = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scheme);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_register_protocol_handler_Navigator(self_, scheme, url, title)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Geolocation", feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_geolocation_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <Geolocation as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Geolocation", feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `geolocation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)\n\n*This API requires the following crate features to be activated: `Geolocation`, `Navigator`*"]
    #[allow(clippy::all)]
    pub fn geolocation(&self) -> Result<Geolocation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Geolocation", feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_geolocation_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Geolocation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_geolocation_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Geolocation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_geolocation_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Geolocation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_taint_enabled_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `taintEnabled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn taint_enabled(&self) -> bool {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_taint_enabled_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_taint_enabled_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_taint_enabled_Navigator(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_code_name_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `appCodeName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appCodeName)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn app_code_name(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_code_name_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_code_name_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_code_name_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_name_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `appName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appName)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn app_name(&self) -> String {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_name_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_name_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_name_Navigator(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_app_version_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `appVersion` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appVersion)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn app_version(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_app_version_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_app_version_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_app_version_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_platform_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `platform` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn platform(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_platform_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_platform_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_platform_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_user_agent_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `userAgent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn user_agent(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_user_agent_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_user_agent_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_user_agent_Navigator(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_product_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `product` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/product)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn product(&self) -> String {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_product_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_product_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_product_Navigator(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `language` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn language(&self) -> Option<String> {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_Navigator(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_languages_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `languages` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn languages(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_languages_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_languages_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_languages_Navigator(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_on_line_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator",))]
    #[allow(bad_style)]
    #[doc = "The `onLine` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)\n\n*This API requires the following crate features to be activated: `Navigator`*"]
    #[allow(clippy::all)]
    pub fn on_line(&self) -> bool {
        #[cfg(all(feature = "Navigator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_on_line_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_on_line_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_on_line_Navigator(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Navigator", feature = "StorageManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_storage_Navigator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Navigator as WasmDescribe>::describe();
    <StorageManager as WasmDescribe>::describe();
}
impl Navigator {
    #[cfg(all(feature = "Navigator", feature = "StorageManager",))]
    #[allow(bad_style)]
    #[doc = "The `storage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)\n\n*This API requires the following crate features to be activated: `Navigator`, `StorageManager`*"]
    #[allow(clippy::all)]
    pub fn storage(&self) -> StorageManager {
        #[cfg(all(feature = "Navigator", feature = "StorageManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_storage_Navigator(
                self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StorageManager as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_storage_Navigator(
            self_: <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StorageManager as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Navigator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_storage_Navigator(self_)
            };
            <StorageManager as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_98a5c976ab6d2f52: [u8; 4042usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x88\x0F\0\0\0\0-\0\0\x02\tNavigator\x1B__widl_instanceof_Navigator\0\0\0\0\x1F__widl_f_get_gamepads_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\x0BgetGamepads\0\0\0\"__widl_f_get_vr_displays_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\rgetVRDisplays\0\0\0/__widl_f_request_gamepad_service_test_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\x19requestGamepadServiceTest\0\0\0&__widl_f_request_midi_access_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\x11requestMIDIAccess\0\0\03__widl_f_request_midi_access_with_options_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x02\x05self_\x07options\x11requestMIDIAccess\0\0\02__widl_f_request_media_key_system_access_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\nkey_system\x18supported_configurations\x1BrequestMediaKeySystemAccess\0\0\0*__widl_f_request_vr_service_test_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\x14requestVRServiceTest\0\0\0\x1E__widl_f_send_beacon_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x02\x05self_\x03url\nsendBeacon\0\0\0,__widl_f_send_beacon_with_opt_blob_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\05__widl_f_send_beacon_with_opt_buffer_source_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\00__widl_f_send_beacon_with_opt_u8_array_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\01__widl_f_send_beacon_with_opt_form_data_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\09__widl_f_send_beacon_with_opt_url_search_params_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\0+__widl_f_send_beacon_with_opt_str_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\07__widl_f_send_beacon_with_opt_readable_stream_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x03\x05self_\x03url\x04data\nsendBeacon\0\0\0(__widl_f_vibrate_with_duration_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x02\x05self_\x08duration\x07vibrate\0\0\0'__widl_f_vibrate_with_pattern_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x02\x05self_\x07pattern\x07vibrate\0\0\0\x1E__widl_f_permissions_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x0Bpermissions\x01\x01\x05self_\x0Bpermissions\0\0\0\x1D__widl_f_mime_types_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\tmimeTypes\x01\x01\x05self_\tmimeTypes\0\0\0\x1A__widl_f_plugins_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x07plugins\x01\x01\x05self_\x07plugins\0\0\0\x1F__widl_f_do_not_track_Navigator\0\0\0\x01\tNavigator\x01\0\x01\ndoNotTrack\x01\x01\x05self_\ndoNotTrack\0\0\0#__widl_f_max_touch_points_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x0EmaxTouchPoints\x01\x01\x05self_\x0EmaxTouchPoints\0\0\0%__widl_f_media_capabilities_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x11mediaCapabilities\x01\x01\x05self_\x11mediaCapabilities\0\0\0\x1D__widl_f_connection_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\nconnection\x01\x01\x05self_\nconnection\0\0\0%__widl_f_active_vr_displays_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x10activeVRDisplays\x01\x01\x05self_\x10activeVRDisplays\0\0\0 __widl_f_media_devices_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x0CmediaDevices\x01\x01\x05self_\x0CmediaDevices\0\0\0!__widl_f_service_worker_Navigator\0\0\0\x01\tNavigator\x01\0\x01\rserviceWorker\x01\x01\x05self_\rserviceWorker\0\0\0\x1F__widl_f_presentation_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x0Cpresentation\x01\x01\x05self_\x0Cpresentation\0\0\0\x1E__widl_f_credentials_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x0Bcredentials\x01\x01\x05self_\x0Bcredentials\0\0\0'__widl_f_hardware_concurrency_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x13hardwareConcurrency\x01\x01\x05self_\x13hardwareConcurrency\0\0\0+__widl_f_register_content_handler_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x04\x05self_\tmime_type\x03url\x05title\x16registerContentHandler\0\0\0,__widl_f_register_protocol_handler_Navigator\x01\0\0\x01\tNavigator\x01\0\0\x01\x04\x05self_\x06scheme\x03url\x05title\x17registerProtocolHandler\0\0\0\x1E__widl_f_geolocation_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x0Bgeolocation\x01\x01\x05self_\x0Bgeolocation\0\0\0 __widl_f_taint_enabled_Navigator\0\0\0\x01\tNavigator\x01\0\0\x01\x01\x05self_\x0CtaintEnabled\0\0\0 __widl_f_app_code_name_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x0BappCodeName\x01\x01\x05self_\x0BappCodeName\0\0\0\x1B__widl_f_app_name_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x07appName\x01\x01\x05self_\x07appName\0\0\0\x1E__widl_f_app_version_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\nappVersion\x01\x01\x05self_\nappVersion\0\0\0\x1B__widl_f_platform_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\x08platform\x01\x01\x05self_\x08platform\0\0\0\x1D__widl_f_user_agent_Navigator\x01\0\0\x01\tNavigator\x01\0\x01\tuserAgent\x01\x01\x05self_\tuserAgent\0\0\0\x1A__widl_f_product_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x07product\x01\x01\x05self_\x07product\0\0\0\x1B__widl_f_language_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x08language\x01\x01\x05self_\x08language\0\0\0\x1C__widl_f_languages_Navigator\0\0\0\x01\tNavigator\x01\0\x01\tlanguages\x01\x01\x05self_\tlanguages\0\0\0\x1A__widl_f_on_line_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x06onLine\x01\x01\x05self_\x06onLine\0\0\0\x1A__widl_f_storage_Navigator\0\0\0\x01\tNavigator\x01\0\x01\x07storage\x01\x01\x05self_\x07storage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadHapticActuator` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadHapticActuator {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadHapticActuator: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadHapticActuator {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(72u32);
            inform(97u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
            inform(99u32);
            inform(65u32);
            inform(99u32);
            inform(116u32);
            inform(117u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for GamepadHapticActuator {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadHapticActuator {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadHapticActuator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadHapticActuator {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadHapticActuator {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadHapticActuator {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadHapticActuator {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadHapticActuator {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadHapticActuator {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadHapticActuator>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadHapticActuator {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadHapticActuator {
        #[inline]
        fn from(obj: JsValue) -> GamepadHapticActuator {
            GamepadHapticActuator { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadHapticActuator {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadHapticActuator> for GamepadHapticActuator {
        #[inline]
        fn as_ref(&self) -> &GamepadHapticActuator {
            self
        }
    }
    impl From<GamepadHapticActuator> for JsValue {
        #[inline]
        fn from(obj: GamepadHapticActuator) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadHapticActuator {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadHapticActuator(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadHapticActuator(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadHapticActuator(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadHapticActuator { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadHapticActuator) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadHapticActuator> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadHapticActuator) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadHapticActuator {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadHapticActuator",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pulse_GamepadHapticActuator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&GamepadHapticActuator as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl GamepadHapticActuator {
    #[cfg(all(feature = "GamepadHapticActuator",))]
    #[allow(bad_style)]
    #[doc = "The `pulse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[allow(clippy::all)]
    pub fn pulse(
        &self,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadHapticActuator",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pulse_GamepadHapticActuator(
                self_: <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pulse_GamepadHapticActuator(
            self_: <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            drop(duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let duration = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                __widl_f_pulse_GamepadHapticActuator(self_, value, duration)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "GamepadHapticActuator",
    feature = "GamepadHapticActuatorType",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_GamepadHapticActuator() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadHapticActuator as WasmDescribe>::describe();
    <GamepadHapticActuatorType as WasmDescribe>::describe();
}
impl GamepadHapticActuator {
    #[cfg(all(
        feature = "GamepadHapticActuator",
        feature = "GamepadHapticActuatorType",
    ))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)\n\n*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> GamepadHapticActuatorType {
        #[cfg(all(
            feature = "GamepadHapticActuator",
            feature = "GamepadHapticActuatorType",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_GamepadHapticActuator(
                self_: <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadHapticActuatorType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_GamepadHapticActuator(
            self_: <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadHapticActuatorType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadHapticActuator as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_GamepadHapticActuator(self_)
            };
            <GamepadHapticActuatorType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d2efdf2dc8c941f5: [u8; 355usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}!\x01\0\0\0\0\x03\0\0\x02\x15GamepadHapticActuator'__widl_instanceof_GamepadHapticActuator\0\0\0\0$__widl_f_pulse_GamepadHapticActuator\x01\0\0\x01\x15GamepadHapticActuator\x01\0\0\x01\x03\x05self_\x05value\x08duration\x05pulse\0\0\0#__widl_f_type_GamepadHapticActuator\0\0\0\x01\x15GamepadHapticActuator\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadButton` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadButton {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadButton: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadButton {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(66u32);
            inform(117u32);
            inform(116u32);
            inform(116u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for GamepadButton {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadButton {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadButton {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadButton {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadButton {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadButton {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadButton {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadButton {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadButton {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadButton>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadButton {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadButton {
        #[inline]
        fn from(obj: JsValue) -> GamepadButton {
            GamepadButton { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadButton {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadButton> for GamepadButton {
        #[inline]
        fn as_ref(&self) -> &GamepadButton {
            self
        }
    }
    impl From<GamepadButton> for JsValue {
        #[inline]
        fn from(obj: GamepadButton) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadButton {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadButton(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadButton(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadButton(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadButton { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadButton) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadButton> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadButton) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadButton {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadButton",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pressed_GamepadButton() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadButton as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GamepadButton {
    #[cfg(all(feature = "GamepadButton",))]
    #[allow(bad_style)]
    #[doc = "The `pressed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    #[allow(clippy::all)]
    pub fn pressed(&self) -> bool {
        #[cfg(all(feature = "GamepadButton",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pressed_GamepadButton(
                self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pressed_GamepadButton(
            self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pressed_GamepadButton(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadButton",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_touched_GamepadButton() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadButton as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl GamepadButton {
    #[cfg(all(feature = "GamepadButton",))]
    #[allow(bad_style)]
    #[doc = "The `touched` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/touched)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    #[allow(clippy::all)]
    pub fn touched(&self) -> bool {
        #[cfg(all(feature = "GamepadButton",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_touched_GamepadButton(
                self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_touched_GamepadButton(
            self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_touched_GamepadButton(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadButton",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_GamepadButton() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadButton as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl GamepadButton {
    #[cfg(all(feature = "GamepadButton",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)\n\n*This API requires the following crate features to be activated: `GamepadButton`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "GamepadButton",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_GamepadButton(
                self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_GamepadButton(
            self_: <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadButton as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_GamepadButton(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_31bcb998f5c79a6a: [u8; 386usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}@\x01\0\0\0\0\x04\0\0\x02\rGamepadButton\x1F__widl_instanceof_GamepadButton\0\0\0\0\x1E__widl_f_pressed_GamepadButton\0\0\0\x01\rGamepadButton\x01\0\x01\x07pressed\x01\x01\x05self_\x07pressed\0\0\0\x1E__widl_f_touched_GamepadButton\0\0\0\x01\rGamepadButton\x01\0\x01\x07touched\x01\x01\x05self_\x07touched\0\0\0\x1C__widl_f_value_GamepadButton\0\0\0\x01\rGamepadButton\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

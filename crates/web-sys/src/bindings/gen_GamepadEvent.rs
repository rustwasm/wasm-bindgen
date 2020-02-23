use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for GamepadEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadEvent {
        #[inline]
        fn from(obj: JsValue) -> GamepadEvent {
            GamepadEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadEvent> for GamepadEvent {
        #[inline]
        fn as_ref(&self) -> &GamepadEvent {
            self
        }
    }
    impl From<GamepadEvent> for JsValue {
        #[inline]
        fn from(obj: GamepadEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadEvent> for Event {
    #[inline]
    fn from(obj: GamepadEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for GamepadEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GamepadEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_GamepadEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <GamepadEvent as WasmDescribe>::describe();
}
impl GamepadEvent {
    #[cfg(all(feature = "GamepadEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<GamepadEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_GamepadEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_GamepadEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_GamepadEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadEvent", feature = "GamepadEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_GamepadEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&GamepadEventInit as WasmDescribe>::describe();
    <GamepadEvent as WasmDescribe>::describe();
}
impl GamepadEvent {
    #[cfg(all(feature = "GamepadEvent", feature = "GamepadEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)\n\n*This API requires the following crate features to be activated: `GamepadEvent`, `GamepadEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadEventInit,
    ) -> Result<GamepadEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadEvent", feature = "GamepadEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_GamepadEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&GamepadEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_GamepadEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&GamepadEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            drop(event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let event_init_dict =
                    <&GamepadEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_GamepadEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Gamepad", feature = "GamepadEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_gamepad_GamepadEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadEvent as WasmDescribe>::describe();
    <Option<Gamepad> as WasmDescribe>::describe();
}
impl GamepadEvent {
    #[cfg(all(feature = "Gamepad", feature = "GamepadEvent",))]
    #[allow(bad_style)]
    #[doc = "The `gamepad` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)\n\n*This API requires the following crate features to be activated: `Gamepad`, `GamepadEvent`*"]
    #[allow(clippy::all)]
    pub fn gamepad(&self) -> Option<Gamepad> {
        #[cfg(all(feature = "Gamepad", feature = "GamepadEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_gamepad_GamepadEvent(
                self_: <&GamepadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Gamepad> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_gamepad_GamepadEvent(
            self_: <&GamepadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Gamepad> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GamepadEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_gamepad_GamepadEvent(self_)
            };
            <Option<Gamepad> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8ce481b11e3330d1: [u8; 385usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}?\x01\0\0\0\0\x04\0\0\x02\x0CGamepadEvent\x1E__widl_instanceof_GamepadEvent\0\0\0\0\x19__widl_f_new_GamepadEvent\x01\0\0\x01\x0CGamepadEvent\0\x01\x01\x05type_\x03new\0\0\0.__widl_f_new_with_event_init_dict_GamepadEvent\x01\0\0\x01\x0CGamepadEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1D__widl_f_gamepad_GamepadEvent\0\0\0\x01\x0CGamepadEvent\x01\0\x01\x07gamepad\x01\x01\x05self_\x07gamepad\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

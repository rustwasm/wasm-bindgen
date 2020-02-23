use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadButtonEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent)\n\n*This API requires the following crate features to be activated: `GamepadButtonEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadButtonEvent {
    obj: GamepadEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadButtonEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadButtonEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
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
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for GamepadButtonEvent {
        type Target = GamepadEvent;
        #[inline]
        fn deref(&self) -> &GamepadEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadButtonEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadButtonEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadButtonEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadButtonEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadButtonEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadButtonEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadButtonEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadButtonEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadButtonEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadButtonEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadButtonEvent {
        #[inline]
        fn from(obj: JsValue) -> GamepadButtonEvent {
            GamepadButtonEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadButtonEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadButtonEvent> for GamepadButtonEvent {
        #[inline]
        fn as_ref(&self) -> &GamepadButtonEvent {
            self
        }
    }
    impl From<GamepadButtonEvent> for JsValue {
        #[inline]
        fn from(obj: GamepadButtonEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadButtonEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadButtonEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadButtonEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadButtonEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadButtonEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadButtonEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadButtonEvent> for GamepadEvent {
    #[inline]
    fn from(obj: GamepadButtonEvent) -> GamepadEvent {
        use wasm_bindgen::JsCast;
        GamepadEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<GamepadEvent> for GamepadButtonEvent {
    #[inline]
    fn as_ref(&self) -> &GamepadEvent {
        use wasm_bindgen::JsCast;
        GamepadEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GamepadButtonEvent> for Event {
    #[inline]
    fn from(obj: GamepadButtonEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for GamepadButtonEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GamepadButtonEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadButtonEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadButtonEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadButtonEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_GamepadButtonEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <GamepadButtonEvent as WasmDescribe>::describe();
}
impl GamepadButtonEvent {
    #[cfg(all(feature = "GamepadButtonEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadButtonEvent(..)` constructor, creating a new instance of `GamepadButtonEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/GamepadButtonEvent)\n\n*This API requires the following crate features to be activated: `GamepadButtonEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<GamepadButtonEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadButtonEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_GamepadButtonEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_GamepadButtonEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_GamepadButtonEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadButtonEvent", feature = "GamepadButtonEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_GamepadButtonEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&GamepadButtonEventInit as WasmDescribe>::describe();
    <GamepadButtonEvent as WasmDescribe>::describe();
}
impl GamepadButtonEvent {
    #[cfg(all(feature = "GamepadButtonEvent", feature = "GamepadButtonEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadButtonEvent(..)` constructor, creating a new instance of `GamepadButtonEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/GamepadButtonEvent)\n\n*This API requires the following crate features to be activated: `GamepadButtonEvent`, `GamepadButtonEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadButtonEventInit,
    ) -> Result<GamepadButtonEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadButtonEvent", feature = "GamepadButtonEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_GamepadButtonEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & GamepadButtonEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_GamepadButtonEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&GamepadButtonEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&GamepadButtonEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_GamepadButtonEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadButtonEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadButtonEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_button_GamepadButtonEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadButtonEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl GamepadButtonEvent {
    #[cfg(all(feature = "GamepadButtonEvent",))]
    #[allow(bad_style)]
    #[doc = "The `button` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButtonEvent/button)\n\n*This API requires the following crate features to be activated: `GamepadButtonEvent`*"]
    #[allow(clippy::all)]
    pub fn button(&self) -> u32 {
        #[cfg(all(feature = "GamepadButtonEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_button_GamepadButtonEvent(
                self_: <&GamepadButtonEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_button_GamepadButtonEvent(
            self_: <&GamepadButtonEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadButtonEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_button_GamepadButtonEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_2df889b587aae327: [u8; 430usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}l\x01\0\0\0\0\x04\0\0\x02\x12GamepadButtonEvent$__widl_instanceof_GamepadButtonEvent\0\0\0\0\x1F__widl_f_new_GamepadButtonEvent\x01\0\0\x01\x12GamepadButtonEvent\0\x01\x01\x05type_\x03new\0\0\04__widl_f_new_with_event_init_dict_GamepadButtonEvent\x01\0\0\x01\x12GamepadButtonEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\"__widl_f_button_GamepadButtonEvent\0\0\0\x01\x12GamepadButtonEvent\x01\0\x01\x06button\x01\x01\x05self_\x06button\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

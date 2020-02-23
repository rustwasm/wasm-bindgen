use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadAxisMoveEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadAxisMoveEvent {
    obj: GamepadEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadAxisMoveEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadAxisMoveEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(65u32);
            inform(120u32);
            inform(105u32);
            inform(115u32);
            inform(77u32);
            inform(111u32);
            inform(118u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for GamepadAxisMoveEvent {
        type Target = GamepadEvent;
        #[inline]
        fn deref(&self) -> &GamepadEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadAxisMoveEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadAxisMoveEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadAxisMoveEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadAxisMoveEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadAxisMoveEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadAxisMoveEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadAxisMoveEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadAxisMoveEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadAxisMoveEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadAxisMoveEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadAxisMoveEvent {
        #[inline]
        fn from(obj: JsValue) -> GamepadAxisMoveEvent {
            GamepadAxisMoveEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadAxisMoveEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadAxisMoveEvent> for GamepadAxisMoveEvent {
        #[inline]
        fn as_ref(&self) -> &GamepadAxisMoveEvent {
            self
        }
    }
    impl From<GamepadAxisMoveEvent> for JsValue {
        #[inline]
        fn from(obj: GamepadAxisMoveEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadAxisMoveEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadAxisMoveEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadAxisMoveEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadAxisMoveEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadAxisMoveEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadAxisMoveEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadAxisMoveEvent> for GamepadEvent {
    #[inline]
    fn from(obj: GamepadAxisMoveEvent) -> GamepadEvent {
        use wasm_bindgen::JsCast;
        GamepadEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<GamepadEvent> for GamepadAxisMoveEvent {
    #[inline]
    fn as_ref(&self) -> &GamepadEvent {
        use wasm_bindgen::JsCast;
        GamepadEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GamepadAxisMoveEvent> for Event {
    #[inline]
    fn from(obj: GamepadAxisMoveEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for GamepadAxisMoveEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GamepadAxisMoveEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadAxisMoveEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadAxisMoveEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "GamepadAxisMoveEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_GamepadAxisMoveEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <GamepadAxisMoveEvent as WasmDescribe>::describe();
}
impl GamepadAxisMoveEvent {
    #[cfg(all(feature = "GamepadAxisMoveEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadAxisMoveEvent(..)` constructor, creating a new instance of `GamepadAxisMoveEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/GamepadAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<GamepadAxisMoveEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadAxisMoveEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_GamepadAxisMoveEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_GamepadAxisMoveEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_GamepadAxisMoveEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadAxisMoveEvent", feature = "GamepadAxisMoveEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_GamepadAxisMoveEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&GamepadAxisMoveEventInit as WasmDescribe>::describe();
    <GamepadAxisMoveEvent as WasmDescribe>::describe();
}
impl GamepadAxisMoveEvent {
    #[cfg(all(feature = "GamepadAxisMoveEvent", feature = "GamepadAxisMoveEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new GamepadAxisMoveEvent(..)` constructor, creating a new instance of `GamepadAxisMoveEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/GamepadAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`, `GamepadAxisMoveEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &GamepadAxisMoveEventInit,
    ) -> Result<GamepadAxisMoveEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "GamepadAxisMoveEvent", feature = "GamepadAxisMoveEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_GamepadAxisMoveEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & GamepadAxisMoveEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_GamepadAxisMoveEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&GamepadAxisMoveEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&GamepadAxisMoveEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_GamepadAxisMoveEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GamepadAxisMoveEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadAxisMoveEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_axis_GamepadAxisMoveEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadAxisMoveEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl GamepadAxisMoveEvent {
    #[cfg(all(feature = "GamepadAxisMoveEvent",))]
    #[allow(bad_style)]
    #[doc = "The `axis` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/axis)\n\n*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*"]
    #[allow(clippy::all)]
    pub fn axis(&self) -> u32 {
        #[cfg(all(feature = "GamepadAxisMoveEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_axis_GamepadAxisMoveEvent(
                self_: <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_axis_GamepadAxisMoveEvent(
            self_: <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_axis_GamepadAxisMoveEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadAxisMoveEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_GamepadAxisMoveEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadAxisMoveEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl GamepadAxisMoveEvent {
    #[cfg(all(feature = "GamepadAxisMoveEvent",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadAxisMoveEvent/value)\n\n*This API requires the following crate features to be activated: `GamepadAxisMoveEvent`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "GamepadAxisMoveEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_GamepadAxisMoveEvent(
                self_: <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_GamepadAxisMoveEvent(
            self_: <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&GamepadAxisMoveEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_GamepadAxisMoveEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_7d0c35cbd352e2a8: [u8; 527usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCD\x01\0\0\0\0\x05\0\0\x02\x14GamepadAxisMoveEvent&__widl_instanceof_GamepadAxisMoveEvent\0\0\0\0!__widl_f_new_GamepadAxisMoveEvent\x01\0\0\x01\x14GamepadAxisMoveEvent\0\x01\x01\x05type_\x03new\0\0\06__widl_f_new_with_event_init_dict_GamepadAxisMoveEvent\x01\0\0\x01\x14GamepadAxisMoveEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\"__widl_f_axis_GamepadAxisMoveEvent\0\0\0\x01\x14GamepadAxisMoveEvent\x01\0\x01\x04axis\x01\x01\x05self_\x04axis\0\0\0#__widl_f_value_GamepadAxisMoveEvent\0\0\0\x01\x14GamepadAxisMoveEvent\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

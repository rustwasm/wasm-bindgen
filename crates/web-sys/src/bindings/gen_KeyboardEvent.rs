use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `KeyboardEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct KeyboardEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_KeyboardEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for KeyboardEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(98u32);
            inform(111u32);
            inform(97u32);
            inform(114u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for KeyboardEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for KeyboardEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for KeyboardEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a KeyboardEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for KeyboardEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            KeyboardEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for KeyboardEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a KeyboardEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for KeyboardEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<KeyboardEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(KeyboardEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for KeyboardEvent {
        #[inline]
        fn from(obj: JsValue) -> KeyboardEvent {
            KeyboardEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for KeyboardEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<KeyboardEvent> for KeyboardEvent {
        #[inline]
        fn as_ref(&self) -> &KeyboardEvent {
            self
        }
    }
    impl From<KeyboardEvent> for JsValue {
        #[inline]
        fn from(obj: KeyboardEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for KeyboardEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_KeyboardEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_KeyboardEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_KeyboardEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            KeyboardEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const KeyboardEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<KeyboardEvent> for UiEvent {
    #[inline]
    fn from(obj: KeyboardEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for KeyboardEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<KeyboardEvent> for Event {
    #[inline]
    fn from(obj: KeyboardEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for KeyboardEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<KeyboardEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: KeyboardEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for KeyboardEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <KeyboardEvent as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_arg: &str) -> Result<KeyboardEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_KeyboardEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_KeyboardEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_new_KeyboardEvent(type_arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "KeyboardEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_keyboard_event_init_dict_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&KeyboardEventInit as WasmDescribe>::describe();
    <KeyboardEvent as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "KeyboardEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `KeyboardEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_keyboard_event_init_dict(
        type_arg: &str,
        keyboard_event_init_dict: &KeyboardEventInit,
    ) -> Result<KeyboardEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "KeyboardEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_keyboard_event_init_dict_KeyboardEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyboard_event_init_dict : < & KeyboardEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_keyboard_event_init_dict_KeyboardEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyboard_event_init_dict : < & KeyboardEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            drop(keyboard_event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let keyboard_event_init_dict =
                    <&KeyboardEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyboard_event_init_dict,
                    );
                __widl_f_new_with_keyboard_event_init_dict_KeyboardEvent(
                    type_arg,
                    keyboard_event_init_dict,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyboardEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_modifier_state_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `getModifierState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn get_modifier_state(&self, key: &str) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_modifier_state_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_modifier_state_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_get_modifier_state_KeyboardEvent(self_, key)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event(&self, type_arg: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_init_keyboard_event_KeyboardEvent(self_, type_arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_KeyboardEvent()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                __widl_f_init_keyboard_event_with_bubbles_arg_KeyboardEvent(
                    self_,
                    type_arg,
                    bubbles_arg,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_KeyboardEvent(
                    self_,
                    type_arg,
                    bubbles_arg,
                    cancelable_arg,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            drop(location_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                let location_arg =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location_arg);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg , location_arg )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            drop(location_arg);
            drop(ctrl_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                let location_arg =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location_arg);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg , location_arg , ctrl_key )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            drop(location_arg);
            drop(ctrl_key);
            drop(alt_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                let location_arg =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location_arg);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg , location_arg , ctrl_key , alt_key )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            drop(location_arg);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                let location_arg =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location_arg);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg , location_arg , ctrl_key , alt_key , shift_key )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyboardEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initKeyboardEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        key_arg: &str,
        location_arg: u32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyboardEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location_arg: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(bubbles_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(key_arg);
            drop(location_arg);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let bubbles_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                let location_arg =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location_arg);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                __widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyboardEvent ( self_ , type_arg , bubbles_arg , cancelable_arg , view_arg , key_arg , location_arg , ctrl_key , alt_key , shift_key , meta_key )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_char_code_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `charCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/charCode)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn char_code(&self) -> u32 {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_char_code_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_char_code_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_char_code_KeyboardEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_code_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `keyCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn key_code(&self) -> u32 {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_code_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_code_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_code_KeyboardEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_key_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `altKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn alt_key(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_key_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ctrl_key_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `ctrlKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn ctrl_key(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ctrl_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ctrl_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ctrl_key_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shift_key_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `shiftKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn shift_key(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shift_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shift_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shift_key_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_meta_key_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `metaKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn meta_key(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_meta_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_meta_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_meta_key_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_location_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `location` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn location(&self) -> u32 {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_location_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_location_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_location_KeyboardEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_repeat_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `repeat` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn repeat(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_repeat_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_repeat_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_repeat_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_composing_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `isComposing` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn is_composing(&self) -> bool {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_composing_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_composing_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_composing_KeyboardEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `key` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn key(&self) -> String {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_KeyboardEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyboardEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_KeyboardEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyboardEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl KeyboardEvent {
    #[cfg(all(feature = "KeyboardEvent",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)\n\n*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> String {
        #[cfg(all(feature = "KeyboardEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_KeyboardEvent(
                self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_KeyboardEvent(
            self_: <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyboardEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_KeyboardEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl KeyboardEvent {
    pub const DOM_KEY_LOCATION_STANDARD: u32 = 0u64 as u32;
}
impl KeyboardEvent {
    pub const DOM_KEY_LOCATION_LEFT: u32 = 1u64 as u32;
}
impl KeyboardEvent {
    pub const DOM_KEY_LOCATION_RIGHT: u32 = 2u64 as u32;
}
impl KeyboardEvent {
    pub const DOM_KEY_LOCATION_NUMPAD: u32 = 3u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b2fb2536b8a9f6e5: [u8; 3502usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}l\r\0\0\0\0\x19\0\0\x02\rKeyboardEvent\x1F__widl_instanceof_KeyboardEvent\0\0\0\0\x1A__widl_f_new_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\0\x01\x01\x08type_arg\x03new\0\0\08__widl_f_new_with_keyboard_event_init_dict_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\0\x01\x02\x08type_arg\x18keyboard_event_init_dict\x03new\0\0\0)__widl_f_get_modifier_state_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\0\x01\x02\x05self_\x03key\x10getModifierState\0\0\0*__widl_f_init_keyboard_event_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x02\x05self_\x08type_arg\x11initKeyboardEvent\0\0\0;__widl_f_init_keyboard_event_with_bubbles_arg_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x03\x05self_\x08type_arg\x0Bbubbles_arg\x11initKeyboardEvent\0\0\0N__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x04\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x11initKeyboardEvent\0\0\0[__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x05\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x11initKeyboardEvent\0\0\0g__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x06\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x11initKeyboardEvent\0\0\0x__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x07\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x0Clocation_arg\x11initKeyboardEvent\0\0\0\x85\x01__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x08\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x0Clocation_arg\x08ctrl_key\x11initKeyboardEvent\0\0\0\x91\x01__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\t\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x0Clocation_arg\x08ctrl_key\x07alt_key\x11initKeyboardEvent\0\0\0\x9F\x01__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\n\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x0Clocation_arg\x08ctrl_key\x07alt_key\tshift_key\x11initKeyboardEvent\0\0\0\xAC\x01__widl_f_init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_KeyboardEvent\x01\0\0\x01\rKeyboardEvent\x01\0\0\x01\x0B\x05self_\x08type_arg\x0Bbubbles_arg\x0Ecancelable_arg\x08view_arg\x07key_arg\x0Clocation_arg\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x11initKeyboardEvent\0\0\0 __widl_f_char_code_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x08charCode\x01\x01\x05self_\x08charCode\0\0\0\x1F__widl_f_key_code_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x07keyCode\x01\x01\x05self_\x07keyCode\0\0\0\x1E__widl_f_alt_key_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x06altKey\x01\x01\x05self_\x06altKey\0\0\0\x1F__widl_f_ctrl_key_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x07ctrlKey\x01\x01\x05self_\x07ctrlKey\0\0\0 __widl_f_shift_key_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x08shiftKey\x01\x01\x05self_\x08shiftKey\0\0\0\x1F__widl_f_meta_key_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x07metaKey\x01\x01\x05self_\x07metaKey\0\0\0\x1F__widl_f_location_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x08location\x01\x01\x05self_\x08location\0\0\0\x1D__widl_f_repeat_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x06repeat\x01\x01\x05self_\x06repeat\0\0\0#__widl_f_is_composing_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x0BisComposing\x01\x01\x05self_\x0BisComposing\0\0\0\x1A__widl_f_key_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x03key\x01\x01\x05self_\x03key\0\0\0\x1B__widl_f_code_KeyboardEvent\0\0\0\x01\rKeyboardEvent\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

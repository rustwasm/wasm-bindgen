use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MouseEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MouseEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MouseEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MouseEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(77u32);
            inform(111u32);
            inform(117u32);
            inform(115u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MouseEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for MouseEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MouseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MouseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MouseEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MouseEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MouseEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MouseEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MouseEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MouseEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MouseEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MouseEvent {
        #[inline]
        fn from(obj: JsValue) -> MouseEvent {
            MouseEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MouseEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MouseEvent> for MouseEvent {
        #[inline]
        fn as_ref(&self) -> &MouseEvent {
            self
        }
    }
    impl From<MouseEvent> for JsValue {
        #[inline]
        fn from(obj: MouseEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MouseEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MouseEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MouseEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MouseEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MouseEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MouseEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MouseEvent> for UiEvent {
    #[inline]
    fn from(obj: MouseEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for MouseEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MouseEvent> for Event {
    #[inline]
    fn from(obj: MouseEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MouseEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MouseEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MouseEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MouseEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <MouseEvent as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_arg: &str) -> Result<MouseEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MouseEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MouseEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_new_MouseEvent(type_arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MouseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "MouseEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_mouse_event_init_dict_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&MouseEventInit as WasmDescribe>::describe();
    <MouseEvent as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "MouseEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `MouseEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_mouse_event_init_dict(
        type_arg: &str,
        mouse_event_init_dict: &MouseEventInit,
    ) -> Result<MouseEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MouseEvent", feature = "MouseEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_mouse_event_init_dict_MouseEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mouse_event_init_dict: <&MouseEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_mouse_event_init_dict_MouseEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mouse_event_init_dict: <&MouseEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MouseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            drop(mouse_event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let mouse_event_init_dict =
                    <&MouseEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        mouse_event_init_dict,
                    );
                __widl_f_new_with_mouse_event_init_dict_MouseEvent(type_arg, mouse_event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MouseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_modifier_state_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `getModifierState()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn get_modifier_state(&self, key_arg: &str) -> bool {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_modifier_state_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_modifier_state_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_arg);
                __widl_f_get_modifier_state_MouseEvent(self_, key_arg)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event(&self, type_arg: &str) {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_init_mouse_event_MouseEvent(self_, type_arg)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_MouseEvent(
                    self_,
                    type_arg,
                    can_bubble_arg,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    ) {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_MouseEvent(
                    self_,
                    type_arg,
                    can_bubble_arg,
                    cancelable_arg,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(12u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            drop(alt_key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                let alt_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg , alt_key_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(13u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            drop(alt_key_arg);
            drop(shift_key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                let alt_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key_arg);
                let shift_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg , alt_key_arg , shift_key_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(14u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            drop(alt_key_arg);
            drop(shift_key_arg);
            drop(meta_key_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                let alt_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key_arg);
                let shift_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key_arg);
                let meta_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg , alt_key_arg , shift_key_arg , meta_key_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(15u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
    ) {
        #[cfg(all(feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button_arg: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button_arg: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            drop(alt_key_arg);
            drop(shift_key_arg);
            drop(meta_key_arg);
            drop(button_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                let alt_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key_arg);
                let shift_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key_arg);
                let meta_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key_arg);
                let button_arg = <i16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button_arg);
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg , alt_key_arg , shift_key_arg , meta_key_arg , button_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "EventTarget", feature = "MouseEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg_MouseEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(16u32);
    <&MouseEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
    <Option<&EventTarget> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "EventTarget", feature = "MouseEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg(
        &self,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        view_arg: Option<&Window>,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
        related_target_arg: Option<&EventTarget>,
    ) {
        #[cfg(all(feature = "EventTarget", feature = "MouseEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button_arg: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_target_arg : < Option < & EventTarget > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view_arg: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y_arg: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key_arg: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button_arg: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_target_arg: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_arg);
            drop(can_bubble_arg);
            drop(cancelable_arg);
            drop(view_arg);
            drop(detail_arg);
            drop(screen_x_arg);
            drop(screen_y_arg);
            drop(client_x_arg);
            drop(client_y_arg);
            drop(ctrl_key_arg);
            drop(alt_key_arg);
            drop(shift_key_arg);
            drop(meta_key_arg);
            drop(button_arg);
            drop(related_target_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let can_bubble_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble_arg);
                let cancelable_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable_arg);
                let view_arg =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view_arg);
                let detail_arg = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail_arg);
                let screen_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x_arg);
                let screen_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y_arg);
                let client_x_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x_arg);
                let client_y_arg =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y_arg);
                let ctrl_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key_arg);
                let alt_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key_arg);
                let shift_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key_arg);
                let meta_key_arg =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key_arg);
                let button_arg = <i16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button_arg);
                let related_target_arg =
                    <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        related_target_arg,
                    );
                __widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg_MouseEvent ( self_ , type_arg , can_bubble_arg , cancelable_arg , view_arg , detail_arg , screen_x_arg , screen_y_arg , client_x_arg , client_y_arg , ctrl_key_arg , alt_key_arg , shift_key_arg , meta_key_arg , button_arg , related_target_arg )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_screen_x_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `screenX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn screen_x(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_screen_x_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_screen_x_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_screen_x_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_screen_y_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `screenY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn screen_y(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_screen_y_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_screen_y_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_screen_y_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_x_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `clientX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn client_x(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_x_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_x_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_x_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_client_y_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `clientY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn client_y(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_client_y_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_client_y_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_client_y_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/x)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/y)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_x_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `offsetX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn offset_x(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_x_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_x_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_x_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_y_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `offsetY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn offset_y(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_y_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_y_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_y_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ctrl_key_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `ctrlKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn ctrl_key(&self) -> bool {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ctrl_key_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ctrl_key_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ctrl_key_MouseEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shift_key_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `shiftKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn shift_key(&self) -> bool {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shift_key_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shift_key_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shift_key_MouseEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_key_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `altKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn alt_key(&self) -> bool {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_key_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_key_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_key_MouseEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_meta_key_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `metaKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn meta_key(&self) -> bool {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_meta_key_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_meta_key_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_meta_key_MouseEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_button_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i16 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `button` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn button(&self) -> i16 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_button_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_button_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_button_MouseEvent(self_)
            };
            <i16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buttons_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `buttons` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn buttons(&self) -> u16 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buttons_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buttons_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buttons_MouseEvent(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "EventTarget", feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_related_target_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <Option<EventTarget> as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "EventTarget", feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `relatedTarget` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn related_target(&self) -> Option<EventTarget> {
        #[cfg(all(feature = "EventTarget", feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_related_target_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_related_target_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_related_target_MouseEvent(self_)
            };
            <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_region_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `region` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/region)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn region(&self) -> Option<String> {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_region_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_region_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_region_MouseEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_movement_x_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `movementX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn movement_x(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_movement_x_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_movement_x_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_movement_x_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MouseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_movement_y_MouseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseEvent {
    #[cfg(all(feature = "MouseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `movementY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY)\n\n*This API requires the following crate features to be activated: `MouseEvent`*"]
    #[allow(clippy::all)]
    pub fn movement_y(&self) -> i32 {
        #[cfg(all(feature = "MouseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_movement_y_MouseEvent(
                self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_movement_y_MouseEvent(
            self_: <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MouseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_movement_y_MouseEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7899df07efc0c12e: [u8; 6215usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x05\x18\0\0\0\0%\0\0\x02\nMouseEvent\x1C__widl_instanceof_MouseEvent\0\0\0\0\x17__widl_f_new_MouseEvent\x01\0\0\x01\nMouseEvent\0\x01\x01\x08type_arg\x03new\0\0\02__widl_f_new_with_mouse_event_init_dict_MouseEvent\x01\0\0\x01\nMouseEvent\0\x01\x02\x08type_arg\x15mouse_event_init_dict\x03new\0\0\0&__widl_f_get_modifier_state_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x02\x05self_\x07key_arg\x10getModifierState\0\0\0$__widl_f_init_mouse_event_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x02\x05self_\x08type_arg\x0EinitMouseEvent\0\0\08__widl_f_init_mouse_event_with_can_bubble_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x03\x05self_\x08type_arg\x0Ecan_bubble_arg\x0EinitMouseEvent\0\0\0K__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x04\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x0EinitMouseEvent\0\0\0X__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x05\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\x0EinitMouseEvent\0\0\0g__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x06\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0EinitMouseEvent\0\0\0x__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x07\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0EinitMouseEvent\0\0\0\x89\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x08\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0EinitMouseEvent\0\0\0\x9A\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\t\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0EinitMouseEvent\0\0\0\xAB\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\n\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0EinitMouseEvent\0\0\0\xBC\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x0B\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0EinitMouseEvent\0\0\0\xCC\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x0C\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0Balt_key_arg\x0EinitMouseEvent\0\0\0\xDE\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\r\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0Balt_key_arg\rshift_key_arg\x0EinitMouseEvent\0\0\0\xEF\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x0E\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0Balt_key_arg\rshift_key_arg\x0Cmeta_key_arg\x0EinitMouseEvent\0\0\0\xFE\x01__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x0F\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0Balt_key_arg\rshift_key_arg\x0Cmeta_key_arg\nbutton_arg\x0EinitMouseEvent\0\0\0\x95\x02__widl_f_init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\0\x01\x10\x05self_\x08type_arg\x0Ecan_bubble_arg\x0Ecancelable_arg\x08view_arg\ndetail_arg\x0Cscreen_x_arg\x0Cscreen_y_arg\x0Cclient_x_arg\x0Cclient_y_arg\x0Cctrl_key_arg\x0Balt_key_arg\rshift_key_arg\x0Cmeta_key_arg\nbutton_arg\x12related_target_arg\x0EinitMouseEvent\0\0\0\x1C__widl_f_screen_x_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07screenX\x01\x01\x05self_\x07screenX\0\0\0\x1C__widl_f_screen_y_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07screenY\x01\x01\x05self_\x07screenY\0\0\0\x1C__widl_f_client_x_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07clientX\x01\x01\x05self_\x07clientX\0\0\0\x1C__widl_f_client_y_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07clientY\x01\x01\x05self_\x07clientY\0\0\0\x15__widl_f_x_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x15__widl_f_y_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1C__widl_f_offset_x_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07offsetX\x01\x01\x05self_\x07offsetX\0\0\0\x1C__widl_f_offset_y_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07offsetY\x01\x01\x05self_\x07offsetY\0\0\0\x1C__widl_f_ctrl_key_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07ctrlKey\x01\x01\x05self_\x07ctrlKey\0\0\0\x1D__widl_f_shift_key_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x08shiftKey\x01\x01\x05self_\x08shiftKey\0\0\0\x1B__widl_f_alt_key_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x06altKey\x01\x01\x05self_\x06altKey\0\0\0\x1C__widl_f_meta_key_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07metaKey\x01\x01\x05self_\x07metaKey\0\0\0\x1A__widl_f_button_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x06button\x01\x01\x05self_\x06button\0\0\0\x1B__widl_f_buttons_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x07buttons\x01\x01\x05self_\x07buttons\0\0\0\"__widl_f_related_target_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\rrelatedTarget\x01\x01\x05self_\rrelatedTarget\0\0\0\x1A__widl_f_region_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\x06region\x01\x01\x05self_\x06region\0\0\0\x1E__widl_f_movement_x_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\tmovementX\x01\x01\x05self_\tmovementX\0\0\0\x1E__widl_f_movement_y_MouseEvent\0\0\0\x01\nMouseEvent\x01\0\x01\tmovementY\x01\x01\x05self_\tmovementY\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

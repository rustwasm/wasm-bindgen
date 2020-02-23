use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MouseScrollEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MouseScrollEvent {
    obj: MouseEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MouseScrollEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MouseScrollEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(77u32);
            inform(111u32);
            inform(117u32);
            inform(115u32);
            inform(101u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for MouseScrollEvent {
        type Target = MouseEvent;
        #[inline]
        fn deref(&self) -> &MouseEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for MouseScrollEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MouseScrollEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MouseScrollEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MouseScrollEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MouseScrollEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MouseScrollEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MouseScrollEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MouseScrollEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MouseScrollEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MouseScrollEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MouseScrollEvent {
        #[inline]
        fn from(obj: JsValue) -> MouseScrollEvent {
            MouseScrollEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MouseScrollEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MouseScrollEvent> for MouseScrollEvent {
        #[inline]
        fn as_ref(&self) -> &MouseScrollEvent {
            self
        }
    }
    impl From<MouseScrollEvent> for JsValue {
        #[inline]
        fn from(obj: MouseScrollEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MouseScrollEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MouseScrollEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MouseScrollEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MouseScrollEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MouseScrollEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MouseScrollEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MouseScrollEvent> for MouseEvent {
    #[inline]
    fn from(obj: MouseScrollEvent) -> MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MouseEvent> for MouseScrollEvent {
    #[inline]
    fn as_ref(&self) -> &MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MouseScrollEvent> for UiEvent {
    #[inline]
    fn from(obj: MouseScrollEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for MouseScrollEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MouseScrollEvent> for Event {
    #[inline]
    fn from(obj: MouseScrollEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for MouseScrollEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MouseScrollEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: MouseScrollEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MouseScrollEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MouseScrollEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_MouseScrollEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event(&self, type_: &str) {
        #[cfg(all(feature = "MouseScrollEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_mouse_scroll_event_MouseScrollEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "MouseScrollEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_mouse_scroll_event_with_can_bubble_MouseScrollEvent(
                    self_, type_, can_bubble,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "MouseScrollEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_MouseScrollEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            drop(ctrl_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(12u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            drop(ctrl_key);
            drop(alt_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(13u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
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
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key , shift_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(14u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
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
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key , shift_key , meta_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(15u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
    ) {
        #[cfg(all(feature = "MouseScrollEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(button);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let button = <i16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key , shift_key , meta_key , button )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "EventTarget",
    feature = "MouseScrollEvent",
    feature = "Window",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(16u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
impl MouseScrollEvent {
    #[cfg(all(
        feature = "EventTarget",
        feature = "MouseScrollEvent",
        feature = "Window",
    ))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
        related_target: Option<&EventTarget>,
    ) {
        #[cfg(all(
            feature = "EventTarget",
            feature = "MouseScrollEvent",
            feature = "Window",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(button);
            drop(related_target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let button = <i16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                let related_target =
                    <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        related_target,
                    );
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key , shift_key , meta_key , button , related_target )
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "EventTarget",
    feature = "MouseScrollEvent",
    feature = "Window",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis_MouseScrollEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(17u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
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
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(
        feature = "EventTarget",
        feature = "MouseScrollEvent",
        feature = "Window",
    ))]
    #[allow(bad_style)]
    #[doc = "The `initMouseScrollEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)\n\n*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        screen_x: i32,
        screen_y: i32,
        client_x: i32,
        client_y: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        button: i16,
        related_target: Option<&EventTarget>,
        axis: i32,
    ) {
        #[cfg(all(
            feature = "EventTarget",
            feature = "MouseScrollEvent",
            feature = "Window",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                axis: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            screen_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_x: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            client_y: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <i16 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            related_target: <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            axis: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(screen_x);
            drop(screen_y);
            drop(client_x);
            drop(client_y);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(button);
            drop(related_target);
            drop(axis);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let screen_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_x);
                let screen_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(screen_y);
                let client_x = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_x);
                let client_y = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(client_y);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let button = <i16 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                let related_target =
                    <Option<&EventTarget> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        related_target,
                    );
                let axis = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(axis);
                __widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis_MouseScrollEvent ( self_ , type_ , can_bubble , cancelable , view , detail , screen_x , screen_y , client_x , client_y , ctrl_key , alt_key , shift_key , meta_key , button , related_target , axis )
            };
            ()
        }
    }
}
#[cfg(all(feature = "MouseScrollEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_axis_MouseScrollEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MouseScrollEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl MouseScrollEvent {
    #[cfg(all(feature = "MouseScrollEvent",))]
    #[allow(bad_style)]
    #[doc = "The `axis` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/axis)\n\n*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    #[allow(clippy::all)]
    pub fn axis(&self) -> i32 {
        #[cfg(all(feature = "MouseScrollEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_axis_MouseScrollEvent(
                self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_axis_MouseScrollEvent(
            self_: <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MouseScrollEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_axis_MouseScrollEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl MouseScrollEvent {
    pub const HORIZONTAL_AXIS: i32 = 1u64 as i32;
}
impl MouseScrollEvent {
    pub const VERTICAL_AXIS: i32 = 2u64 as i32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_35e25d57c23a4f5a: [u8; 4659usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF1\x11\0\0\0\0\x12\0\0\x02\x10MouseScrollEvent\"__widl_instanceof_MouseScrollEvent\0\0\0\01__widl_f_init_mouse_scroll_event_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x02\x05self_\x05type_\x14initMouseScrollEvent\0\0\0A__widl_f_init_mouse_scroll_event_with_can_bubble_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x14initMouseScrollEvent\0\0\0P__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x14initMouseScrollEvent\0\0\0Y__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x14initMouseScrollEvent\0\0\0d__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x14initMouseScrollEvent\0\0\0q__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x14initMouseScrollEvent\0\0\0~__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x14initMouseScrollEvent\0\0\0\x8B\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x14initMouseScrollEvent\0\0\0\x98\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\n\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x14initMouseScrollEvent\0\0\0\xA5\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x0B\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x14initMouseScrollEvent\0\0\0\xB1\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x0C\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\x14initMouseScrollEvent\0\0\0\xBF\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\r\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\tshift_key\x14initMouseScrollEvent\0\0\0\xCC\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x0E\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x14initMouseScrollEvent\0\0\0\xD7\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x0F\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x06button\x14initMouseScrollEvent\0\0\0\xEA\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x10\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x06button\x0Erelated_target\x14initMouseScrollEvent\0\0\0\xF3\x01__widl_f_init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\0\x01\x11\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08screen_x\x08screen_y\x08client_x\x08client_y\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x06button\x0Erelated_target\x04axis\x14initMouseScrollEvent\0\0\0\x1E__widl_f_axis_MouseScrollEvent\0\0\0\x01\x10MouseScrollEvent\x01\0\x01\x04axis\x01\x01\x05self_\x04axis\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ScrollAreaEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ScrollAreaEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ScrollAreaEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ScrollAreaEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(65u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for ScrollAreaEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for ScrollAreaEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ScrollAreaEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ScrollAreaEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ScrollAreaEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ScrollAreaEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ScrollAreaEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ScrollAreaEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ScrollAreaEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ScrollAreaEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ScrollAreaEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ScrollAreaEvent {
        #[inline]
        fn from(obj: JsValue) -> ScrollAreaEvent {
            ScrollAreaEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ScrollAreaEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ScrollAreaEvent> for ScrollAreaEvent {
        #[inline]
        fn as_ref(&self) -> &ScrollAreaEvent {
            self
        }
    }
    impl From<ScrollAreaEvent> for JsValue {
        #[inline]
        fn from(obj: ScrollAreaEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ScrollAreaEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ScrollAreaEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ScrollAreaEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ScrollAreaEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ScrollAreaEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ScrollAreaEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ScrollAreaEvent> for UiEvent {
    #[inline]
    fn from(obj: ScrollAreaEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for ScrollAreaEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ScrollAreaEvent> for Event {
    #[inline]
    fn from(obj: ScrollAreaEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ScrollAreaEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ScrollAreaEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ScrollAreaEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ScrollAreaEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_ScrollAreaEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event(&self, type_: &str) {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_scroll_area_event_ScrollAreaEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_scroll_area_event_with_can_bubble_ScrollAreaEvent(
                    self_, type_, can_bubble,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_ScrollAreaEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view , detail )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(x);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view , detail , x )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(x);
            drop(y);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view , detail , x , y )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
        width: f32,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(x);
            drop(y);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view , detail , x , y , width )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height_ScrollAreaEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initScrollAreaEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        #[cfg(all(feature = "ScrollAreaEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(x);
            drop(y);
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let x = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let width = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height_ScrollAreaEvent ( self_ , type_ , can_bubble , cancelable , view , detail , x , y , width , height )
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_x_ScrollAreaEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `x` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/x)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn x(&self) -> f32 {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_x_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_x_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_x_ScrollAreaEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_y_ScrollAreaEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `y` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/y)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn y(&self) -> f32 {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_y_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_y_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_y_ScrollAreaEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_ScrollAreaEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/width)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> f32 {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_ScrollAreaEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScrollAreaEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_ScrollAreaEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScrollAreaEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl ScrollAreaEvent {
    #[cfg(all(feature = "ScrollAreaEvent",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/height)\n\n*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> f32 {
        #[cfg(all(feature = "ScrollAreaEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_ScrollAreaEvent(
                self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_ScrollAreaEvent(
            self_: <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScrollAreaEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_ScrollAreaEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f1f2ecb953f68ec4: [u8; 2097usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEF\x07\0\0\0\0\x0E\0\0\x02\x0FScrollAreaEvent!__widl_instanceof_ScrollAreaEvent\0\0\0\0/__widl_f_init_scroll_area_event_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x02\x05self_\x05type_\x13initScrollAreaEvent\0\0\0?__widl_f_init_scroll_area_event_with_can_bubble_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x13initScrollAreaEvent\0\0\0N__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x13initScrollAreaEvent\0\0\0W__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x13initScrollAreaEvent\0\0\0b__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x13initScrollAreaEvent\0\0\0h__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x01x\x13initScrollAreaEvent\0\0\0n__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x01x\x01y\x13initScrollAreaEvent\0\0\0x__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x01x\x01y\x05width\x13initScrollAreaEvent\0\0\0\x83\x01__widl_f_init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\0\x01\n\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x01x\x01y\x05width\x06height\x13initScrollAreaEvent\0\0\0\x1A__widl_f_x_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\x01\x01x\x01\x01\x05self_\x01x\0\0\0\x1A__widl_f_y_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\x01\x01y\x01\x01\x05self_\x01y\0\0\0\x1E__widl_f_width_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1F__widl_f_height_ScrollAreaEvent\0\0\0\x01\x0FScrollAreaEvent\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

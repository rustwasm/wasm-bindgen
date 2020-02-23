use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TouchEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TouchEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TouchEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TouchEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(84u32);
            inform(111u32);
            inform(117u32);
            inform(99u32);
            inform(104u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TouchEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for TouchEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TouchEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TouchEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TouchEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TouchEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TouchEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TouchEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TouchEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TouchEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TouchEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TouchEvent {
        #[inline]
        fn from(obj: JsValue) -> TouchEvent {
            TouchEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TouchEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TouchEvent> for TouchEvent {
        #[inline]
        fn as_ref(&self) -> &TouchEvent {
            self
        }
    }
    impl From<TouchEvent> for JsValue {
        #[inline]
        fn from(obj: TouchEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TouchEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TouchEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TouchEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TouchEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TouchEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TouchEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TouchEvent> for UiEvent {
    #[inline]
    fn from(obj: TouchEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for TouchEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TouchEvent> for Event {
    #[inline]
    fn from(obj: TouchEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TouchEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TouchEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TouchEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TouchEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TouchEvent as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<TouchEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TouchEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TouchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TouchEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TouchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_TouchEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TouchEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TouchEventInit as WasmDescribe>::describe();
    <TouchEvent as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TouchEventInit,
    ) -> Result<TouchEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TouchEvent", feature = "TouchEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_TouchEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&TouchEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TouchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_TouchEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&TouchEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TouchEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&TouchEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_TouchEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TouchEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event(&self, type_: &str) {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_touch_event_TouchEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_touch_event_with_can_bubble_TouchEvent(self_, type_, can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_TouchEvent(
                    self_, type_, can_bubble, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_TouchEvent(
                    self_, type_, can_bubble, cancelable, view,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(ctrl_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(ctrl_key);
            drop(alt_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(9u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key , shift_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(10u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key , shift_key , meta_key )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(11u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(touches);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touches);
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key , shift_key , meta_key , touches )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(12u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(touches);
            drop(target_touches);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touches);
                let target_touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target_touches,
                    );
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key , shift_key , meta_key , touches , target_touches )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches_TouchEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(13u32);
    <&TouchEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <Option<&TouchList> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTouchEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        view: Option<&Window>,
        detail: i32,
        ctrl_key: bool,
        alt_key: bool,
        shift_key: bool,
        meta_key: bool,
        touches: Option<&TouchList>,
        target_touches: Option<&TouchList>,
        changed_touches: Option<&TouchList>,
    ) {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                changed_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ctrl_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shift_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            meta_key: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            changed_touches: <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(view);
            drop(detail);
            drop(ctrl_key);
            drop(alt_key);
            drop(shift_key);
            drop(meta_key);
            drop(touches);
            drop(target_touches);
            drop(changed_touches);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let view = <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(view);
                let detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(detail);
                let ctrl_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ctrl_key);
                let alt_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt_key);
                let shift_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shift_key);
                let meta_key = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(meta_key);
                let touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touches);
                let target_touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target_touches,
                    );
                let changed_touches =
                    <Option<&TouchList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        changed_touches,
                    );
                __widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches_TouchEvent ( self_ , type_ , can_bubble , cancelable , view , detail , ctrl_key , alt_key , shift_key , meta_key , touches , target_touches , changed_touches )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_touches_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <TouchList as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
    #[allow(bad_style)]
    #[doc = "The `touches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    #[allow(clippy::all)]
    pub fn touches(&self) -> TouchList {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_touches_TouchEvent(self_)
            };
            <TouchList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_touches_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <TouchList as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
    #[allow(bad_style)]
    #[doc = "The `targetTouches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    #[allow(clippy::all)]
    pub fn target_touches(&self) -> TouchList {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_touches_TouchEvent(self_)
            };
            <TouchList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_changed_touches_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <TouchList as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
    #[allow(bad_style)]
    #[doc = "The `changedTouches` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)\n\n*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
    #[allow(clippy::all)]
    pub fn changed_touches(&self) -> TouchList {
        #[cfg(all(feature = "TouchEvent", feature = "TouchList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_changed_touches_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_changed_touches_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TouchList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_changed_touches_TouchEvent(self_)
            };
            <TouchList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_key_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `altKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn alt_key(&self) -> bool {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_key_TouchEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_meta_key_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `metaKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn meta_key(&self) -> bool {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_meta_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_meta_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_meta_key_TouchEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ctrl_key_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `ctrlKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn ctrl_key(&self) -> bool {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ctrl_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ctrl_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ctrl_key_TouchEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TouchEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shift_key_TouchEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TouchEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TouchEvent {
    #[cfg(all(feature = "TouchEvent",))]
    #[allow(bad_style)]
    #[doc = "The `shiftKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)\n\n*This API requires the following crate features to be activated: `TouchEvent`*"]
    #[allow(clippy::all)]
    pub fn shift_key(&self) -> bool {
        #[cfg(all(feature = "TouchEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shift_key_TouchEvent(
                self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shift_key_TouchEvent(
            self_: <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TouchEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shift_key_TouchEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b30ae61bca85c33: [u8; 3373usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEB\x0C\0\0\0\0\x16\0\0\x02\nTouchEvent\x1C__widl_instanceof_TouchEvent\0\0\0\0\x17__widl_f_new_TouchEvent\x01\0\0\x01\nTouchEvent\0\x01\x01\x05type_\x03new\0\0\0,__widl_f_new_with_event_init_dict_TouchEvent\x01\0\0\x01\nTouchEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0$__widl_f_init_touch_event_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x02\x05self_\x05type_\x0EinitTouchEvent\0\0\04__widl_f_init_touch_event_with_can_bubble_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x0EinitTouchEvent\0\0\0C__widl_f_init_touch_event_with_can_bubble_and_cancelable_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x0EinitTouchEvent\0\0\0L__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x0EinitTouchEvent\0\0\0W__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x0EinitTouchEvent\0\0\0d__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x0EinitTouchEvent\0\0\0p__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\x0EinitTouchEvent\0\0\0~__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\t\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\tshift_key\x0EinitTouchEvent\0\0\0\x8B\x01__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\n\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x0EinitTouchEvent\0\0\0\x97\x01__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x0B\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x07touches\x0EinitTouchEvent\0\0\0\xAA\x01__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\x0C\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x07touches\x0Etarget_touches\x0EinitTouchEvent\0\0\0\xBE\x01__widl_f_init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\0\x01\r\x05self_\x05type_\ncan_bubble\ncancelable\x04view\x06detail\x08ctrl_key\x07alt_key\tshift_key\x08meta_key\x07touches\x0Etarget_touches\x0Fchanged_touches\x0EinitTouchEvent\0\0\0\x1B__widl_f_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x07touches\x01\x01\x05self_\x07touches\0\0\0\"__widl_f_target_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\rtargetTouches\x01\x01\x05self_\rtargetTouches\0\0\0#__widl_f_changed_touches_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x0EchangedTouches\x01\x01\x05self_\x0EchangedTouches\0\0\0\x1B__widl_f_alt_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x06altKey\x01\x01\x05self_\x06altKey\0\0\0\x1C__widl_f_meta_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x07metaKey\x01\x01\x05self_\x07metaKey\0\0\0\x1C__widl_f_ctrl_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x07ctrlKey\x01\x01\x05self_\x07ctrlKey\0\0\0\x1D__widl_f_shift_key_TouchEvent\0\0\0\x01\nTouchEvent\x01\0\x01\x08shiftKey\x01\x01\x05self_\x08shiftKey\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

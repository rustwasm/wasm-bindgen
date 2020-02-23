use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `UIEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct UiEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_UiEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for UiEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(85u32);
            inform(73u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for UiEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for UiEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for UiEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a UiEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for UiEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            UiEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for UiEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a UiEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for UiEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<UiEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(UiEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for UiEvent {
        #[inline]
        fn from(obj: JsValue) -> UiEvent {
            UiEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for UiEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<UiEvent> for UiEvent {
        #[inline]
        fn as_ref(&self) -> &UiEvent {
            self
        }
    }
    impl From<UiEvent> for JsValue {
        #[inline]
        fn from(obj: UiEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for UiEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_UIEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_UIEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_UIEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            UiEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const UiEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<UiEvent> for Event {
    #[inline]
    fn from(obj: UiEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for UiEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<UiEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: UiEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for UiEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <UiEvent as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new UIEvent(..)` constructor, creating a new instance of `UIEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<UiEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_UIEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <UiEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_UIEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <UiEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_UIEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<UiEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "UiEvent", feature = "UiEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&UiEventInit as WasmDescribe>::describe();
    <UiEvent as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent", feature = "UiEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new UIEvent(..)` constructor, creating a new instance of `UIEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `UiEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &UiEventInit,
    ) -> Result<UiEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "UiEvent", feature = "UiEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_UIEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&UiEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <UiEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_UIEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&UiEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <UiEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&UiEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(event_init_dict);
                __widl_f_new_with_event_init_dict_UIEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<UiEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_ui_event_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&UiEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initUIEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn init_ui_event(&self, a_type: &str) {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_ui_event_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_ui_event_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                __widl_f_init_ui_event_UIEvent(self_, a_type)
            };
            ()
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_ui_event_with_a_can_bubble_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&UiEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initUIEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn init_ui_event_with_a_can_bubble(&self, a_type: &str, a_can_bubble: bool) {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_ui_event_with_a_can_bubble_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_ui_event_with_a_can_bubble_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_can_bubble);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_can_bubble =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_can_bubble);
                __widl_f_init_ui_event_with_a_can_bubble_UIEvent(self_, a_type, a_can_bubble)
            };
            ()
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_UIEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&UiEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initUIEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable(
        &self,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
    ) {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_can_bubble);
            drop(a_cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_can_bubble =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_can_bubble);
                let a_cancelable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_cancelable);
                __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_UIEvent(
                    self_,
                    a_type,
                    a_can_bubble,
                    a_cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "UiEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_UIEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&UiEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initUIEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view(
        &self,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
    ) {
        #[cfg(all(feature = "UiEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_can_bubble);
            drop(a_cancelable);
            drop(a_view);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_can_bubble =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_can_bubble);
                let a_cancelable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_UIEvent(
                    self_,
                    a_type,
                    a_can_bubble,
                    a_cancelable,
                    a_view,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "UiEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail_UIEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&UiEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initUIEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail(
        &self,
        a_type: &str,
        a_can_bubble: bool,
        a_cancelable: bool,
        a_view: Option<&Window>,
        a_detail: i32,
    ) {
        #[cfg(all(feature = "UiEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_can_bubble);
            drop(a_cancelable);
            drop(a_view);
            drop(a_detail);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_can_bubble =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_can_bubble);
                let a_cancelable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_cancelable);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                __widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail_UIEvent ( self_ , a_type , a_can_bubble , a_cancelable , a_view , a_detail )
            };
            ()
        }
    }
}
#[cfg(all(feature = "UiEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `view` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)\n\n*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn view(&self) -> Option<Window> {
        #[cfg(all(feature = "UiEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_UIEvent(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detail_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `detail` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn detail(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detail_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detail_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detail_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_layer_x_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `layerX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerX)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn layer_x(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_layer_x_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_layer_x_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_layer_x_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_layer_y_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `layerY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerY)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn layer_y(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_layer_y_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_layer_y_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_layer_y_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_page_x_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pageX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageX)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn page_x(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_page_x_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_page_x_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_page_x_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_page_y_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pageY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageY)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn page_y(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_page_y_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_page_y_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_page_y_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_which_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `which` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn which(&self) -> u32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_which_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_which_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_which_UIEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Node", feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_parent_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "Node", feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `rangeParent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeParent)\n\n*This API requires the following crate features to be activated: `Node`, `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn range_parent(&self) -> Option<Node> {
        #[cfg(all(feature = "Node", feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_parent_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_parent_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_parent_UIEvent(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "UiEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_offset_UIEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&UiEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl UiEvent {
    #[cfg(all(feature = "UiEvent",))]
    #[allow(bad_style)]
    #[doc = "The `rangeOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeOffset)\n\n*This API requires the following crate features to be activated: `UiEvent`*"]
    #[allow(clippy::all)]
    pub fn range_offset(&self) -> i32 {
        #[cfg(all(feature = "UiEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_offset_UIEvent(
                self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_offset_UIEvent(
            self_: <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&UiEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_offset_UIEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl UiEvent {
    pub const SCROLL_PAGE_UP: i32 = -32768i64 as i32;
}
impl UiEvent {
    pub const SCROLL_PAGE_DOWN: i32 = 32768u64 as i32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a86b4a7a744c9891: [u8; 1527usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB5\x05\0\0\0\0\x11\0\0\x02\x07UIEvent\x19__widl_instanceof_UIEvent\0\0\0\0\x14__widl_f_new_UIEvent\x01\0\0\x01\x07UIEvent\0\x01\x01\x05type_\x03new\0\0\0)__widl_f_new_with_event_init_dict_UIEvent\x01\0\0\x01\x07UIEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1E__widl_f_init_ui_event_UIEvent\0\0\0\x01\x07UIEvent\x01\0\0\x01\x02\x05self_\x06a_type\x0BinitUIEvent\0\0\00__widl_f_init_ui_event_with_a_can_bubble_UIEvent\0\0\0\x01\x07UIEvent\x01\0\0\x01\x03\x05self_\x06a_type\x0Ca_can_bubble\x0BinitUIEvent\0\0\0A__widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_UIEvent\0\0\0\x01\x07UIEvent\x01\0\0\x01\x04\x05self_\x06a_type\x0Ca_can_bubble\x0Ca_cancelable\x0BinitUIEvent\0\0\0L__widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_UIEvent\0\0\0\x01\x07UIEvent\x01\0\0\x01\x05\x05self_\x06a_type\x0Ca_can_bubble\x0Ca_cancelable\x06a_view\x0BinitUIEvent\0\0\0Y__widl_f_init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail_UIEvent\0\0\0\x01\x07UIEvent\x01\0\0\x01\x06\x05self_\x06a_type\x0Ca_can_bubble\x0Ca_cancelable\x06a_view\x08a_detail\x0BinitUIEvent\0\0\0\x15__widl_f_view_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x04view\x01\x01\x05self_\x04view\0\0\0\x17__widl_f_detail_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x06detail\x01\x01\x05self_\x06detail\0\0\0\x18__widl_f_layer_x_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x06layerX\x01\x01\x05self_\x06layerX\0\0\0\x18__widl_f_layer_y_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x06layerY\x01\x01\x05self_\x06layerY\0\0\0\x17__widl_f_page_x_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x05pageX\x01\x01\x05self_\x05pageX\0\0\0\x17__widl_f_page_y_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x05pageY\x01\x01\x05self_\x05pageY\0\0\0\x16__widl_f_which_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x05which\x01\x01\x05self_\x05which\0\0\0\x1D__widl_f_range_parent_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x0BrangeParent\x01\x01\x05self_\x0BrangeParent\0\0\0\x1D__widl_f_range_offset_UIEvent\0\0\0\x01\x07UIEvent\x01\0\x01\x0BrangeOffset\x01\x01\x05self_\x0BrangeOffset\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

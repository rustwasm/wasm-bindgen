use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PointerEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PointerEvent {
    obj: MouseEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PointerEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PointerEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(80u32);
            inform(111u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PointerEvent {
        type Target = MouseEvent;
        #[inline]
        fn deref(&self) -> &MouseEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for PointerEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PointerEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PointerEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PointerEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PointerEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PointerEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PointerEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PointerEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PointerEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PointerEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PointerEvent {
        #[inline]
        fn from(obj: JsValue) -> PointerEvent {
            PointerEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PointerEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PointerEvent> for PointerEvent {
        #[inline]
        fn as_ref(&self) -> &PointerEvent {
            self
        }
    }
    impl From<PointerEvent> for JsValue {
        #[inline]
        fn from(obj: PointerEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PointerEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PointerEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PointerEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PointerEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PointerEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PointerEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PointerEvent> for MouseEvent {
    #[inline]
    fn from(obj: PointerEvent) -> MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MouseEvent> for PointerEvent {
    #[inline]
    fn as_ref(&self) -> &MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PointerEvent> for UiEvent {
    #[inline]
    fn from(obj: PointerEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for PointerEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PointerEvent> for Event {
    #[inline]
    fn from(obj: PointerEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PointerEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PointerEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PointerEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PointerEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PointerEvent as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PointerEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PointerEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PointerEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PointerEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PointerEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PointerEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PointerEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PointerEvent", feature = "PointerEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PointerEventInit as WasmDescribe>::describe();
    <PointerEvent as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent", feature = "PointerEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)\n\n*This API requires the following crate features to be activated: `PointerEvent`, `PointerEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PointerEventInit,
    ) -> Result<PointerEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PointerEvent", feature = "PointerEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PointerEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&PointerEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PointerEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PointerEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&PointerEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PointerEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PointerEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_PointerEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PointerEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_coalesced_events_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `getCoalescedEvents()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn get_coalesced_events(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_coalesced_events_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_coalesced_events_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_coalesced_events_PointerEvent(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pointer_id_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pointerId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn pointer_id(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pointer_id_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pointer_id_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pointer_id_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pressure_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pressure` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn pressure(&self) -> f32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pressure_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pressure_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pressure_PointerEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tangential_pressure_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `tangentialPressure` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn tangential_pressure(&self) -> f32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tangential_pressure_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tangential_pressure_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tangential_pressure_PointerEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tilt_x_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `tiltX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn tilt_x(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tilt_x_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tilt_x_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tilt_x_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tilt_y_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `tiltY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn tilt_y(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tilt_y_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tilt_y_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tilt_y_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_twist_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `twist` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn twist(&self) -> i32 {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_twist_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_twist_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_twist_PointerEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pointer_type_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pointerType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn pointer_type(&self) -> String {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pointer_type_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pointer_type_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pointer_type_PointerEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PointerEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_primary_PointerEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PointerEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl PointerEvent {
    #[cfg(all(feature = "PointerEvent",))]
    #[allow(bad_style)]
    #[doc = "The `isPrimary` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)\n\n*This API requires the following crate features to be activated: `PointerEvent`*"]
    #[allow(clippy::all)]
    pub fn is_primary(&self) -> bool {
        #[cfg(all(feature = "PointerEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_primary_PointerEvent(
                self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_primary_PointerEvent(
            self_: <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PointerEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_primary_PointerEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_b0b5d5dd270e3318: [u8; 1210usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}x\x04\0\0\0\0\x0E\0\0\x02\x0CPointerEvent\x1E__widl_instanceof_PointerEvent\0\0\0\0\x19__widl_f_new_PointerEvent\x01\0\0\x01\x0CPointerEvent\0\x01\x01\x05type_\x03new\0\0\0.__widl_f_new_with_event_init_dict_PointerEvent\x01\0\0\x01\x0CPointerEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0*__widl_f_get_coalesced_events_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\0\x01\x01\x05self_\x12getCoalescedEvents\0\0\0 __widl_f_pointer_id_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\tpointerId\x01\x01\x05self_\tpointerId\0\0\0\x1B__widl_f_width_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\x1C__widl_f_height_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0\x1E__widl_f_pressure_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x08pressure\x01\x01\x05self_\x08pressure\0\0\0)__widl_f_tangential_pressure_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x12tangentialPressure\x01\x01\x05self_\x12tangentialPressure\0\0\0\x1C__widl_f_tilt_x_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x05tiltX\x01\x01\x05self_\x05tiltX\0\0\0\x1C__widl_f_tilt_y_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x05tiltY\x01\x01\x05self_\x05tiltY\0\0\0\x1B__widl_f_twist_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x05twist\x01\x01\x05self_\x05twist\0\0\0\"__widl_f_pointer_type_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\x0BpointerType\x01\x01\x05self_\x0BpointerType\0\0\0 __widl_f_is_primary_PointerEvent\0\0\0\x01\x0CPointerEvent\x01\0\x01\tisPrimary\x01\x01\x05self_\tisPrimary\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

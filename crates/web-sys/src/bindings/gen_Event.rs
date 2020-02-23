use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Event` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event)\n\n*This API requires the following crate features to be activated: `Event`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Event {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Event: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Event {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(5u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Event {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Event {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Event {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Event {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Event {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Event {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Event {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Event {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Event {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Event>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Event {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Event {
        #[inline]
        fn from(obj: JsValue) -> Event {
            Event { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Event {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Event> for Event {
        #[inline]
        fn as_ref(&self) -> &Event {
            self
        }
    }
    impl From<Event> for JsValue {
        #[inline]
        fn from(obj: Event) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Event {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Event(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Event(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Event(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Event { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Event) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Event> for ::js_sys::Object {
    #[inline]
    fn from(obj: Event) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Event {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Event as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<Event, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Event(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Event(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_Event(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Event as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Event", feature = "EventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&EventInit as WasmDescribe>::describe();
    <Event as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event", feature = "EventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)\n\n*This API requires the following crate features to be activated: `Event`, `EventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &EventInit,
    ) -> Result<Event, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Event", feature = "EventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_Event(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&EventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_Event(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&EventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Event as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&EventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(event_init_dict);
                __widl_f_new_with_event_init_dict_Event(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Event as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_composed_path_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `composedPath()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn composed_path(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_composed_path_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_composed_path_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_composed_path_Event(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_event_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Event as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `initEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn init_event(&self, type_: &str) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_event_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_event_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_event_Event(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_event_with_bubbles_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Event as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `initEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn init_event_with_bubbles(&self, type_: &str, bubbles: bool) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_event_with_bubbles_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_event_with_bubbles_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                __widl_f_init_event_with_bubbles_Event(self_, type_, bubbles)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_event_with_bubbles_and_cancelable_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&Event as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `initEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn init_event_with_bubbles_and_cancelable(
        &self,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_event_with_bubbles_and_cancelable_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_event_with_bubbles_and_cancelable_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bubbles: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(bubbles);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let bubbles = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bubbles);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_event_with_bubbles_and_cancelable_Event(
                    self_, type_, bubbles, cancelable,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prevent_default_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `preventDefault()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn prevent_default(&self) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prevent_default_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prevent_default_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prevent_default_Event(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_immediate_propagation_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `stopImmediatePropagation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn stop_immediate_propagation(&self) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_immediate_propagation_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_immediate_propagation_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_immediate_propagation_Event(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_propagation_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `stopPropagation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn stop_propagation(&self) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_propagation_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_propagation_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_propagation_Event(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_Event(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <Option<EventTarget> as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)\n\n*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> Option<EventTarget> {
        #[cfg(all(feature = "Event", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_Event(self_)
            };
            <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event", feature = "EventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_target_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <Option<EventTarget> as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event", feature = "EventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `currentTarget` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)\n\n*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    #[allow(clippy::all)]
    pub fn current_target(&self) -> Option<EventTarget> {
        #[cfg(all(feature = "Event", feature = "EventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_target_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_target_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_target_Event(self_)
            };
            <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_event_phase_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `eventPhase` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn event_phase(&self) -> u16 {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_event_phase_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_event_phase_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_event_phase_Event(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bubbles_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `bubbles` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn bubbles(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bubbles_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bubbles_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bubbles_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancelable_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `cancelable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn cancelable(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancelable_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancelable_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cancelable_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_prevented_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `defaultPrevented` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn default_prevented(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_prevented_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_prevented_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_prevented_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_composed_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `composed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn composed(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_composed_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_composed_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_composed_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_trusted_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `isTrusted` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn is_trusted(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_trusted_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_trusted_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_trusted_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_time_stamp_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `timeStamp` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn time_stamp(&self) -> f64 {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_time_stamp_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_time_stamp_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_time_stamp_Event(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_bubble_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `cancelBubble` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn cancel_bubble(&self) -> bool {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancel_bubble_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancel_bubble_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cancel_bubble_Event(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Event",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cancel_bubble_Event() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Event as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Event {
    #[cfg(all(feature = "Event",))]
    #[allow(bad_style)]
    #[doc = "The `cancelBubble` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)\n\n*This API requires the following crate features to be activated: `Event`*"]
    #[allow(clippy::all)]
    pub fn set_cancel_bubble(&self, cancel_bubble: bool) {
        #[cfg(all(feature = "Event",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cancel_bubble_Event(
                self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancel_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cancel_bubble_Event(
            self_: <&Event as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancel_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cancel_bubble);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Event as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cancel_bubble =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancel_bubble);
                __widl_f_set_cancel_bubble_Event(self_, cancel_bubble)
            };
            ()
        }
    }
}
impl Event {
    pub const NONE: u16 = 0i64 as u16;
}
impl Event {
    pub const CAPTURING_PHASE: u16 = 1u64 as u16;
}
impl Event {
    pub const AT_TARGET: u16 = 2u64 as u16;
}
impl Event {
    pub const BUBBLING_PHASE: u16 = 3u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_380fddb5f9902884: [u8; 1708usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}j\x06\0\0\0\0\x16\0\0\x02\x05Event\x17__widl_instanceof_Event\0\0\0\0\x12__widl_f_new_Event\x01\0\0\x01\x05Event\0\x01\x01\x05type_\x03new\0\0\0'__widl_f_new_with_event_init_dict_Event\x01\0\0\x01\x05Event\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1C__widl_f_composed_path_Event\0\0\0\x01\x05Event\x01\0\0\x01\x01\x05self_\x0CcomposedPath\0\0\0\x19__widl_f_init_event_Event\0\0\0\x01\x05Event\x01\0\0\x01\x02\x05self_\x05type_\tinitEvent\0\0\0&__widl_f_init_event_with_bubbles_Event\0\0\0\x01\x05Event\x01\0\0\x01\x03\x05self_\x05type_\x07bubbles\tinitEvent\0\0\05__widl_f_init_event_with_bubbles_and_cancelable_Event\0\0\0\x01\x05Event\x01\0\0\x01\x04\x05self_\x05type_\x07bubbles\ncancelable\tinitEvent\0\0\0\x1E__widl_f_prevent_default_Event\0\0\0\x01\x05Event\x01\0\0\x01\x01\x05self_\x0EpreventDefault\0\0\0)__widl_f_stop_immediate_propagation_Event\0\0\0\x01\x05Event\x01\0\0\x01\x01\x05self_\x18stopImmediatePropagation\0\0\0\x1F__widl_f_stop_propagation_Event\0\0\0\x01\x05Event\x01\0\0\x01\x01\x05self_\x0FstopPropagation\0\0\0\x13__widl_f_type_Event\0\0\0\x01\x05Event\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x15__widl_f_target_Event\0\0\0\x01\x05Event\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0\x1D__widl_f_current_target_Event\0\0\0\x01\x05Event\x01\0\x01\rcurrentTarget\x01\x01\x05self_\rcurrentTarget\0\0\0\x1A__widl_f_event_phase_Event\0\0\0\x01\x05Event\x01\0\x01\neventPhase\x01\x01\x05self_\neventPhase\0\0\0\x16__widl_f_bubbles_Event\0\0\0\x01\x05Event\x01\0\x01\x07bubbles\x01\x01\x05self_\x07bubbles\0\0\0\x19__widl_f_cancelable_Event\0\0\0\x01\x05Event\x01\0\x01\ncancelable\x01\x01\x05self_\ncancelable\0\0\0 __widl_f_default_prevented_Event\0\0\0\x01\x05Event\x01\0\x01\x10defaultPrevented\x01\x01\x05self_\x10defaultPrevented\0\0\0\x17__widl_f_composed_Event\0\0\0\x01\x05Event\x01\0\x01\x08composed\x01\x01\x05self_\x08composed\0\0\0\x19__widl_f_is_trusted_Event\0\0\0\x01\x05Event\x01\0\x01\tisTrusted\x01\x01\x05self_\tisTrusted\0\0\0\x19__widl_f_time_stamp_Event\0\0\0\x01\x05Event\x01\0\x01\ttimeStamp\x01\x01\x05self_\ttimeStamp\0\0\0\x1C__widl_f_cancel_bubble_Event\0\0\0\x01\x05Event\x01\0\x01\x0CcancelBubble\x01\x01\x05self_\x0CcancelBubble\0\0\0 __widl_f_set_cancel_bubble_Event\0\0\0\x01\x05Event\x01\0\x02\x0CcancelBubble\x01\x02\x05self_\rcancel_bubble\x0CcancelBubble\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

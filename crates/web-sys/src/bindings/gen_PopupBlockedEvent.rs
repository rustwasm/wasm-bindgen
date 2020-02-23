use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PopupBlockedEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PopupBlockedEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PopupBlockedEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PopupBlockedEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(80u32);
            inform(111u32);
            inform(112u32);
            inform(117u32);
            inform(112u32);
            inform(66u32);
            inform(108u32);
            inform(111u32);
            inform(99u32);
            inform(107u32);
            inform(101u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PopupBlockedEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for PopupBlockedEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PopupBlockedEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PopupBlockedEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PopupBlockedEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PopupBlockedEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PopupBlockedEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PopupBlockedEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PopupBlockedEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PopupBlockedEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PopupBlockedEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PopupBlockedEvent {
        #[inline]
        fn from(obj: JsValue) -> PopupBlockedEvent {
            PopupBlockedEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PopupBlockedEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PopupBlockedEvent> for PopupBlockedEvent {
        #[inline]
        fn as_ref(&self) -> &PopupBlockedEvent {
            self
        }
    }
    impl From<PopupBlockedEvent> for JsValue {
        #[inline]
        fn from(obj: PopupBlockedEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PopupBlockedEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PopupBlockedEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PopupBlockedEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PopupBlockedEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PopupBlockedEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PopupBlockedEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PopupBlockedEvent> for Event {
    #[inline]
    fn from(obj: PopupBlockedEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PopupBlockedEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PopupBlockedEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PopupBlockedEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PopupBlockedEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PopupBlockedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PopupBlockedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PopupBlockedEvent as WasmDescribe>::describe();
}
impl PopupBlockedEvent {
    #[cfg(all(feature = "PopupBlockedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PopupBlockedEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PopupBlockedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PopupBlockedEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PopupBlockedEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PopupBlockedEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PopupBlockedEvent", feature = "PopupBlockedEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PopupBlockedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PopupBlockedEventInit as WasmDescribe>::describe();
    <PopupBlockedEvent as WasmDescribe>::describe();
}
impl PopupBlockedEvent {
    #[cfg(all(feature = "PopupBlockedEvent", feature = "PopupBlockedEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopupBlockedEventInit,
    ) -> Result<PopupBlockedEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PopupBlockedEvent", feature = "PopupBlockedEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PopupBlockedEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PopupBlockedEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PopupBlockedEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&PopupBlockedEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PopupBlockedEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_PopupBlockedEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PopupBlockedEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PopupBlockedEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_requesting_window_PopupBlockedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PopupBlockedEvent as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl PopupBlockedEvent {
    #[cfg(all(feature = "PopupBlockedEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `requestingWindow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn requesting_window(&self) -> Option<Window> {
        #[cfg(all(feature = "PopupBlockedEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_requesting_window_PopupBlockedEvent(
                self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_requesting_window_PopupBlockedEvent(
            self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_requesting_window_PopupBlockedEvent(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PopupBlockedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_popup_window_name_PopupBlockedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PopupBlockedEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PopupBlockedEvent {
    #[cfg(all(feature = "PopupBlockedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `popupWindowName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    #[allow(clippy::all)]
    pub fn popup_window_name(&self) -> Option<String> {
        #[cfg(all(feature = "PopupBlockedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_popup_window_name_PopupBlockedEvent(
                self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_popup_window_name_PopupBlockedEvent(
            self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_popup_window_name_PopupBlockedEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PopupBlockedEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_popup_window_features_PopupBlockedEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PopupBlockedEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PopupBlockedEvent {
    #[cfg(all(feature = "PopupBlockedEvent",))]
    #[allow(bad_style)]
    #[doc = "The `popupWindowFeatures` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)\n\n*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    #[allow(clippy::all)]
    pub fn popup_window_features(&self) -> Option<String> {
        #[cfg(all(feature = "PopupBlockedEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_popup_window_features_PopupBlockedEvent(
                self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_popup_window_features_PopupBlockedEvent(
            self_: <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PopupBlockedEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_popup_window_features_PopupBlockedEvent(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_47e9dd93beccc9f6: [u8; 691usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}q\x02\0\0\0\0\x06\0\0\x02\x11PopupBlockedEvent#__widl_instanceof_PopupBlockedEvent\0\0\0\0\x1E__widl_f_new_PopupBlockedEvent\x01\0\0\x01\x11PopupBlockedEvent\0\x01\x01\x05type_\x03new\0\0\03__widl_f_new_with_event_init_dict_PopupBlockedEvent\x01\0\0\x01\x11PopupBlockedEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0,__widl_f_requesting_window_PopupBlockedEvent\0\0\0\x01\x11PopupBlockedEvent\x01\0\x01\x10requestingWindow\x01\x01\x05self_\x10requestingWindow\0\0\0,__widl_f_popup_window_name_PopupBlockedEvent\0\0\0\x01\x11PopupBlockedEvent\x01\0\x01\x0FpopupWindowName\x01\x01\x05self_\x0FpopupWindowName\0\0\00__widl_f_popup_window_features_PopupBlockedEvent\0\0\0\x01\x11PopupBlockedEvent\x01\0\x01\x13popupWindowFeatures\x01\x01\x05self_\x13popupWindowFeatures\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

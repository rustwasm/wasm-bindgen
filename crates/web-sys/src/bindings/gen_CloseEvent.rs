use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CloseEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CloseEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CloseEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CloseEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(67u32);
            inform(108u32);
            inform(111u32);
            inform(115u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for CloseEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for CloseEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CloseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CloseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CloseEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CloseEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CloseEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CloseEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CloseEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CloseEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CloseEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CloseEvent {
        #[inline]
        fn from(obj: JsValue) -> CloseEvent {
            CloseEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CloseEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CloseEvent> for CloseEvent {
        #[inline]
        fn as_ref(&self) -> &CloseEvent {
            self
        }
    }
    impl From<CloseEvent> for JsValue {
        #[inline]
        fn from(obj: CloseEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CloseEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CloseEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CloseEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CloseEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CloseEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CloseEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CloseEvent> for Event {
    #[inline]
    fn from(obj: CloseEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for CloseEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CloseEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: CloseEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CloseEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CloseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_CloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <CloseEvent as WasmDescribe>::describe();
}
impl CloseEvent {
    #[cfg(all(feature = "CloseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<CloseEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CloseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_CloseEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_CloseEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_CloseEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CloseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CloseEvent", feature = "CloseEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_CloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&CloseEventInit as WasmDescribe>::describe();
    <CloseEvent as WasmDescribe>::describe();
}
impl CloseEvent {
    #[cfg(all(feature = "CloseEvent", feature = "CloseEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)\n\n*This API requires the following crate features to be activated: `CloseEvent`, `CloseEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CloseEventInit,
    ) -> Result<CloseEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CloseEvent", feature = "CloseEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_CloseEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&CloseEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_CloseEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&CloseEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CloseEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_CloseEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<CloseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CloseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_was_clean_CloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CloseEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CloseEvent {
    #[cfg(all(feature = "CloseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `wasClean` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    #[allow(clippy::all)]
    pub fn was_clean(&self) -> bool {
        #[cfg(all(feature = "CloseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_was_clean_CloseEvent(
                self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_was_clean_CloseEvent(
            self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_was_clean_CloseEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CloseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_CloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CloseEvent as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl CloseEvent {
    #[cfg(all(feature = "CloseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> u16 {
        #[cfg(all(feature = "CloseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_CloseEvent(
                self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_CloseEvent(
            self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_CloseEvent(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CloseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reason_CloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CloseEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CloseEvent {
    #[cfg(all(feature = "CloseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `reason` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)\n\n*This API requires the following crate features to be activated: `CloseEvent`*"]
    #[allow(clippy::all)]
    pub fn reason(&self) -> String {
        #[cfg(all(feature = "CloseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reason_CloseEvent(
                self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reason_CloseEvent(
            self_: <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CloseEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reason_CloseEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8e7bc2555b55d0ae: [u8; 507usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB9\x01\0\0\0\0\x06\0\0\x02\nCloseEvent\x1C__widl_instanceof_CloseEvent\0\0\0\0\x17__widl_f_new_CloseEvent\x01\0\0\x01\nCloseEvent\0\x01\x01\x05type_\x03new\0\0\0,__widl_f_new_with_event_init_dict_CloseEvent\x01\0\0\x01\nCloseEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1D__widl_f_was_clean_CloseEvent\0\0\0\x01\nCloseEvent\x01\0\x01\x08wasClean\x01\x01\x05self_\x08wasClean\0\0\0\x18__widl_f_code_CloseEvent\0\0\0\x01\nCloseEvent\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0\x1A__widl_f_reason_CloseEvent\0\0\0\x01\nCloseEvent\x01\0\x01\x06reason\x01\x01\x05self_\x06reason\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FocusEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)\n\n*This API requires the following crate features to be activated: `FocusEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FocusEvent {
    obj: UiEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FocusEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FocusEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(70u32);
            inform(111u32);
            inform(99u32);
            inform(117u32);
            inform(115u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for FocusEvent {
        type Target = UiEvent;
        #[inline]
        fn deref(&self) -> &UiEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for FocusEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FocusEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FocusEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FocusEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FocusEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FocusEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FocusEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FocusEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FocusEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FocusEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FocusEvent {
        #[inline]
        fn from(obj: JsValue) -> FocusEvent {
            FocusEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FocusEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FocusEvent> for FocusEvent {
        #[inline]
        fn as_ref(&self) -> &FocusEvent {
            self
        }
    }
    impl From<FocusEvent> for JsValue {
        #[inline]
        fn from(obj: FocusEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FocusEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FocusEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FocusEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FocusEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FocusEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FocusEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FocusEvent> for UiEvent {
    #[inline]
    fn from(obj: FocusEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for FocusEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FocusEvent> for Event {
    #[inline]
    fn from(obj: FocusEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for FocusEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FocusEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: FocusEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FocusEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FocusEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_FocusEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <FocusEvent as WasmDescribe>::describe();
}
impl FocusEvent {
    #[cfg(all(feature = "FocusEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)\n\n*This API requires the following crate features to be activated: `FocusEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_arg: &str) -> Result<FocusEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FocusEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_FocusEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FocusEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_FocusEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FocusEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                __widl_f_new_FocusEvent(type_arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FocusEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "FocusEvent", feature = "FocusEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_focus_event_init_dict_FocusEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&FocusEventInit as WasmDescribe>::describe();
    <FocusEvent as WasmDescribe>::describe();
}
impl FocusEvent {
    #[cfg(all(feature = "FocusEvent", feature = "FocusEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)\n\n*This API requires the following crate features to be activated: `FocusEvent`, `FocusEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_focus_event_init_dict(
        type_arg: &str,
        focus_event_init_dict: &FocusEventInit,
    ) -> Result<FocusEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "FocusEvent", feature = "FocusEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_focus_event_init_dict_FocusEvent(
                type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                focus_event_init_dict: <&FocusEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FocusEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_focus_event_init_dict_FocusEvent(
            type_arg: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            focus_event_init_dict: <&FocusEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FocusEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_arg);
            drop(focus_event_init_dict);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_arg = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_arg);
                let focus_event_init_dict =
                    <&FocusEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        focus_event_init_dict,
                    );
                __widl_f_new_with_focus_event_init_dict_FocusEvent(type_arg, focus_event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<FocusEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "EventTarget", feature = "FocusEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_related_target_FocusEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FocusEvent as WasmDescribe>::describe();
    <Option<EventTarget> as WasmDescribe>::describe();
}
impl FocusEvent {
    #[cfg(all(feature = "EventTarget", feature = "FocusEvent",))]
    #[allow(bad_style)]
    #[doc = "The `relatedTarget` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/relatedTarget)\n\n*This API requires the following crate features to be activated: `EventTarget`, `FocusEvent`*"]
    #[allow(clippy::all)]
    pub fn related_target(&self) -> Option<EventTarget> {
        #[cfg(all(feature = "EventTarget", feature = "FocusEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_related_target_FocusEvent(
                self_: <&FocusEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_related_target_FocusEvent(
            self_: <&FocusEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FocusEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_related_target_FocusEvent(self_)
            };
            <Option<EventTarget> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_67e2dc8663e813b0: [u8; 406usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}T\x01\0\0\0\0\x04\0\0\x02\nFocusEvent\x1C__widl_instanceof_FocusEvent\0\0\0\0\x17__widl_f_new_FocusEvent\x01\0\0\x01\nFocusEvent\0\x01\x01\x08type_arg\x03new\0\0\02__widl_f_new_with_focus_event_init_dict_FocusEvent\x01\0\0\x01\nFocusEvent\0\x01\x02\x08type_arg\x15focus_event_init_dict\x03new\0\0\0\"__widl_f_related_target_FocusEvent\0\0\0\x01\nFocusEvent\x01\0\x01\rrelatedTarget\x01\x01\x05self_\rrelatedTarget\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TransitionEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TransitionEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TransitionEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TransitionEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(105u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TransitionEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for TransitionEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TransitionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TransitionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TransitionEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TransitionEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TransitionEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TransitionEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TransitionEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TransitionEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TransitionEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TransitionEvent {
        #[inline]
        fn from(obj: JsValue) -> TransitionEvent {
            TransitionEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TransitionEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TransitionEvent> for TransitionEvent {
        #[inline]
        fn as_ref(&self) -> &TransitionEvent {
            self
        }
    }
    impl From<TransitionEvent> for JsValue {
        #[inline]
        fn from(obj: TransitionEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TransitionEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TransitionEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TransitionEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TransitionEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TransitionEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TransitionEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TransitionEvent> for Event {
    #[inline]
    fn from(obj: TransitionEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TransitionEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TransitionEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TransitionEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TransitionEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_TransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <TransitionEvent as WasmDescribe>::describe();
}
impl TransitionEvent {
    #[cfg(all(feature = "TransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<TransitionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_TransitionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_TransitionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_TransitionEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TransitionEvent", feature = "TransitionEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_TransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&TransitionEventInit as WasmDescribe>::describe();
    <TransitionEvent as WasmDescribe>::describe();
}
impl TransitionEvent {
    #[cfg(all(feature = "TransitionEvent", feature = "TransitionEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)\n\n*This API requires the following crate features to be activated: `TransitionEvent`, `TransitionEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TransitionEventInit,
    ) -> Result<TransitionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TransitionEvent", feature = "TransitionEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_TransitionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&TransitionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_TransitionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&TransitionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&TransitionEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_TransitionEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TransitionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "TransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_property_name_TransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TransitionEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TransitionEvent {
    #[cfg(all(feature = "TransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `propertyName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn property_name(&self) -> String {
        #[cfg(all(feature = "TransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_property_name_TransitionEvent(
                self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_property_name_TransitionEvent(
            self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_property_name_TransitionEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elapsed_time_TransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TransitionEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl TransitionEvent {
    #[cfg(all(feature = "TransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `elapsedTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn elapsed_time(&self) -> f32 {
        #[cfg(all(feature = "TransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elapsed_time_TransitionEvent(
                self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elapsed_time_TransitionEvent(
            self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_elapsed_time_TransitionEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TransitionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pseudo_element_TransitionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TransitionEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TransitionEvent {
    #[cfg(all(feature = "TransitionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pseudoElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)\n\n*This API requires the following crate features to be activated: `TransitionEvent`*"]
    #[allow(clippy::all)]
    pub fn pseudo_element(&self) -> String {
        #[cfg(all(feature = "TransitionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pseudo_element_TransitionEvent(
                self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pseudo_element_TransitionEvent(
            self_: <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&TransitionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pseudo_element_TransitionEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_510d7241013710c1: [u8; 623usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}-\x02\0\0\0\0\x06\0\0\x02\x0FTransitionEvent!__widl_instanceof_TransitionEvent\0\0\0\0\x1C__widl_f_new_TransitionEvent\x01\0\0\x01\x0FTransitionEvent\0\x01\x01\x05type_\x03new\0\0\01__widl_f_new_with_event_init_dict_TransitionEvent\x01\0\0\x01\x0FTransitionEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_property_name_TransitionEvent\0\0\0\x01\x0FTransitionEvent\x01\0\x01\x0CpropertyName\x01\x01\x05self_\x0CpropertyName\0\0\0%__widl_f_elapsed_time_TransitionEvent\0\0\0\x01\x0FTransitionEvent\x01\0\x01\x0BelapsedTime\x01\x01\x05self_\x0BelapsedTime\0\0\0'__widl_f_pseudo_element_TransitionEvent\0\0\0\x01\x0FTransitionEvent\x01\0\x01\rpseudoElement\x01\x01\x05self_\rpseudoElement\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

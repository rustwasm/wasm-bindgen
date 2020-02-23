use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechSynthesisEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechSynthesisEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechSynthesisEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechSynthesisEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(83u32);
            inform(112u32);
            inform(101u32);
            inform(101u32);
            inform(99u32);
            inform(104u32);
            inform(83u32);
            inform(121u32);
            inform(110u32);
            inform(116u32);
            inform(104u32);
            inform(101u32);
            inform(115u32);
            inform(105u32);
            inform(115u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SpeechSynthesisEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechSynthesisEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechSynthesisEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechSynthesisEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechSynthesisEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechSynthesisEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechSynthesisEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechSynthesisEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechSynthesisEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechSynthesisEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechSynthesisEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechSynthesisEvent {
        #[inline]
        fn from(obj: JsValue) -> SpeechSynthesisEvent {
            SpeechSynthesisEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechSynthesisEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechSynthesisEvent> for SpeechSynthesisEvent {
        #[inline]
        fn as_ref(&self) -> &SpeechSynthesisEvent {
            self
        }
    }
    impl From<SpeechSynthesisEvent> for JsValue {
        #[inline]
        fn from(obj: SpeechSynthesisEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechSynthesisEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechSynthesisEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechSynthesisEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechSynthesisEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechSynthesisEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechSynthesisEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechSynthesisEvent> for Event {
    #[inline]
    fn from(obj: SpeechSynthesisEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for SpeechSynthesisEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesisEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechSynthesisEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechSynthesisEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&SpeechSynthesisEventInit as WasmDescribe>::describe();
    <SpeechSynthesisEvent as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechSynthesisEvent(..)` constructor, creating a new instance of `SpeechSynthesisEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/SpeechSynthesisEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &SpeechSynthesisEventInit,
    ) -> Result<SpeechSynthesisEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechSynthesisEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & SpeechSynthesisEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SpeechSynthesisEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechSynthesisEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&SpeechSynthesisEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechSynthesisEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&SpeechSynthesisEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_SpeechSynthesisEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechSynthesisEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisUtterance",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_utterance_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisEvent as WasmDescribe>::describe();
    <SpeechSynthesisUtterance as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisUtterance",))]
    #[allow(bad_style)]
    #[doc = "The `utterance` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisUtterance`*"]
    #[allow(clippy::all)]
    pub fn utterance(&self) -> SpeechSynthesisUtterance {
        #[cfg(all(feature = "SpeechSynthesisEvent", feature = "SpeechSynthesisUtterance",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_utterance_SpeechSynthesisEvent(
                self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_utterance_SpeechSynthesisEvent(
            self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_utterance_SpeechSynthesisEvent(self_)
            };
            <SpeechSynthesisUtterance as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_char_index_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent",))]
    #[allow(bad_style)]
    #[doc = "The `charIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    #[allow(clippy::all)]
    pub fn char_index(&self) -> u32 {
        #[cfg(all(feature = "SpeechSynthesisEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_char_index_SpeechSynthesisEvent(
                self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_char_index_SpeechSynthesisEvent(
            self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_char_index_SpeechSynthesisEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_char_length_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisEvent as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent",))]
    #[allow(bad_style)]
    #[doc = "The `charLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    #[allow(clippy::all)]
    pub fn char_length(&self) -> Option<u32> {
        #[cfg(all(feature = "SpeechSynthesisEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_char_length_SpeechSynthesisEvent(
                self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_char_length_SpeechSynthesisEvent(
            self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_char_length_SpeechSynthesisEvent(self_)
            };
            <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elapsed_time_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent",))]
    #[allow(bad_style)]
    #[doc = "The `elapsedTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    #[allow(clippy::all)]
    pub fn elapsed_time(&self) -> f32 {
        #[cfg(all(feature = "SpeechSynthesisEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elapsed_time_SpeechSynthesisEvent(
                self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elapsed_time_SpeechSynthesisEvent(
            self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_elapsed_time_SpeechSynthesisEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SpeechSynthesisEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_SpeechSynthesisEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisEvent as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl SpeechSynthesisEvent {
    #[cfg(all(feature = "SpeechSynthesisEvent",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> Option<String> {
        #[cfg(all(feature = "SpeechSynthesisEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_SpeechSynthesisEvent(
                self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_SpeechSynthesisEvent(
            self_: <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SpeechSynthesisEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_SpeechSynthesisEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_ed41910cd0e61f16: [u8; 752usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAE\x02\0\0\0\0\x07\0\0\x02\x14SpeechSynthesisEvent&__widl_instanceof_SpeechSynthesisEvent\0\0\0\0!__widl_f_new_SpeechSynthesisEvent\x01\0\0\x01\x14SpeechSynthesisEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0'__widl_f_utterance_SpeechSynthesisEvent\0\0\0\x01\x14SpeechSynthesisEvent\x01\0\x01\tutterance\x01\x01\x05self_\tutterance\0\0\0(__widl_f_char_index_SpeechSynthesisEvent\0\0\0\x01\x14SpeechSynthesisEvent\x01\0\x01\tcharIndex\x01\x01\x05self_\tcharIndex\0\0\0)__widl_f_char_length_SpeechSynthesisEvent\0\0\0\x01\x14SpeechSynthesisEvent\x01\0\x01\ncharLength\x01\x01\x05self_\ncharLength\0\0\0*__widl_f_elapsed_time_SpeechSynthesisEvent\0\0\0\x01\x14SpeechSynthesisEvent\x01\0\x01\x0BelapsedTime\x01\x01\x05self_\x0BelapsedTime\0\0\0\"__widl_f_name_SpeechSynthesisEvent\0\0\0\x01\x14SpeechSynthesisEvent\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

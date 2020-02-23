use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SpeechSynthesisErrorEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SpeechSynthesisErrorEvent {
    obj: SpeechSynthesisEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SpeechSynthesisErrorEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SpeechSynthesisErrorEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
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
            inform(114u32);
            inform(114u32);
            inform(111u32);
            inform(114u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SpeechSynthesisErrorEvent {
        type Target = SpeechSynthesisEvent;
        #[inline]
        fn deref(&self) -> &SpeechSynthesisEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for SpeechSynthesisErrorEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SpeechSynthesisErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SpeechSynthesisErrorEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SpeechSynthesisErrorEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SpeechSynthesisErrorEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SpeechSynthesisErrorEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SpeechSynthesisErrorEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SpeechSynthesisErrorEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SpeechSynthesisErrorEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SpeechSynthesisErrorEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SpeechSynthesisErrorEvent {
        #[inline]
        fn from(obj: JsValue) -> SpeechSynthesisErrorEvent {
            SpeechSynthesisErrorEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SpeechSynthesisErrorEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SpeechSynthesisErrorEvent> for SpeechSynthesisErrorEvent {
        #[inline]
        fn as_ref(&self) -> &SpeechSynthesisErrorEvent {
            self
        }
    }
    impl From<SpeechSynthesisErrorEvent> for JsValue {
        #[inline]
        fn from(obj: SpeechSynthesisErrorEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SpeechSynthesisErrorEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SpeechSynthesisErrorEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SpeechSynthesisErrorEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SpeechSynthesisErrorEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SpeechSynthesisErrorEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SpeechSynthesisErrorEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SpeechSynthesisErrorEvent> for SpeechSynthesisEvent {
    #[inline]
    fn from(obj: SpeechSynthesisErrorEvent) -> SpeechSynthesisEvent {
        use wasm_bindgen::JsCast;
        SpeechSynthesisEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SpeechSynthesisEvent> for SpeechSynthesisErrorEvent {
    #[inline]
    fn as_ref(&self) -> &SpeechSynthesisEvent {
        use wasm_bindgen::JsCast;
        SpeechSynthesisEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesisErrorEvent> for Event {
    #[inline]
    fn from(obj: SpeechSynthesisErrorEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for SpeechSynthesisErrorEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SpeechSynthesisErrorEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: SpeechSynthesisErrorEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SpeechSynthesisErrorEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "SpeechSynthesisErrorEvent",
    feature = "SpeechSynthesisErrorEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_SpeechSynthesisErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&SpeechSynthesisErrorEventInit as WasmDescribe>::describe();
    <SpeechSynthesisErrorEvent as WasmDescribe>::describe();
}
impl SpeechSynthesisErrorEvent {
    #[cfg(all(
        feature = "SpeechSynthesisErrorEvent",
        feature = "SpeechSynthesisErrorEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new SpeechSynthesisErrorEvent(..)` constructor, creating a new instance of `SpeechSynthesisErrorEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/SpeechSynthesisErrorEvent)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisErrorEvent`, `SpeechSynthesisErrorEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &SpeechSynthesisErrorEventInit,
    ) -> Result<SpeechSynthesisErrorEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "SpeechSynthesisErrorEvent",
            feature = "SpeechSynthesisErrorEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_SpeechSynthesisErrorEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & SpeechSynthesisErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <SpeechSynthesisErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_SpeechSynthesisErrorEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & SpeechSynthesisErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <SpeechSynthesisErrorEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let event_init_dict = < & SpeechSynthesisErrorEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_SpeechSynthesisErrorEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SpeechSynthesisErrorEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "SpeechSynthesisErrorCode",
    feature = "SpeechSynthesisErrorEvent",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_SpeechSynthesisErrorEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SpeechSynthesisErrorEvent as WasmDescribe>::describe();
    <SpeechSynthesisErrorCode as WasmDescribe>::describe();
}
impl SpeechSynthesisErrorEvent {
    #[cfg(all(
        feature = "SpeechSynthesisErrorCode",
        feature = "SpeechSynthesisErrorEvent",
    ))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/error)\n\n*This API requires the following crate features to be activated: `SpeechSynthesisErrorCode`, `SpeechSynthesisErrorEvent`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> SpeechSynthesisErrorCode {
        #[cfg(all(
            feature = "SpeechSynthesisErrorCode",
            feature = "SpeechSynthesisErrorEvent",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_SpeechSynthesisErrorEvent(
                self_: <&SpeechSynthesisErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SpeechSynthesisErrorCode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_SpeechSynthesisErrorEvent(
            self_: <&SpeechSynthesisErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SpeechSynthesisErrorCode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SpeechSynthesisErrorEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_error_SpeechSynthesisErrorEvent(self_)
            };
            <SpeechSynthesisErrorCode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8c918b731f3f197d: [u8; 377usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}7\x01\0\0\0\0\x03\0\0\x02\x19SpeechSynthesisErrorEvent+__widl_instanceof_SpeechSynthesisErrorEvent\0\0\0\0&__widl_f_new_SpeechSynthesisErrorEvent\x01\0\0\x01\x19SpeechSynthesisErrorEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0(__widl_f_error_SpeechSynthesisErrorEvent\0\0\0\x01\x19SpeechSynthesisErrorEvent\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

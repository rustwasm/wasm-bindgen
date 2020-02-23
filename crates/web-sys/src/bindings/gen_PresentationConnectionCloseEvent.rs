use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PresentationConnectionCloseEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PresentationConnectionCloseEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PresentationConnectionCloseEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PresentationConnectionCloseEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(32u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
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
    impl core::ops::Deref for PresentationConnectionCloseEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for PresentationConnectionCloseEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PresentationConnectionCloseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PresentationConnectionCloseEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PresentationConnectionCloseEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PresentationConnectionCloseEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PresentationConnectionCloseEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PresentationConnectionCloseEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PresentationConnectionCloseEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PresentationConnectionCloseEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PresentationConnectionCloseEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PresentationConnectionCloseEvent {
        #[inline]
        fn from(obj: JsValue) -> PresentationConnectionCloseEvent {
            PresentationConnectionCloseEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PresentationConnectionCloseEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PresentationConnectionCloseEvent> for PresentationConnectionCloseEvent {
        #[inline]
        fn as_ref(&self) -> &PresentationConnectionCloseEvent {
            self
        }
    }
    impl From<PresentationConnectionCloseEvent> for JsValue {
        #[inline]
        fn from(obj: PresentationConnectionCloseEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PresentationConnectionCloseEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PresentationConnectionCloseEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PresentationConnectionCloseEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PresentationConnectionCloseEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PresentationConnectionCloseEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PresentationConnectionCloseEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PresentationConnectionCloseEvent> for Event {
    #[inline]
    fn from(obj: PresentationConnectionCloseEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PresentationConnectionCloseEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PresentationConnectionCloseEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PresentationConnectionCloseEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PresentationConnectionCloseEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "PresentationConnectionCloseEvent",
    feature = "PresentationConnectionCloseEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PresentationConnectionCloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PresentationConnectionCloseEventInit as WasmDescribe>::describe();
    <PresentationConnectionCloseEvent as WasmDescribe>::describe();
}
impl PresentationConnectionCloseEvent {
    #[cfg(all(
        feature = "PresentationConnectionCloseEvent",
        feature = "PresentationConnectionCloseEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PresentationConnectionCloseEvent(..)` constructor, creating a new instance of `PresentationConnectionCloseEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/PresentationConnectionCloseEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionCloseEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionCloseEventInit,
    ) -> Result<PresentationConnectionCloseEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "PresentationConnectionCloseEvent",
            feature = "PresentationConnectionCloseEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PresentationConnectionCloseEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PresentationConnectionCloseEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PresentationConnectionCloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PresentationConnectionCloseEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & PresentationConnectionCloseEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <PresentationConnectionCloseEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let event_init_dict = < & PresentationConnectionCloseEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_PresentationConnectionCloseEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <PresentationConnectionCloseEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[cfg(all(
    feature = "PresentationConnectionCloseEvent",
    feature = "PresentationConnectionClosedReason",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reason_PresentationConnectionCloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnectionCloseEvent as WasmDescribe>::describe();
    <PresentationConnectionClosedReason as WasmDescribe>::describe();
}
impl PresentationConnectionCloseEvent {
    #[cfg(all(
        feature = "PresentationConnectionCloseEvent",
        feature = "PresentationConnectionClosedReason",
    ))]
    #[allow(bad_style)]
    #[doc = "The `reason` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionClosedReason`*"]
    #[allow(clippy::all)]
    pub fn reason(&self) -> PresentationConnectionClosedReason {
        #[cfg(all(
            feature = "PresentationConnectionCloseEvent",
            feature = "PresentationConnectionClosedReason",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reason_PresentationConnectionCloseEvent(
                self_ : < & PresentationConnectionCloseEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PresentationConnectionClosedReason as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reason_PresentationConnectionCloseEvent(
            self_: <&PresentationConnectionCloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PresentationConnectionClosedReason as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & PresentationConnectionCloseEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_reason_PresentationConnectionCloseEvent(self_)
            };
            <PresentationConnectionClosedReason as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            )
        }
    }
}
#[cfg(all(feature = "PresentationConnectionCloseEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_message_PresentationConnectionCloseEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnectionCloseEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PresentationConnectionCloseEvent {
    #[cfg(all(feature = "PresentationConnectionCloseEvent",))]
    #[allow(bad_style)]
    #[doc = "The `message` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)\n\n*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
    #[allow(clippy::all)]
    pub fn message(&self) -> String {
        #[cfg(all(feature = "PresentationConnectionCloseEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_message_PresentationConnectionCloseEvent(
                self_ : < & PresentationConnectionCloseEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_message_PresentationConnectionCloseEvent(
            self_: <&PresentationConnectionCloseEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & PresentationConnectionCloseEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_message_PresentationConnectionCloseEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_52c41e1b5d3f76e3: [u8; 539usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD9\x01\0\0\0\0\x04\0\0\x02 PresentationConnectionCloseEvent2__widl_instanceof_PresentationConnectionCloseEvent\0\0\0\0-__widl_f_new_PresentationConnectionCloseEvent\x01\0\0\x01 PresentationConnectionCloseEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\00__widl_f_reason_PresentationConnectionCloseEvent\0\0\0\x01 PresentationConnectionCloseEvent\x01\0\x01\x06reason\x01\x01\x05self_\x06reason\0\0\01__widl_f_message_PresentationConnectionCloseEvent\0\0\0\x01 PresentationConnectionCloseEvent\x01\0\x01\x07message\x01\x01\x05self_\x07message\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

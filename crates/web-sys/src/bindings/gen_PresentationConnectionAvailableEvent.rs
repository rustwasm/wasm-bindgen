use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PresentationConnectionAvailableEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PresentationConnectionAvailableEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PresentationConnectionAvailableEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PresentationConnectionAvailableEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(36u32);
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
            inform(65u32);
            inform(118u32);
            inform(97u32);
            inform(105u32);
            inform(108u32);
            inform(97u32);
            inform(98u32);
            inform(108u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PresentationConnectionAvailableEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for PresentationConnectionAvailableEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PresentationConnectionAvailableEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PresentationConnectionAvailableEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PresentationConnectionAvailableEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PresentationConnectionAvailableEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PresentationConnectionAvailableEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PresentationConnectionAvailableEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PresentationConnectionAvailableEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PresentationConnectionAvailableEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PresentationConnectionAvailableEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PresentationConnectionAvailableEvent {
        #[inline]
        fn from(obj: JsValue) -> PresentationConnectionAvailableEvent {
            PresentationConnectionAvailableEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PresentationConnectionAvailableEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PresentationConnectionAvailableEvent> for PresentationConnectionAvailableEvent {
        #[inline]
        fn as_ref(&self) -> &PresentationConnectionAvailableEvent {
            self
        }
    }
    impl From<PresentationConnectionAvailableEvent> for JsValue {
        #[inline]
        fn from(obj: PresentationConnectionAvailableEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PresentationConnectionAvailableEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PresentationConnectionAvailableEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PresentationConnectionAvailableEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PresentationConnectionAvailableEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PresentationConnectionAvailableEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PresentationConnectionAvailableEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PresentationConnectionAvailableEvent> for Event {
    #[inline]
    fn from(obj: PresentationConnectionAvailableEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PresentationConnectionAvailableEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PresentationConnectionAvailableEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PresentationConnectionAvailableEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PresentationConnectionAvailableEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "PresentationConnectionAvailableEvent",
    feature = "PresentationConnectionAvailableEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PresentationConnectionAvailableEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PresentationConnectionAvailableEventInit as WasmDescribe>::describe();
    <PresentationConnectionAvailableEvent as WasmDescribe>::describe();
}
impl PresentationConnectionAvailableEvent {
    #[cfg(all(
        feature = "PresentationConnectionAvailableEvent",
        feature = "PresentationConnectionAvailableEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new instance of `PresentationConnectionAvailableEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/PresentationConnectionAvailableEvent)\n\n*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`, `PresentationConnectionAvailableEventInit`*"]
    #[allow(clippy::all)]
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionAvailableEventInit,
    ) -> Result<PresentationConnectionAvailableEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "PresentationConnectionAvailableEvent",
            feature = "PresentationConnectionAvailableEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PresentationConnectionAvailableEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & PresentationConnectionAvailableEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PresentationConnectionAvailableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PresentationConnectionAvailableEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & PresentationConnectionAvailableEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <PresentationConnectionAvailableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
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
                let event_init_dict = < & PresentationConnectionAvailableEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( event_init_dict ) ;
                __widl_f_new_PresentationConnectionAvailableEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok ( < PresentationConnectionAvailableEvent as wasm_bindgen :: convert :: FromWasmAbi > :: from_abi ( _ret ) )
        }
    }
}
#[cfg(all(
    feature = "PresentationConnection",
    feature = "PresentationConnectionAvailableEvent",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connection_PresentationConnectionAvailableEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnectionAvailableEvent as WasmDescribe>::describe();
    <PresentationConnection as WasmDescribe>::describe();
}
impl PresentationConnectionAvailableEvent {
    #[cfg(all(
        feature = "PresentationConnection",
        feature = "PresentationConnectionAvailableEvent",
    ))]
    #[allow(bad_style)]
    #[doc = "The `connection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEvent`*"]
    #[allow(clippy::all)]
    pub fn connection(&self) -> PresentationConnection {
        #[cfg(all(
            feature = "PresentationConnection",
            feature = "PresentationConnectionAvailableEvent",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connection_PresentationConnectionAvailableEvent(
                self_ : < & PresentationConnectionAvailableEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <PresentationConnection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connection_PresentationConnectionAvailableEvent(
            self_ : < & PresentationConnectionAvailableEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <PresentationConnection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & PresentationConnectionAvailableEvent as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_connection_PresentationConnectionAvailableEvent(self_)
            };
            <PresentationConnection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f3348a2f40ef4726: [u8; 458usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x88\x01\0\0\0\0\x03\0\0\x02$PresentationConnectionAvailableEvent6__widl_instanceof_PresentationConnectionAvailableEvent\0\0\0\01__widl_f_new_PresentationConnectionAvailableEvent\x01\0\0\x01$PresentationConnectionAvailableEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\08__widl_f_connection_PresentationConnectionAvailableEvent\0\0\0\x01$PresentationConnectionAvailableEvent\x01\0\x01\nconnection\x01\x01\x05self_\nconnection\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

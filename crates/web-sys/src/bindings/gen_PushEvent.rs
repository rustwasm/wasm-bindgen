use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PushEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PushEvent {
    obj: ExtendableEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PushEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PushEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(80u32);
            inform(117u32);
            inform(115u32);
            inform(104u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PushEvent {
        type Target = ExtendableEvent;
        #[inline]
        fn deref(&self) -> &ExtendableEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for PushEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PushEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PushEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PushEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PushEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PushEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PushEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PushEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PushEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PushEvent {
        #[inline]
        fn from(obj: JsValue) -> PushEvent {
            PushEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PushEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PushEvent> for PushEvent {
        #[inline]
        fn as_ref(&self) -> &PushEvent {
            self
        }
    }
    impl From<PushEvent> for JsValue {
        #[inline]
        fn from(obj: PushEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PushEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PushEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PushEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PushEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PushEvent> for ExtendableEvent {
    #[inline]
    fn from(obj: PushEvent) -> ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<ExtendableEvent> for PushEvent {
    #[inline]
    fn as_ref(&self) -> &ExtendableEvent {
        use wasm_bindgen::JsCast;
        ExtendableEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PushEvent> for Event {
    #[inline]
    fn from(obj: PushEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for PushEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PushEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: PushEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PushEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PushEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PushEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PushEvent as WasmDescribe>::describe();
}
impl PushEvent {
    #[cfg(all(feature = "PushEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<PushEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PushEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PushEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PushEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PushEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_PushEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PushEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "PushEvent", feature = "PushEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_PushEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&PushEventInit as WasmDescribe>::describe();
    <PushEvent as WasmDescribe>::describe();
}
impl PushEvent {
    #[cfg(all(feature = "PushEvent", feature = "PushEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)\n\n*This API requires the following crate features to be activated: `PushEvent`, `PushEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PushEventInit,
    ) -> Result<PushEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PushEvent", feature = "PushEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_PushEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&PushEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PushEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_PushEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&PushEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PushEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PushEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_PushEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PushEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "PushEvent", feature = "PushMessageData",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_PushEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PushEvent as WasmDescribe>::describe();
    <Option<PushMessageData> as WasmDescribe>::describe();
}
impl PushEvent {
    #[cfg(all(feature = "PushEvent", feature = "PushMessageData",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/data)\n\n*This API requires the following crate features to be activated: `PushEvent`, `PushMessageData`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> Option<PushMessageData> {
        #[cfg(all(feature = "PushEvent", feature = "PushMessageData",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_PushEvent(
                self_: <&PushEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<PushMessageData> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_PushEvent(
            self_: <&PushEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<PushMessageData> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PushEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_PushEvent(self_)
            };
            <Option<PushMessageData> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_03c981814c9529a4: [u8; 352usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1E\x01\0\0\0\0\x04\0\0\x02\tPushEvent\x1B__widl_instanceof_PushEvent\0\0\0\0\x16__widl_f_new_PushEvent\x01\0\0\x01\tPushEvent\0\x01\x01\x05type_\x03new\0\0\0+__widl_f_new_with_event_init_dict_PushEvent\x01\0\0\x01\tPushEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x17__widl_f_data_PushEvent\0\0\0\x01\tPushEvent\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

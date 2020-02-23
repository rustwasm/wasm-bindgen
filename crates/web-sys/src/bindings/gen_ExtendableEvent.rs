use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ExtendableEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtendableEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtendableEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtendableEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(69u32);
            inform(120u32);
            inform(116u32);
            inform(101u32);
            inform(110u32);
            inform(100u32);
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
    impl core::ops::Deref for ExtendableEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtendableEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtendableEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtendableEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtendableEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtendableEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtendableEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtendableEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtendableEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtendableEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtendableEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtendableEvent {
        #[inline]
        fn from(obj: JsValue) -> ExtendableEvent {
            ExtendableEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtendableEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtendableEvent> for ExtendableEvent {
        #[inline]
        fn as_ref(&self) -> &ExtendableEvent {
            self
        }
    }
    impl From<ExtendableEvent> for JsValue {
        #[inline]
        fn from(obj: ExtendableEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtendableEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ExtendableEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ExtendableEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ExtendableEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtendableEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtendableEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtendableEvent> for Event {
    #[inline]
    fn from(obj: ExtendableEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for ExtendableEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ExtendableEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtendableEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtendableEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ExtendableEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ExtendableEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ExtendableEvent as WasmDescribe>::describe();
}
impl ExtendableEvent {
    #[cfg(all(feature = "ExtendableEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<ExtendableEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ExtendableEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ExtendableEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ExtendableEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_ExtendableEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ExtendableEvent", feature = "ExtendableEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_ExtendableEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&ExtendableEventInit as WasmDescribe>::describe();
    <ExtendableEvent as WasmDescribe>::describe();
}
impl ExtendableEvent {
    #[cfg(all(feature = "ExtendableEvent", feature = "ExtendableEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`, `ExtendableEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ExtendableEventInit,
    ) -> Result<ExtendableEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ExtendableEvent", feature = "ExtendableEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_ExtendableEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&ExtendableEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_ExtendableEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&ExtendableEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ExtendableEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_ExtendableEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ExtendableEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ExtendableEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_wait_until_ExtendableEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ExtendableEvent as WasmDescribe>::describe();
    <&::js_sys::Promise as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ExtendableEvent {
    #[cfg(all(feature = "ExtendableEvent",))]
    #[allow(bad_style)]
    #[doc = "The `waitUntil()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/waitUntil)\n\n*This API requires the following crate features to be activated: `ExtendableEvent`*"]
    #[allow(clippy::all)]
    pub fn wait_until(&self, p: &::js_sys::Promise) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ExtendableEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_wait_until_ExtendableEvent(
                self_: <&ExtendableEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                p: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_wait_until_ExtendableEvent(
            self_: <&ExtendableEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            p: <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(p);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtendableEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let p = <&::js_sys::Promise as wasm_bindgen::convert::IntoWasmAbi>::into_abi(p);
                __widl_f_wait_until_ExtendableEvent(self_, p)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1c97c8021b337acf: [u8; 408usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}V\x01\0\0\0\0\x04\0\0\x02\x0FExtendableEvent!__widl_instanceof_ExtendableEvent\0\0\0\0\x1C__widl_f_new_ExtendableEvent\x01\0\0\x01\x0FExtendableEvent\0\x01\x01\x05type_\x03new\0\0\01__widl_f_new_with_event_init_dict_ExtendableEvent\x01\0\0\x01\x0FExtendableEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0#__widl_f_wait_until_ExtendableEvent\x01\0\0\x01\x0FExtendableEvent\x01\0\0\x01\x02\x05self_\x01p\twaitUntil\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

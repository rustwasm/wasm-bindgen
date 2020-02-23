use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceLightEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceLightEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceLightEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceLightEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(76u32);
            inform(105u32);
            inform(103u32);
            inform(104u32);
            inform(116u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DeviceLightEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceLightEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceLightEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceLightEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceLightEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceLightEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceLightEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceLightEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceLightEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceLightEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceLightEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceLightEvent {
        #[inline]
        fn from(obj: JsValue) -> DeviceLightEvent {
            DeviceLightEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceLightEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceLightEvent> for DeviceLightEvent {
        #[inline]
        fn as_ref(&self) -> &DeviceLightEvent {
            self
        }
    }
    impl From<DeviceLightEvent> for JsValue {
        #[inline]
        fn from(obj: DeviceLightEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceLightEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceLightEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceLightEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceLightEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceLightEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceLightEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceLightEvent> for Event {
    #[inline]
    fn from(obj: DeviceLightEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for DeviceLightEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DeviceLightEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceLightEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceLightEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceLightEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DeviceLightEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DeviceLightEvent as WasmDescribe>::describe();
}
impl DeviceLightEvent {
    #[cfg(all(feature = "DeviceLightEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<DeviceLightEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceLightEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DeviceLightEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DeviceLightEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_DeviceLightEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceLightEvent", feature = "DeviceLightEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_DeviceLightEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&DeviceLightEventInit as WasmDescribe>::describe();
    <DeviceLightEvent as WasmDescribe>::describe();
}
impl DeviceLightEvent {
    #[cfg(all(feature = "DeviceLightEvent", feature = "DeviceLightEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`, `DeviceLightEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceLightEventInit,
    ) -> Result<DeviceLightEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceLightEvent", feature = "DeviceLightEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_DeviceLightEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&DeviceLightEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_DeviceLightEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&DeviceLightEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DeviceLightEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_DeviceLightEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceLightEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceLightEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_DeviceLightEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceLightEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DeviceLightEvent {
    #[cfg(all(feature = "DeviceLightEvent",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/value)\n\n*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "DeviceLightEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_DeviceLightEvent(
                self_: <&DeviceLightEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_DeviceLightEvent(
            self_: <&DeviceLightEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceLightEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_DeviceLightEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_fb89a6c4783f2915: [u8; 411usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Y\x01\0\0\0\0\x04\0\0\x02\x10DeviceLightEvent\"__widl_instanceof_DeviceLightEvent\0\0\0\0\x1D__widl_f_new_DeviceLightEvent\x01\0\0\x01\x10DeviceLightEvent\0\x01\x01\x05type_\x03new\0\0\02__widl_f_new_with_event_init_dict_DeviceLightEvent\x01\0\0\x01\x10DeviceLightEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1F__widl_f_value_DeviceLightEvent\0\0\0\x01\x10DeviceLightEvent\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

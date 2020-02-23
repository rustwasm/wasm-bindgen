use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceProximityEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceProximityEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceProximityEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceProximityEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(120u32);
            inform(105u32);
            inform(109u32);
            inform(105u32);
            inform(116u32);
            inform(121u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DeviceProximityEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceProximityEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceProximityEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceProximityEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceProximityEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceProximityEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceProximityEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceProximityEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceProximityEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceProximityEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceProximityEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceProximityEvent {
        #[inline]
        fn from(obj: JsValue) -> DeviceProximityEvent {
            DeviceProximityEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceProximityEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceProximityEvent> for DeviceProximityEvent {
        #[inline]
        fn as_ref(&self) -> &DeviceProximityEvent {
            self
        }
    }
    impl From<DeviceProximityEvent> for JsValue {
        #[inline]
        fn from(obj: DeviceProximityEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceProximityEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceProximityEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceProximityEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceProximityEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceProximityEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceProximityEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceProximityEvent> for Event {
    #[inline]
    fn from(obj: DeviceProximityEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for DeviceProximityEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DeviceProximityEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceProximityEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceProximityEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceProximityEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DeviceProximityEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DeviceProximityEvent as WasmDescribe>::describe();
}
impl DeviceProximityEvent {
    #[cfg(all(feature = "DeviceProximityEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<DeviceProximityEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceProximityEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DeviceProximityEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DeviceProximityEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_DeviceProximityEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceProximityEvent", feature = "DeviceProximityEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_DeviceProximityEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&DeviceProximityEventInit as WasmDescribe>::describe();
    <DeviceProximityEvent as WasmDescribe>::describe();
}
impl DeviceProximityEvent {
    #[cfg(all(feature = "DeviceProximityEvent", feature = "DeviceProximityEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`, `DeviceProximityEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceProximityEventInit,
    ) -> Result<DeviceProximityEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceProximityEvent", feature = "DeviceProximityEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_DeviceProximityEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & DeviceProximityEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_DeviceProximityEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&DeviceProximityEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DeviceProximityEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_DeviceProximityEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceProximityEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceProximityEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_DeviceProximityEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceProximityEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DeviceProximityEvent {
    #[cfg(all(feature = "DeviceProximityEvent",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/value)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "DeviceProximityEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_DeviceProximityEvent(
                self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_DeviceProximityEvent(
            self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_DeviceProximityEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceProximityEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_DeviceProximityEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceProximityEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DeviceProximityEvent {
    #[cfg(all(feature = "DeviceProximityEvent",))]
    #[allow(bad_style)]
    #[doc = "The `min` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/min)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    #[allow(clippy::all)]
    pub fn min(&self) -> f64 {
        #[cfg(all(feature = "DeviceProximityEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_DeviceProximityEvent(
                self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_DeviceProximityEvent(
            self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_DeviceProximityEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceProximityEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_DeviceProximityEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceProximityEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl DeviceProximityEvent {
    #[cfg(all(feature = "DeviceProximityEvent",))]
    #[allow(bad_style)]
    #[doc = "The `max` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/max)\n\n*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    #[allow(clippy::all)]
    pub fn max(&self) -> f64 {
        #[cfg(all(feature = "DeviceProximityEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_DeviceProximityEvent(
                self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_DeviceProximityEvent(
            self_: <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceProximityEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_DeviceProximityEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_653438890b912529: [u8; 605usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1B\x02\0\0\0\0\x06\0\0\x02\x14DeviceProximityEvent&__widl_instanceof_DeviceProximityEvent\0\0\0\0!__widl_f_new_DeviceProximityEvent\x01\0\0\x01\x14DeviceProximityEvent\0\x01\x01\x05type_\x03new\0\0\06__widl_f_new_with_event_init_dict_DeviceProximityEvent\x01\0\0\x01\x14DeviceProximityEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0#__widl_f_value_DeviceProximityEvent\0\0\0\x01\x14DeviceProximityEvent\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0!__widl_f_min_DeviceProximityEvent\0\0\0\x01\x14DeviceProximityEvent\x01\0\x01\x03min\x01\x01\x05self_\x03min\0\0\0!__widl_f_max_DeviceProximityEvent\0\0\0\x01\x14DeviceProximityEvent\x01\0\x01\x03max\x01\x01\x05self_\x03max\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

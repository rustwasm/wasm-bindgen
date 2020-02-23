use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceMotionEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceMotionEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceMotionEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceMotionEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(77u32);
            inform(111u32);
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
    impl core::ops::Deref for DeviceMotionEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceMotionEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceMotionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceMotionEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceMotionEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceMotionEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceMotionEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceMotionEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceMotionEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceMotionEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceMotionEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceMotionEvent {
        #[inline]
        fn from(obj: JsValue) -> DeviceMotionEvent {
            DeviceMotionEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceMotionEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceMotionEvent> for DeviceMotionEvent {
        #[inline]
        fn as_ref(&self) -> &DeviceMotionEvent {
            self
        }
    }
    impl From<DeviceMotionEvent> for JsValue {
        #[inline]
        fn from(obj: DeviceMotionEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceMotionEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceMotionEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceMotionEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceMotionEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceMotionEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceMotionEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceMotionEvent> for Event {
    #[inline]
    fn from(obj: DeviceMotionEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for DeviceMotionEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DeviceMotionEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceMotionEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceMotionEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceMotionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DeviceMotionEvent as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceMotionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<DeviceMotionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceMotionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DeviceMotionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DeviceMotionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_DeviceMotionEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceMotionEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&DeviceMotionEventInit as WasmDescribe>::describe();
    <DeviceMotionEvent as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceMotionEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceMotionEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceMotionEventInit,
    ) -> Result<DeviceMotionEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceMotionEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_DeviceMotionEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & DeviceMotionEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_DeviceMotionEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&DeviceMotionEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DeviceMotionEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_DeviceMotionEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceMotionEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_acceleration_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceMotionEvent as WasmDescribe>::describe();
    <Option<DeviceAcceleration> as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `acceleration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    #[allow(clippy::all)]
    pub fn acceleration(&self) -> Option<DeviceAcceleration> {
        #[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_acceleration_DeviceMotionEvent(
                self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_acceleration_DeviceMotionEvent(
            self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_acceleration_DeviceMotionEvent(self_)
            };
            <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_acceleration_including_gravity_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceMotionEvent as WasmDescribe>::describe();
    <Option<DeviceAcceleration> as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `accelerationIncludingGravity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)\n\n*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    #[allow(clippy::all)]
    pub fn acceleration_including_gravity(&self) -> Option<DeviceAcceleration> {
        #[cfg(all(feature = "DeviceAcceleration", feature = "DeviceMotionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_acceleration_including_gravity_DeviceMotionEvent(
                self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_acceleration_including_gravity_DeviceMotionEvent(
            self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_acceleration_including_gravity_DeviceMotionEvent(self_)
            };
            <Option<DeviceAcceleration> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceRotationRate",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rotation_rate_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceMotionEvent as WasmDescribe>::describe();
    <Option<DeviceRotationRate> as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceRotationRate",))]
    #[allow(bad_style)]
    #[doc = "The `rotationRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceRotationRate`*"]
    #[allow(clippy::all)]
    pub fn rotation_rate(&self) -> Option<DeviceRotationRate> {
        #[cfg(all(feature = "DeviceMotionEvent", feature = "DeviceRotationRate",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rotation_rate_DeviceMotionEvent(
                self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DeviceRotationRate> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rotation_rate_DeviceMotionEvent(
            self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DeviceRotationRate> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rotation_rate_DeviceMotionEvent(self_)
            };
            <Option<DeviceRotationRate> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceMotionEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_interval_DeviceMotionEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceMotionEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceMotionEvent {
    #[cfg(all(feature = "DeviceMotionEvent",))]
    #[allow(bad_style)]
    #[doc = "The `interval` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)\n\n*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    #[allow(clippy::all)]
    pub fn interval(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceMotionEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_interval_DeviceMotionEvent(
                self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_interval_DeviceMotionEvent(
            self_: <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceMotionEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_interval_DeviceMotionEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5225170a49192ab9: [u8; 785usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCF\x02\0\0\0\0\x07\0\0\x02\x11DeviceMotionEvent#__widl_instanceof_DeviceMotionEvent\0\0\0\0\x1E__widl_f_new_DeviceMotionEvent\x01\0\0\x01\x11DeviceMotionEvent\0\x01\x01\x05type_\x03new\0\0\03__widl_f_new_with_event_init_dict_DeviceMotionEvent\x01\0\0\x01\x11DeviceMotionEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0'__widl_f_acceleration_DeviceMotionEvent\0\0\0\x01\x11DeviceMotionEvent\x01\0\x01\x0Cacceleration\x01\x01\x05self_\x0Cacceleration\0\0\09__widl_f_acceleration_including_gravity_DeviceMotionEvent\0\0\0\x01\x11DeviceMotionEvent\x01\0\x01\x1CaccelerationIncludingGravity\x01\x01\x05self_\x1CaccelerationIncludingGravity\0\0\0(__widl_f_rotation_rate_DeviceMotionEvent\0\0\0\x01\x11DeviceMotionEvent\x01\0\x01\x0CrotationRate\x01\x01\x05self_\x0CrotationRate\0\0\0#__widl_f_interval_DeviceMotionEvent\0\0\0\x01\x11DeviceMotionEvent\x01\0\x01\x08interval\x01\x01\x05self_\x08interval\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

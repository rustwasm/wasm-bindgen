use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DeviceOrientationEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DeviceOrientationEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DeviceOrientationEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DeviceOrientationEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(68u32);
            inform(101u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(79u32);
            inform(114u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
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
    impl core::ops::Deref for DeviceOrientationEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for DeviceOrientationEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DeviceOrientationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceOrientationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DeviceOrientationEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DeviceOrientationEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DeviceOrientationEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceOrientationEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DeviceOrientationEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DeviceOrientationEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DeviceOrientationEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DeviceOrientationEvent {
        #[inline]
        fn from(obj: JsValue) -> DeviceOrientationEvent {
            DeviceOrientationEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DeviceOrientationEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DeviceOrientationEvent> for DeviceOrientationEvent {
        #[inline]
        fn as_ref(&self) -> &DeviceOrientationEvent {
            self
        }
    }
    impl From<DeviceOrientationEvent> for JsValue {
        #[inline]
        fn from(obj: DeviceOrientationEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DeviceOrientationEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DeviceOrientationEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DeviceOrientationEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DeviceOrientationEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceOrientationEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceOrientationEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DeviceOrientationEvent> for Event {
    #[inline]
    fn from(obj: DeviceOrientationEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for DeviceOrientationEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DeviceOrientationEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: DeviceOrientationEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DeviceOrientationEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <DeviceOrientationEvent as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<DeviceOrientationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DeviceOrientationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DeviceOrientationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_DeviceOrientationEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "DeviceOrientationEvent",
    feature = "DeviceOrientationEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&DeviceOrientationEventInit as WasmDescribe>::describe();
    <DeviceOrientationEvent as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(
        feature = "DeviceOrientationEvent",
        feature = "DeviceOrientationEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`, `DeviceOrientationEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceOrientationEventInit,
    ) -> Result<DeviceOrientationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DeviceOrientationEvent",
            feature = "DeviceOrientationEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_DeviceOrientationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & DeviceOrientationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_DeviceOrientationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & DeviceOrientationEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DeviceOrientationEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_DeviceOrientationEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DeviceOrientationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event(&self, type_: &str) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_init_device_orientation_event_DeviceOrientationEvent(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble(&self, type_: &str, can_bubble: bool) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                __widl_f_init_device_orientation_event_with_can_bubble_DeviceOrientationEvent(
                    self_, type_, can_bubble,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    ) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_DeviceOrientationEvent ( self_ , type_ , can_bubble , cancelable )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
    ) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(alpha);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let alpha = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_DeviceOrientationEvent ( self_ , type_ , can_bubble , cancelable , alpha )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
    ) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(alpha);
            drop(beta);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let alpha = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                let beta = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(beta);
                __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_DeviceOrientationEvent ( self_ , type_ , can_bubble , cancelable , alpha , beta )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
        gamma: Option<f64>,
    ) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                gamma: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            gamma: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(alpha);
            drop(beta);
            drop(gamma);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let alpha = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                let beta = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(beta);
                let gamma = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(gamma);
                __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_DeviceOrientationEvent ( self_ , type_ , can_bubble , cancelable , alpha , beta , gamma )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute_DeviceOrientationEvent(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initDeviceOrientationEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute(
        &self,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        alpha: Option<f64>,
        beta: Option<f64>,
        gamma: Option<f64>,
        absolute: bool,
    ) {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                gamma: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                absolute: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            can_bubble: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cancelable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alpha: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            beta: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            gamma: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            absolute: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            drop(can_bubble);
            drop(cancelable);
            drop(alpha);
            drop(beta);
            drop(gamma);
            drop(absolute);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let can_bubble = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(can_bubble);
                let cancelable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cancelable);
                let alpha = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alpha);
                let beta = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(beta);
                let gamma = <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(gamma);
                let absolute = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(absolute);
                __widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute_DeviceOrientationEvent ( self_ , type_ , can_bubble , cancelable , alpha , beta , gamma , absolute )
            };
            ()
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alpha_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `alpha` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/alpha)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn alpha(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alpha_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alpha_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alpha_DeviceOrientationEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_beta_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `beta` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/beta)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn beta(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_beta_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_beta_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_beta_DeviceOrientationEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_gamma_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `gamma` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/gamma)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn gamma(&self) -> Option<f64> {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_gamma_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_gamma_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_gamma_DeviceOrientationEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DeviceOrientationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_absolute_DeviceOrientationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DeviceOrientationEvent as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DeviceOrientationEvent {
    #[cfg(all(feature = "DeviceOrientationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `absolute` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/absolute)\n\n*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
    #[allow(clippy::all)]
    pub fn absolute(&self) -> bool {
        #[cfg(all(feature = "DeviceOrientationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_absolute_DeviceOrientationEvent(
                self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_absolute_DeviceOrientationEvent(
            self_: <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DeviceOrientationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_absolute_DeviceOrientationEvent(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_76db1b9d4b3b9c6f: [u8; 2143usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1D\x08\0\0\0\0\x0E\0\0\x02\x16DeviceOrientationEvent(__widl_instanceof_DeviceOrientationEvent\0\0\0\0#__widl_f_new_DeviceOrientationEvent\x01\0\0\x01\x16DeviceOrientationEvent\0\x01\x01\x05type_\x03new\0\0\08__widl_f_new_with_event_init_dict_DeviceOrientationEvent\x01\0\0\x01\x16DeviceOrientationEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0=__widl_f_init_device_orientation_event_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x02\x05self_\x05type_\x1AinitDeviceOrientationEvent\0\0\0M__widl_f_init_device_orientation_event_with_can_bubble_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x03\x05self_\x05type_\ncan_bubble\x1AinitDeviceOrientationEvent\0\0\0\\__widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x04\x05self_\x05type_\ncan_bubble\ncancelable\x1AinitDeviceOrientationEvent\0\0\0f__widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x05\x05self_\x05type_\ncan_bubble\ncancelable\x05alpha\x1AinitDeviceOrientationEvent\0\0\0o__widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x06\x05self_\x05type_\ncan_bubble\ncancelable\x05alpha\x04beta\x1AinitDeviceOrientationEvent\0\0\0y__widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x07\x05self_\x05type_\ncan_bubble\ncancelable\x05alpha\x04beta\x05gamma\x1AinitDeviceOrientationEvent\0\0\0\x86\x01__widl_f_init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\0\x01\x08\x05self_\x05type_\ncan_bubble\ncancelable\x05alpha\x04beta\x05gamma\x08absolute\x1AinitDeviceOrientationEvent\0\0\0%__widl_f_alpha_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\x01\x05alpha\x01\x01\x05self_\x05alpha\0\0\0$__widl_f_beta_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\x01\x04beta\x01\x01\x05self_\x04beta\0\0\0%__widl_f_gamma_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\x01\x05gamma\x01\x01\x05self_\x05gamma\0\0\0(__widl_f_absolute_DeviceOrientationEvent\0\0\0\x01\x16DeviceOrientationEvent\x01\0\x01\x08absolute\x01\x01\x05self_\x08absolute\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

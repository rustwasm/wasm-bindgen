use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WheelEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WheelEvent {
    obj: MouseEvent,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WheelEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WheelEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(87u32);
            inform(104u32);
            inform(101u32);
            inform(101u32);
            inform(108u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WheelEvent {
        type Target = MouseEvent;
        #[inline]
        fn deref(&self) -> &MouseEvent {
            &self.obj
        }
    }
    impl IntoWasmAbi for WheelEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WheelEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WheelEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WheelEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WheelEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WheelEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WheelEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WheelEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WheelEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WheelEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WheelEvent {
        #[inline]
        fn from(obj: JsValue) -> WheelEvent {
            WheelEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WheelEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WheelEvent> for WheelEvent {
        #[inline]
        fn as_ref(&self) -> &WheelEvent {
            self
        }
    }
    impl From<WheelEvent> for JsValue {
        #[inline]
        fn from(obj: WheelEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WheelEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WheelEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WheelEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WheelEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WheelEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WheelEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WheelEvent> for MouseEvent {
    #[inline]
    fn from(obj: WheelEvent) -> MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<MouseEvent> for WheelEvent {
    #[inline]
    fn as_ref(&self) -> &MouseEvent {
        use wasm_bindgen::JsCast;
        MouseEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WheelEvent> for UiEvent {
    #[inline]
    fn from(obj: WheelEvent) -> UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<UiEvent> for WheelEvent {
    #[inline]
    fn as_ref(&self) -> &UiEvent {
        use wasm_bindgen::JsCast;
        UiEvent::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WheelEvent> for Event {
    #[inline]
    fn from(obj: WheelEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for WheelEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WheelEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: WheelEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WheelEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WheelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <WheelEvent as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<WheelEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WheelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_WheelEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WheelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_WheelEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WheelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_WheelEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WheelEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WheelEvent", feature = "WheelEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&WheelEventInit as WasmDescribe>::describe();
    <WheelEvent as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent", feature = "WheelEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)\n\n*This API requires the following crate features to be activated: `WheelEvent`, `WheelEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &WheelEventInit,
    ) -> Result<WheelEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WheelEvent", feature = "WheelEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_WheelEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&WheelEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WheelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_WheelEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&WheelEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WheelEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&WheelEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_WheelEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WheelEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WheelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delta_x_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WheelEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `deltaX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    #[allow(clippy::all)]
    pub fn delta_x(&self) -> f64 {
        #[cfg(all(feature = "WheelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delta_x_WheelEvent(
                self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delta_x_WheelEvent(
            self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delta_x_WheelEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WheelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delta_y_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WheelEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `deltaY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    #[allow(clippy::all)]
    pub fn delta_y(&self) -> f64 {
        #[cfg(all(feature = "WheelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delta_y_WheelEvent(
                self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delta_y_WheelEvent(
            self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delta_y_WheelEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WheelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delta_z_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WheelEvent as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `deltaZ` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    #[allow(clippy::all)]
    pub fn delta_z(&self) -> f64 {
        #[cfg(all(feature = "WheelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delta_z_WheelEvent(
                self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delta_z_WheelEvent(
            self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delta_z_WheelEvent(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WheelEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delta_mode_WheelEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WheelEvent as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl WheelEvent {
    #[cfg(all(feature = "WheelEvent",))]
    #[allow(bad_style)]
    #[doc = "The `deltaMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)\n\n*This API requires the following crate features to be activated: `WheelEvent`*"]
    #[allow(clippy::all)]
    pub fn delta_mode(&self) -> u32 {
        #[cfg(all(feature = "WheelEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delta_mode_WheelEvent(
                self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delta_mode_WheelEvent(
            self_: <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WheelEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delta_mode_WheelEvent(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl WheelEvent {
    pub const DOM_DELTA_PIXEL: u32 = 0u64 as u32;
}
impl WheelEvent {
    pub const DOM_DELTA_LINE: u32 = 1u64 as u32;
}
impl WheelEvent {
    pub const DOM_DELTA_PAGE: u32 = 2u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9aa7830f4f19094a: [u8; 589usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0B\x02\0\0\0\0\x07\0\0\x02\nWheelEvent\x1C__widl_instanceof_WheelEvent\0\0\0\0\x17__widl_f_new_WheelEvent\x01\0\0\x01\nWheelEvent\0\x01\x01\x05type_\x03new\0\0\0,__widl_f_new_with_event_init_dict_WheelEvent\x01\0\0\x01\nWheelEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0\x1B__widl_f_delta_x_WheelEvent\0\0\0\x01\nWheelEvent\x01\0\x01\x06deltaX\x01\x01\x05self_\x06deltaX\0\0\0\x1B__widl_f_delta_y_WheelEvent\0\0\0\x01\nWheelEvent\x01\0\x01\x06deltaY\x01\x01\x05self_\x06deltaY\0\0\0\x1B__widl_f_delta_z_WheelEvent\0\0\0\x01\nWheelEvent\x01\0\x01\x06deltaZ\x01\x01\x05self_\x06deltaZ\0\0\0\x1E__widl_f_delta_mode_WheelEvent\0\0\0\x01\nWheelEvent\x01\0\x01\tdeltaMode\x01\x01\x05self_\tdeltaMode\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

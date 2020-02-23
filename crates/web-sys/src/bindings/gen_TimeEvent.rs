use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TimeEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TimeEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TimeEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TimeEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for TimeEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for TimeEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TimeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TimeEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TimeEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TimeEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TimeEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TimeEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TimeEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TimeEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TimeEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TimeEvent {
        #[inline]
        fn from(obj: JsValue) -> TimeEvent {
            TimeEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TimeEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TimeEvent> for TimeEvent {
        #[inline]
        fn as_ref(&self) -> &TimeEvent {
            self
        }
    }
    impl From<TimeEvent> for JsValue {
        #[inline]
        fn from(obj: TimeEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TimeEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TimeEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TimeEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TimeEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TimeEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TimeEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TimeEvent> for Event {
    #[inline]
    fn from(obj: TimeEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for TimeEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TimeEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: TimeEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TimeEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TimeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_time_event_TimeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TimeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TimeEvent {
    #[cfg(all(feature = "TimeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `initTimeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
    #[allow(clippy::all)]
    pub fn init_time_event(&self, a_type: &str) {
        #[cfg(all(feature = "TimeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_time_event_TimeEvent(
                self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_time_event_TimeEvent(
            self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                __widl_f_init_time_event_TimeEvent(self_, a_type)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TimeEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_time_event_with_a_view_TimeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&TimeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TimeEvent {
    #[cfg(all(feature = "TimeEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTimeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_time_event_with_a_view(&self, a_type: &str, a_view: Option<&Window>) {
        #[cfg(all(feature = "TimeEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_time_event_with_a_view_TimeEvent(
                self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_time_event_with_a_view_TimeEvent(
            self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_view);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                __widl_f_init_time_event_with_a_view_TimeEvent(self_, a_type, a_view)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TimeEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_init_time_event_with_a_view_and_a_detail_TimeEvent()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&TimeEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<&Window> as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TimeEvent {
    #[cfg(all(feature = "TimeEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `initTimeEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn init_time_event_with_a_view_and_a_detail(
        &self,
        a_type: &str,
        a_view: Option<&Window>,
        a_detail: i32,
    ) {
        #[cfg(all(feature = "TimeEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_init_time_event_with_a_view_and_a_detail_TimeEvent(
                self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_init_time_event_with_a_view_and_a_detail_TimeEvent(
            self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_view: <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_detail: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_type);
            drop(a_view);
            drop(a_detail);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_type);
                let a_view =
                    <Option<&Window> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_view);
                let a_detail = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_detail);
                __widl_f_init_time_event_with_a_view_and_a_detail_TimeEvent(
                    self_, a_type, a_view, a_detail,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "TimeEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detail_TimeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TimeEvent as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl TimeEvent {
    #[cfg(all(feature = "TimeEvent",))]
    #[allow(bad_style)]
    #[doc = "The `detail` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)\n\n*This API requires the following crate features to be activated: `TimeEvent`*"]
    #[allow(clippy::all)]
    pub fn detail(&self) -> i32 {
        #[cfg(all(feature = "TimeEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detail_TimeEvent(
                self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detail_TimeEvent(
            self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detail_TimeEvent(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TimeEvent", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_view_TimeEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TimeEvent as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl TimeEvent {
    #[cfg(all(feature = "TimeEvent", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `view` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)\n\n*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
    #[allow(clippy::all)]
    pub fn view(&self) -> Option<Window> {
        #[cfg(all(feature = "TimeEvent", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_view_TimeEvent(
                self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_view_TimeEvent(
            self_: <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TimeEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_view_TimeEvent(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_551e5606f2af8aca: [u8; 589usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0B\x02\0\0\0\0\x06\0\0\x02\tTimeEvent\x1B__widl_instanceof_TimeEvent\0\0\0\0\"__widl_f_init_time_event_TimeEvent\0\0\0\x01\tTimeEvent\x01\0\0\x01\x02\x05self_\x06a_type\rinitTimeEvent\0\0\0.__widl_f_init_time_event_with_a_view_TimeEvent\0\0\0\x01\tTimeEvent\x01\0\0\x01\x03\x05self_\x06a_type\x06a_view\rinitTimeEvent\0\0\0;__widl_f_init_time_event_with_a_view_and_a_detail_TimeEvent\0\0\0\x01\tTimeEvent\x01\0\0\x01\x04\x05self_\x06a_type\x06a_view\x08a_detail\rinitTimeEvent\0\0\0\x19__widl_f_detail_TimeEvent\0\0\0\x01\tTimeEvent\x01\0\x01\x06detail\x01\x01\x05self_\x06detail\0\0\0\x17__widl_f_view_TimeEvent\0\0\0\x01\tTimeEvent\x01\0\x01\x04view\x01\x01\x05self_\x04view\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

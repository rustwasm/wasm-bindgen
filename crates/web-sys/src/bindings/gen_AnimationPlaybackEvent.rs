use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AnimationPlaybackEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent)\n\n*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AnimationPlaybackEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AnimationPlaybackEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AnimationPlaybackEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(80u32);
            inform(108u32);
            inform(97u32);
            inform(121u32);
            inform(98u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for AnimationPlaybackEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for AnimationPlaybackEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AnimationPlaybackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AnimationPlaybackEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AnimationPlaybackEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AnimationPlaybackEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AnimationPlaybackEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AnimationPlaybackEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AnimationPlaybackEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AnimationPlaybackEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AnimationPlaybackEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AnimationPlaybackEvent {
        #[inline]
        fn from(obj: JsValue) -> AnimationPlaybackEvent {
            AnimationPlaybackEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AnimationPlaybackEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AnimationPlaybackEvent> for AnimationPlaybackEvent {
        #[inline]
        fn as_ref(&self) -> &AnimationPlaybackEvent {
            self
        }
    }
    impl From<AnimationPlaybackEvent> for JsValue {
        #[inline]
        fn from(obj: AnimationPlaybackEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AnimationPlaybackEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AnimationPlaybackEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AnimationPlaybackEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AnimationPlaybackEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AnimationPlaybackEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AnimationPlaybackEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AnimationPlaybackEvent> for Event {
    #[inline]
    fn from(obj: AnimationPlaybackEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for AnimationPlaybackEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AnimationPlaybackEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: AnimationPlaybackEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AnimationPlaybackEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AnimationPlaybackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AnimationPlaybackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <AnimationPlaybackEvent as WasmDescribe>::describe();
}
impl AnimationPlaybackEvent {
    #[cfg(all(feature = "AnimationPlaybackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)\n\n*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<AnimationPlaybackEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnimationPlaybackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AnimationPlaybackEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AnimationPlaybackEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_AnimationPlaybackEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AnimationPlaybackEvent",
    feature = "AnimationPlaybackEventInit",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_AnimationPlaybackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&AnimationPlaybackEventInit as WasmDescribe>::describe();
    <AnimationPlaybackEvent as WasmDescribe>::describe();
}
impl AnimationPlaybackEvent {
    #[cfg(all(
        feature = "AnimationPlaybackEvent",
        feature = "AnimationPlaybackEventInit",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)\n\n*This API requires the following crate features to be activated: `AnimationPlaybackEvent`, `AnimationPlaybackEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &AnimationPlaybackEventInit,
    ) -> Result<AnimationPlaybackEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AnimationPlaybackEvent",
            feature = "AnimationPlaybackEventInit",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_AnimationPlaybackEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict : < & AnimationPlaybackEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_AnimationPlaybackEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict : < & AnimationPlaybackEventInit as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&AnimationPlaybackEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_AnimationPlaybackEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnimationPlaybackEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AnimationPlaybackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_AnimationPlaybackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationPlaybackEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl AnimationPlaybackEvent {
    #[cfg(all(feature = "AnimationPlaybackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/currentTime)\n\n*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> Option<f64> {
        #[cfg(all(feature = "AnimationPlaybackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_AnimationPlaybackEvent(
                self_: <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_AnimationPlaybackEvent(
            self_: <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_AnimationPlaybackEvent(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnimationPlaybackEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timeline_time_AnimationPlaybackEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationPlaybackEvent as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl AnimationPlaybackEvent {
    #[cfg(all(feature = "AnimationPlaybackEvent",))]
    #[allow(bad_style)]
    #[doc = "The `timelineTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/timelineTime)\n\n*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
    #[allow(clippy::all)]
    pub fn timeline_time(&self) -> Option<f64> {
        #[cfg(all(feature = "AnimationPlaybackEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timeline_time_AnimationPlaybackEvent(
                self_: <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timeline_time_AnimationPlaybackEvent(
            self_: <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AnimationPlaybackEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timeline_time_AnimationPlaybackEvent(self_)
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
pub static __WASM_BINDGEN_GENERATED_4d0e9954e88c2833: [u8; 591usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\r\x02\0\0\0\0\x05\0\0\x02\x16AnimationPlaybackEvent(__widl_instanceof_AnimationPlaybackEvent\0\0\0\0#__widl_f_new_AnimationPlaybackEvent\x01\0\0\x01\x16AnimationPlaybackEvent\0\x01\x01\x05type_\x03new\0\0\08__widl_f_new_with_event_init_dict_AnimationPlaybackEvent\x01\0\0\x01\x16AnimationPlaybackEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0,__widl_f_current_time_AnimationPlaybackEvent\0\0\0\x01\x16AnimationPlaybackEvent\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0-__widl_f_timeline_time_AnimationPlaybackEvent\0\0\0\x01\x16AnimationPlaybackEvent\x01\0\x01\x0CtimelineTime\x01\x01\x05self_\x0CtimelineTime\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

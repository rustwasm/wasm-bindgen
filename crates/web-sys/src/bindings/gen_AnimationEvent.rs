use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AnimationEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AnimationEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AnimationEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AnimationEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
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
    impl core::ops::Deref for AnimationEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for AnimationEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AnimationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AnimationEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AnimationEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AnimationEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AnimationEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AnimationEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AnimationEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AnimationEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AnimationEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AnimationEvent {
        #[inline]
        fn from(obj: JsValue) -> AnimationEvent {
            AnimationEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AnimationEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AnimationEvent> for AnimationEvent {
        #[inline]
        fn as_ref(&self) -> &AnimationEvent {
            self
        }
    }
    impl From<AnimationEvent> for JsValue {
        #[inline]
        fn from(obj: AnimationEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AnimationEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AnimationEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AnimationEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AnimationEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AnimationEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AnimationEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AnimationEvent> for Event {
    #[inline]
    fn from(obj: AnimationEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for AnimationEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AnimationEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: AnimationEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AnimationEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AnimationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AnimationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <AnimationEvent as WasmDescribe>::describe();
}
impl AnimationEvent {
    #[cfg(all(feature = "AnimationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    #[allow(clippy::all)]
    pub fn new(type_: &str) -> Result<AnimationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnimationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AnimationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AnimationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_new_AnimationEvent(type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AnimationEvent", feature = "AnimationEventInit",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_event_init_dict_AnimationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&AnimationEventInit as WasmDescribe>::describe();
    <AnimationEvent as WasmDescribe>::describe();
}
impl AnimationEvent {
    #[cfg(all(feature = "AnimationEvent", feature = "AnimationEventInit",))]
    #[allow(bad_style)]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)\n\n*This API requires the following crate features to be activated: `AnimationEvent`, `AnimationEventInit`*"]
    #[allow(clippy::all)]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &AnimationEventInit,
    ) -> Result<AnimationEvent, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnimationEvent", feature = "AnimationEventInit",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_event_init_dict_AnimationEvent(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                event_init_dict: <&AnimationEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_event_init_dict_AnimationEvent(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            event_init_dict: <&AnimationEventInit as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&AnimationEventInit as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        event_init_dict,
                    );
                __widl_f_new_with_event_init_dict_AnimationEvent(type_, event_init_dict)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnimationEvent as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AnimationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_animation_name_AnimationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AnimationEvent {
    #[cfg(all(feature = "AnimationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `animationName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    #[allow(clippy::all)]
    pub fn animation_name(&self) -> String {
        #[cfg(all(feature = "AnimationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_animation_name_AnimationEvent(
                self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_animation_name_AnimationEvent(
            self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_animation_name_AnimationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnimationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elapsed_time_AnimationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEvent as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AnimationEvent {
    #[cfg(all(feature = "AnimationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `elapsedTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    #[allow(clippy::all)]
    pub fn elapsed_time(&self) -> f32 {
        #[cfg(all(feature = "AnimationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elapsed_time_AnimationEvent(
                self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elapsed_time_AnimationEvent(
            self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_elapsed_time_AnimationEvent(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AnimationEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pseudo_element_AnimationEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AnimationEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl AnimationEvent {
    #[cfg(all(feature = "AnimationEvent",))]
    #[allow(bad_style)]
    #[doc = "The `pseudoElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)\n\n*This API requires the following crate features to be activated: `AnimationEvent`*"]
    #[allow(clippy::all)]
    pub fn pseudo_element(&self) -> String {
        #[cfg(all(feature = "AnimationEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pseudo_element_AnimationEvent(
                self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pseudo_element_AnimationEvent(
            self_: <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AnimationEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pseudo_element_AnimationEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_095f2b4e39ca03f6: [u8; 614usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}$\x02\0\0\0\0\x06\0\0\x02\x0EAnimationEvent __widl_instanceof_AnimationEvent\0\0\0\0\x1B__widl_f_new_AnimationEvent\x01\0\0\x01\x0EAnimationEvent\0\x01\x01\x05type_\x03new\0\0\00__widl_f_new_with_event_init_dict_AnimationEvent\x01\0\0\x01\x0EAnimationEvent\0\x01\x02\x05type_\x0Fevent_init_dict\x03new\0\0\0&__widl_f_animation_name_AnimationEvent\0\0\0\x01\x0EAnimationEvent\x01\0\x01\ranimationName\x01\x01\x05self_\ranimationName\0\0\0$__widl_f_elapsed_time_AnimationEvent\0\0\0\x01\x0EAnimationEvent\x01\0\x01\x0BelapsedTime\x01\x01\x05self_\x0BelapsedTime\0\0\0&__widl_f_pseudo_element_AnimationEvent\0\0\0\x01\x0EAnimationEvent\x01\0\x01\rpseudoElement\x01\x01\x05self_\rpseudoElement\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BeforeUnloadEvent` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BeforeUnloadEvent {
    obj: Event,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BeforeUnloadEvent: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BeforeUnloadEvent {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(66u32);
            inform(101u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(101u32);
            inform(85u32);
            inform(110u32);
            inform(108u32);
            inform(111u32);
            inform(97u32);
            inform(100u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for BeforeUnloadEvent {
        type Target = Event;
        #[inline]
        fn deref(&self) -> &Event {
            &self.obj
        }
    }
    impl IntoWasmAbi for BeforeUnloadEvent {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BeforeUnloadEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BeforeUnloadEvent {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BeforeUnloadEvent {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BeforeUnloadEvent {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BeforeUnloadEvent {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BeforeUnloadEvent {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BeforeUnloadEvent {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BeforeUnloadEvent>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BeforeUnloadEvent {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BeforeUnloadEvent {
        #[inline]
        fn from(obj: JsValue) -> BeforeUnloadEvent {
            BeforeUnloadEvent { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BeforeUnloadEvent {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BeforeUnloadEvent> for BeforeUnloadEvent {
        #[inline]
        fn as_ref(&self) -> &BeforeUnloadEvent {
            self
        }
    }
    impl From<BeforeUnloadEvent> for JsValue {
        #[inline]
        fn from(obj: BeforeUnloadEvent) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BeforeUnloadEvent {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BeforeUnloadEvent(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BeforeUnloadEvent(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BeforeUnloadEvent(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BeforeUnloadEvent { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BeforeUnloadEvent) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BeforeUnloadEvent> for Event {
    #[inline]
    fn from(obj: BeforeUnloadEvent) -> Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Event> for BeforeUnloadEvent {
    #[inline]
    fn as_ref(&self) -> &Event {
        use wasm_bindgen::JsCast;
        Event::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BeforeUnloadEvent> for ::js_sys::Object {
    #[inline]
    fn from(obj: BeforeUnloadEvent) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BeforeUnloadEvent {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BeforeUnloadEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_return_value_BeforeUnloadEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BeforeUnloadEvent as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl BeforeUnloadEvent {
    #[cfg(all(feature = "BeforeUnloadEvent",))]
    #[allow(bad_style)]
    #[doc = "The `returnValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    #[allow(clippy::all)]
    pub fn return_value(&self) -> String {
        #[cfg(all(feature = "BeforeUnloadEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_return_value_BeforeUnloadEvent(
                self_: <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_return_value_BeforeUnloadEvent(
            self_: <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_return_value_BeforeUnloadEvent(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BeforeUnloadEvent",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_return_value_BeforeUnloadEvent() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BeforeUnloadEvent as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BeforeUnloadEvent {
    #[cfg(all(feature = "BeforeUnloadEvent",))]
    #[allow(bad_style)]
    #[doc = "The `returnValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)\n\n*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    #[allow(clippy::all)]
    pub fn set_return_value(&self, return_value: &str) {
        #[cfg(all(feature = "BeforeUnloadEvent",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_return_value_BeforeUnloadEvent(
                self_: <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_return_value_BeforeUnloadEvent(
            self_: <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            return_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(return_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BeforeUnloadEvent as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let return_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(return_value);
                __widl_f_set_return_value_BeforeUnloadEvent(self_, return_value)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ec80f5d9001411e1: [u8; 380usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}:\x01\0\0\0\0\x03\0\0\x02\x11BeforeUnloadEvent#__widl_instanceof_BeforeUnloadEvent\0\0\0\0'__widl_f_return_value_BeforeUnloadEvent\0\0\0\x01\x11BeforeUnloadEvent\x01\0\x01\x0BreturnValue\x01\x01\x05self_\x0BreturnValue\0\0\0+__widl_f_set_return_value_BeforeUnloadEvent\0\0\0\x01\x11BeforeUnloadEvent\x01\0\x02\x0BreturnValue\x01\x02\x05self_\x0Creturn_value\x0BreturnValue\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

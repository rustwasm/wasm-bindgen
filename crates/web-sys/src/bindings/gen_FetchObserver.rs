use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `FetchObserver` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct FetchObserver {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_FetchObserver: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for FetchObserver {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(70u32);
            inform(101u32);
            inform(116u32);
            inform(99u32);
            inform(104u32);
            inform(79u32);
            inform(98u32);
            inform(115u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for FetchObserver {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for FetchObserver {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for FetchObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FetchObserver {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for FetchObserver {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            FetchObserver {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for FetchObserver {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a FetchObserver {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for FetchObserver {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<FetchObserver>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(FetchObserver {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for FetchObserver {
        #[inline]
        fn from(obj: JsValue) -> FetchObserver {
            FetchObserver { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for FetchObserver {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<FetchObserver> for FetchObserver {
        #[inline]
        fn as_ref(&self) -> &FetchObserver {
            self
        }
    }
    impl From<FetchObserver> for JsValue {
        #[inline]
        fn from(obj: FetchObserver) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for FetchObserver {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_FetchObserver(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_FetchObserver(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_FetchObserver(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FetchObserver { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FetchObserver) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<FetchObserver> for EventTarget {
    #[inline]
    fn from(obj: FetchObserver) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for FetchObserver {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<FetchObserver> for ::js_sys::Object {
    #[inline]
    fn from(obj: FetchObserver) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for FetchObserver {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "FetchObserver", feature = "FetchState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchObserver as WasmDescribe>::describe();
    <FetchState as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver", feature = "FetchState",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/state)\n\n*This API requires the following crate features to be activated: `FetchObserver`, `FetchState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> FetchState {
        #[cfg(all(feature = "FetchObserver", feature = "FetchState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <FetchState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <FetchState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_FetchObserver(self_)
            };
            <FetchState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_FetchObserver(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstatechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_FetchObserver(self_, onstatechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onrequestprogress_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onrequestprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn onrequestprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onrequestprogress_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onrequestprogress_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onrequestprogress_FetchObserver(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onrequestprogress_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onrequestprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn set_onrequestprogress(&self, onrequestprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onrequestprogress_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onrequestprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onrequestprogress_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onrequestprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onrequestprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onrequestprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onrequestprogress,
                    );
                __widl_f_set_onrequestprogress_FetchObserver(self_, onrequestprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresponseprogress_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onresponseprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn onresponseprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresponseprogress_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresponseprogress_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresponseprogress_FetchObserver(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FetchObserver",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresponseprogress_FetchObserver() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&FetchObserver as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl FetchObserver {
    #[cfg(all(feature = "FetchObserver",))]
    #[allow(bad_style)]
    #[doc = "The `onresponseprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)\n\n*This API requires the following crate features to be activated: `FetchObserver`*"]
    #[allow(clippy::all)]
    pub fn set_onresponseprogress(&self, onresponseprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "FetchObserver",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresponseprogress_FetchObserver(
                self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresponseprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresponseprogress_FetchObserver(
            self_: <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresponseprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onresponseprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&FetchObserver as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresponseprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresponseprogress,
                    );
                __widl_f_set_onresponseprogress_FetchObserver(self_, onresponseprogress)
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
pub static __WASM_BINDGEN_GENERATED_9ccbb370c1ad0feb: [u8; 927usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}]\x03\0\0\0\0\x08\0\0\x02\rFetchObserver\x1F__widl_instanceof_FetchObserver\0\0\0\0\x1C__widl_f_state_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0$__widl_f_onstatechange_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0(__widl_f_set_onstatechange_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0(__widl_f_onrequestprogress_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x01\x11onrequestprogress\x01\x01\x05self_\x11onrequestprogress\0\0\0,__widl_f_set_onrequestprogress_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x02\x11onrequestprogress\x01\x02\x05self_\x11onrequestprogress\x11onrequestprogress\0\0\0)__widl_f_onresponseprogress_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x01\x12onresponseprogress\x01\x01\x05self_\x12onresponseprogress\0\0\0-__widl_f_set_onresponseprogress_FetchObserver\0\0\0\x01\rFetchObserver\x01\0\x02\x12onresponseprogress\x01\x02\x05self_\x12onresponseprogress\x12onresponseprogress\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

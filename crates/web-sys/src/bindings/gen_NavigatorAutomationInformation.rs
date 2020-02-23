use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `NavigatorAutomationInformation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation)\n\n*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct NavigatorAutomationInformation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_NavigatorAutomationInformation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for NavigatorAutomationInformation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(30u32);
            inform(78u32);
            inform(97u32);
            inform(118u32);
            inform(105u32);
            inform(103u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(65u32);
            inform(117u32);
            inform(116u32);
            inform(111u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(73u32);
            inform(110u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for NavigatorAutomationInformation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for NavigatorAutomationInformation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for NavigatorAutomationInformation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a NavigatorAutomationInformation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for NavigatorAutomationInformation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            NavigatorAutomationInformation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for NavigatorAutomationInformation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a NavigatorAutomationInformation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for NavigatorAutomationInformation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<NavigatorAutomationInformation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(NavigatorAutomationInformation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for NavigatorAutomationInformation {
        #[inline]
        fn from(obj: JsValue) -> NavigatorAutomationInformation {
            NavigatorAutomationInformation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for NavigatorAutomationInformation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<NavigatorAutomationInformation> for NavigatorAutomationInformation {
        #[inline]
        fn as_ref(&self) -> &NavigatorAutomationInformation {
            self
        }
    }
    impl From<NavigatorAutomationInformation> for JsValue {
        #[inline]
        fn from(obj: NavigatorAutomationInformation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for NavigatorAutomationInformation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_NavigatorAutomationInformation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_NavigatorAutomationInformation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_NavigatorAutomationInformation(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            NavigatorAutomationInformation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const NavigatorAutomationInformation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<NavigatorAutomationInformation> for ::js_sys::Object {
    #[inline]
    fn from(obj: NavigatorAutomationInformation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for NavigatorAutomationInformation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "NavigatorAutomationInformation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_webdriver_NavigatorAutomationInformation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NavigatorAutomationInformation as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl NavigatorAutomationInformation {
    #[cfg(all(feature = "NavigatorAutomationInformation",))]
    #[allow(bad_style)]
    #[doc = "The `webdriver` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation/webdriver)\n\n*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
    #[allow(clippy::all)]
    pub fn webdriver(&self) -> bool {
        #[cfg(all(feature = "NavigatorAutomationInformation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_webdriver_NavigatorAutomationInformation(
                self_: <&NavigatorAutomationInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_webdriver_NavigatorAutomationInformation(
            self_: <&NavigatorAutomationInformation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & NavigatorAutomationInformation as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_webdriver_NavigatorAutomationInformation(self_)
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
pub static __WASM_BINDGEN_GENERATED_90040257b0a0711a: [u8; 308usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF2\0\0\0\0\0\x02\0\0\x02\x1ENavigatorAutomationInformation0__widl_instanceof_NavigatorAutomationInformation\0\0\0\01__widl_f_webdriver_NavigatorAutomationInformation\0\0\0\x01\x1ENavigatorAutomationInformation\x01\0\x01\twebdriver\x01\x01\x05self_\twebdriver\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `External` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External)\n\n*This API requires the following crate features to be activated: `External`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct External {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_External: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for External {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(69u32);
            inform(120u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(110u32);
            inform(97u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for External {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for External {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for External {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a External {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for External {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            External {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for External {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a External {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for External {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<External>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(External {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for External {
        #[inline]
        fn from(obj: JsValue) -> External {
            External { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for External {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<External> for External {
        #[inline]
        fn as_ref(&self) -> &External {
            self
        }
    }
    impl From<External> for JsValue {
        #[inline]
        fn from(obj: External) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for External {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_External(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_External(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_External(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            External { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const External) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<External> for ::js_sys::Object {
    #[inline]
    fn from(obj: External) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for External {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "External",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_search_provider_External() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&External as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl External {
    #[cfg(all(feature = "External",))]
    #[allow(bad_style)]
    #[doc = "The `AddSearchProvider()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/AddSearchProvider)\n\n*This API requires the following crate features to be activated: `External`*"]
    #[allow(clippy::all)]
    pub fn add_search_provider(&self, a_description_url: &str) {
        #[cfg(all(feature = "External",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_search_provider_External(
                self_: <&External as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_description_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_search_provider_External(
            self_: <&External as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_description_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_description_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&External as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_description_url =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_description_url);
                __widl_f_add_search_provider_External(self_, a_description_url)
            };
            ()
        }
    }
}
#[cfg(all(feature = "External",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_search_provider_installed_External() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&External as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl External {
    #[cfg(all(feature = "External",))]
    #[allow(bad_style)]
    #[doc = "The `IsSearchProviderInstalled()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/IsSearchProviderInstalled)\n\n*This API requires the following crate features to be activated: `External`*"]
    #[allow(clippy::all)]
    pub fn is_search_provider_installed(&self, a_search_url: &str) -> u32 {
        #[cfg(all(feature = "External",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_search_provider_installed_External(
                self_: <&External as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_search_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_search_provider_installed_External(
            self_: <&External as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_search_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(a_search_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&External as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_search_url =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_search_url);
                __widl_f_is_search_provider_installed_External(self_, a_search_url)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_327ab4a5e5f69bad: [u8; 359usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}%\x01\0\0\0\0\x03\0\0\x02\x08External\x1A__widl_instanceof_External\0\0\0\0%__widl_f_add_search_provider_External\0\0\0\x01\x08External\x01\0\0\x01\x02\x05self_\x11a_description_url\x11AddSearchProvider\0\0\0.__widl_f_is_search_provider_installed_External\0\0\0\x01\x08External\x01\0\0\x01\x02\x05self_\x0Ca_search_url\x19IsSearchProviderInstalled\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

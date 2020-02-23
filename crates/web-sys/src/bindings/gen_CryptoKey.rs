use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CryptoKey` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CryptoKey {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CryptoKey: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CryptoKey {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(67u32);
            inform(114u32);
            inform(121u32);
            inform(112u32);
            inform(116u32);
            inform(111u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for CryptoKey {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CryptoKey {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CryptoKey {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CryptoKey {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CryptoKey {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CryptoKey {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CryptoKey {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CryptoKey {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CryptoKey {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CryptoKey>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CryptoKey {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CryptoKey {
        #[inline]
        fn from(obj: JsValue) -> CryptoKey {
            CryptoKey { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CryptoKey {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CryptoKey> for CryptoKey {
        #[inline]
        fn as_ref(&self) -> &CryptoKey {
            self
        }
    }
    impl From<CryptoKey> for JsValue {
        #[inline]
        fn from(obj: CryptoKey) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CryptoKey {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CryptoKey(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CryptoKey(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CryptoKey(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CryptoKey { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CryptoKey) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CryptoKey> for ::js_sys::Object {
    #[inline]
    fn from(obj: CryptoKey) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CryptoKey {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CryptoKey",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_CryptoKey() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CryptoKey as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CryptoKey {
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/type)\n\n*This API requires the following crate features to be activated: `CryptoKey`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "CryptoKey",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_CryptoKey(
                self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_CryptoKey(
            self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_CryptoKey(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CryptoKey",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_extractable_CryptoKey() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CryptoKey as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl CryptoKey {
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(bad_style)]
    #[doc = "The `extractable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/extractable)\n\n*This API requires the following crate features to be activated: `CryptoKey`*"]
    #[allow(clippy::all)]
    pub fn extractable(&self) -> bool {
        #[cfg(all(feature = "CryptoKey",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_extractable_CryptoKey(
                self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_extractable_CryptoKey(
            self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_extractable_CryptoKey(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CryptoKey",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_algorithm_CryptoKey() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CryptoKey as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl CryptoKey {
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(bad_style)]
    #[doc = "The `algorithm` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/algorithm)\n\n*This API requires the following crate features to be activated: `CryptoKey`*"]
    #[allow(clippy::all)]
    pub fn algorithm(&self) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_algorithm_CryptoKey(
                self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_algorithm_CryptoKey(
            self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_algorithm_CryptoKey(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_usages_CryptoKey() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CryptoKey as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl CryptoKey {
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(bad_style)]
    #[doc = "The `usages` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/usages)\n\n*This API requires the following crate features to be activated: `CryptoKey`*"]
    #[allow(clippy::all)]
    pub fn usages(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "CryptoKey",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_usages_CryptoKey(
                self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_usages_CryptoKey(
            self_: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_usages_CryptoKey(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a3755da4752ea873: [u8; 437usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}s\x01\0\0\0\0\x05\0\0\x02\tCryptoKey\x1B__widl_instanceof_CryptoKey\0\0\0\0\x17__widl_f_type_CryptoKey\0\0\0\x01\tCryptoKey\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x1E__widl_f_extractable_CryptoKey\0\0\0\x01\tCryptoKey\x01\0\x01\x0Bextractable\x01\x01\x05self_\x0Bextractable\0\0\0\x1C__widl_f_algorithm_CryptoKey\x01\0\0\x01\tCryptoKey\x01\0\x01\talgorithm\x01\x01\x05self_\talgorithm\0\0\0\x19__widl_f_usages_CryptoKey\0\0\0\x01\tCryptoKey\x01\0\x01\x06usages\x01\x01\x05self_\x06usages\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Crypto` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Crypto {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Crypto: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Crypto {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(67u32);
            inform(114u32);
            inform(121u32);
            inform(112u32);
            inform(116u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for Crypto {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Crypto {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Crypto {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Crypto {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Crypto {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Crypto {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Crypto {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Crypto {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Crypto {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Crypto>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Crypto {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Crypto {
        #[inline]
        fn from(obj: JsValue) -> Crypto {
            Crypto { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Crypto {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Crypto> for Crypto {
        #[inline]
        fn as_ref(&self) -> &Crypto {
            self
        }
    }
    impl From<Crypto> for JsValue {
        #[inline]
        fn from(obj: Crypto) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Crypto {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Crypto(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Crypto(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Crypto(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Crypto { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Crypto) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Crypto> for ::js_sys::Object {
    #[inline]
    fn from(obj: Crypto) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Crypto {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Crypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_random_values_with_array_buffer_view_Crypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Crypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl Crypto {
    #[cfg(all(feature = "Crypto",))]
    #[allow(bad_style)]
    #[doc = "The `getRandomValues()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
    #[allow(clippy::all)]
    pub fn get_random_values_with_array_buffer_view(
        &self,
        array: &::js_sys::Object,
    ) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Crypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_random_values_with_array_buffer_view_Crypto(
                self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_random_values_with_array_buffer_view_Crypto(
            self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_random_values_with_array_buffer_view_Crypto(self_, array)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Crypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_random_values_with_u8_array_Crypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Crypto as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl Crypto {
    #[cfg(all(feature = "Crypto",))]
    #[allow(bad_style)]
    #[doc = "The `getRandomValues()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)\n\n*This API requires the following crate features to be activated: `Crypto`*"]
    #[allow(clippy::all)]
    pub fn get_random_values_with_u8_array(
        &self,
        array: &mut [u8],
    ) -> Result<::js_sys::Object, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Crypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_random_values_with_u8_array_Crypto(
                self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_random_values_with_u8_array_Crypto(
            self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(array);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(array);
                __widl_f_get_random_values_with_u8_array_Crypto(self_, array)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Crypto", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_subtle_Crypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Crypto as WasmDescribe>::describe();
    <SubtleCrypto as WasmDescribe>::describe();
}
impl Crypto {
    #[cfg(all(feature = "Crypto", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `subtle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)\n\n*This API requires the following crate features to be activated: `Crypto`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn subtle(&self) -> SubtleCrypto {
        #[cfg(all(feature = "Crypto", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_subtle_Crypto(
                self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SubtleCrypto as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_subtle_Crypto(
            self_: <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SubtleCrypto as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Crypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_subtle_Crypto(self_)
            };
            <SubtleCrypto as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_98ad0837602ce842: [u8; 402usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}P\x01\0\0\0\0\x04\0\0\x02\x06Crypto\x18__widl_instanceof_Crypto\0\0\0\08__widl_f_get_random_values_with_array_buffer_view_Crypto\x01\0\0\x01\x06Crypto\x01\0\0\x01\x02\x05self_\x05array\x0FgetRandomValues\0\0\0/__widl_f_get_random_values_with_u8_array_Crypto\x01\0\0\x01\x06Crypto\x01\0\0\x01\x02\x05self_\x05array\x0FgetRandomValues\0\0\0\x16__widl_f_subtle_Crypto\0\0\0\x01\x06Crypto\x01\0\x01\x06subtle\x01\x01\x05self_\x06subtle\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

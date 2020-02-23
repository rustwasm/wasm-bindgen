use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SubtleCrypto` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SubtleCrypto {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SubtleCrypto: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SubtleCrypto {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(117u32);
            inform(98u32);
            inform(116u32);
            inform(108u32);
            inform(101u32);
            inform(67u32);
            inform(114u32);
            inform(121u32);
            inform(112u32);
            inform(116u32);
            inform(111u32);
        }
    }
    impl core::ops::Deref for SubtleCrypto {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SubtleCrypto {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SubtleCrypto {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SubtleCrypto {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SubtleCrypto {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SubtleCrypto {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SubtleCrypto {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SubtleCrypto {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SubtleCrypto {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SubtleCrypto>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SubtleCrypto {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SubtleCrypto {
        #[inline]
        fn from(obj: JsValue) -> SubtleCrypto {
            SubtleCrypto { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SubtleCrypto {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SubtleCrypto> for SubtleCrypto {
        #[inline]
        fn as_ref(&self) -> &SubtleCrypto {
            self
        }
    }
    impl From<SubtleCrypto> for JsValue {
        #[inline]
        fn from(obj: SubtleCrypto) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SubtleCrypto {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SubtleCrypto(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SubtleCrypto(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SubtleCrypto(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SubtleCrypto { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SubtleCrypto) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SubtleCrypto> for ::js_sys::Object {
    #[inline]
    fn from(obj: SubtleCrypto) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SubtleCrypto {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decrypt_with_object_and_buffer_source_SubtleCrypto()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `decrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn decrypt_with_object_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decrypt_with_object_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decrypt_with_object_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_decrypt_with_object_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decrypt_with_str_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `decrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn decrypt_with_str_and_buffer_source(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decrypt_with_str_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decrypt_with_str_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_decrypt_with_str_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decrypt_with_object_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `decrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn decrypt_with_object_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decrypt_with_object_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decrypt_with_object_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_decrypt_with_object_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decrypt_with_str_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `decrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn decrypt_with_str_and_u8_array(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decrypt_with_str_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decrypt_with_str_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_decrypt_with_str_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_bits_with_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveBits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_bits_with_object(
        &self,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_bits_with_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_bits_with_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                __widl_f_derive_bits_with_object_SubtleCrypto(self_, algorithm, base_key, length)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_bits_with_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveBits()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_bits_with_str(
        &self,
        algorithm: &str,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_bits_with_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_bits_with_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                __widl_f_derive_bits_with_str_SubtleCrypto(self_, algorithm, base_key, length)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_key_with_object_and_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_key_with_object_and_object(
        &self,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_key_with_object_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                derived_key_type: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_key_with_object_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            derived_key_type: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(derived_key_type);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let derived_key_type =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        derived_key_type,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_derive_key_with_object_and_object_SubtleCrypto(
                    self_,
                    algorithm,
                    base_key,
                    derived_key_type,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_key_with_str_and_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_key_with_str_and_object(
        &self,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_key_with_str_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                derived_key_type: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_key_with_str_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            derived_key_type: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(derived_key_type);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let derived_key_type =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        derived_key_type,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_derive_key_with_str_and_object_SubtleCrypto(
                    self_,
                    algorithm,
                    base_key,
                    derived_key_type,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_key_with_object_and_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_key_with_object_and_str(
        &self,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_key_with_object_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                derived_key_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_key_with_object_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            derived_key_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(derived_key_type);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let derived_key_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(derived_key_type);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_derive_key_with_object_and_str_SubtleCrypto(
                    self_,
                    algorithm,
                    base_key,
                    derived_key_type,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_derive_key_with_str_and_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `deriveKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn derive_key_with_str_and_str(
        &self,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_derive_key_with_str_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                derived_key_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_derive_key_with_str_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            derived_key_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(base_key);
            drop(derived_key_type);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let base_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base_key);
                let derived_key_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(derived_key_type);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_derive_key_with_str_and_str_SubtleCrypto(
                    self_,
                    algorithm,
                    base_key,
                    derived_key_type,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_digest_with_object_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `digest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn digest_with_object_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_digest_with_object_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_digest_with_object_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_digest_with_object_and_buffer_source_SubtleCrypto(self_, algorithm, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_digest_with_str_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `digest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn digest_with_str_and_buffer_source(
        &self,
        algorithm: &str,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_digest_with_str_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_digest_with_str_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_digest_with_str_and_buffer_source_SubtleCrypto(self_, algorithm, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_digest_with_object_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `digest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn digest_with_object_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_digest_with_object_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_digest_with_object_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_digest_with_object_and_u8_array_SubtleCrypto(self_, algorithm, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_digest_with_str_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `digest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn digest_with_str_and_u8_array(
        &self,
        algorithm: &str,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_digest_with_str_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_digest_with_str_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_digest_with_str_and_u8_array_SubtleCrypto(self_, algorithm, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encrypt_with_object_and_buffer_source_SubtleCrypto()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `encrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn encrypt_with_object_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encrypt_with_object_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encrypt_with_object_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_encrypt_with_object_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encrypt_with_str_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `encrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn encrypt_with_str_and_buffer_source(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encrypt_with_str_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encrypt_with_str_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_encrypt_with_str_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encrypt_with_object_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `encrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn encrypt_with_object_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encrypt_with_object_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encrypt_with_object_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_encrypt_with_object_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encrypt_with_str_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `encrypt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn encrypt_with_str_and_u8_array(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encrypt_with_str_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encrypt_with_str_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_encrypt_with_str_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_export_key_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `exportKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/exportKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn export_key(
        &self,
        format: &str,
        key: &CryptoKey,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_export_key_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_export_key_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_export_key_SubtleCrypto(self_, format, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_key_with_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `generateKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn generate_key_with_object(
        &self,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_key_with_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_key_with_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_generate_key_with_object_SubtleCrypto(
                    self_,
                    algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_key_with_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `generateKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn generate_key_with_str(
        &self,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_key_with_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_key_with_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_generate_key_with_str_SubtleCrypto(
                    self_,
                    algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_key_with_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `importKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn import_key_with_object(
        &self,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_key_with_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_key_with_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(key_data);
            drop(algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let key_data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_data);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_import_key_with_object_SubtleCrypto(
                    self_,
                    format,
                    key_data,
                    algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_import_key_with_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `importKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)\n\n*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn import_key_with_str(
        &self,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_import_key_with_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_import_key_with_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(key_data);
            drop(algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let key_data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key_data);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_import_key_with_str_SubtleCrypto(
                    self_,
                    format,
                    key_data,
                    algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_with_object_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn sign_with_object_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_with_object_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_with_object_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_sign_with_object_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_with_str_and_buffer_source_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn sign_with_str_and_buffer_source(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_with_str_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_with_str_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_sign_with_str_and_buffer_source_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_with_object_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn sign_with_object_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_with_object_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_with_object_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_sign_with_object_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_with_str_and_u8_array_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn sign_with_str_and_u8_array(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_with_str_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_with_str_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_sign_with_str_and_u8_array_SubtleCrypto(self_, algorithm, key, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_buffer_source_and_object_and_object_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_buffer_source_and_object_and_object(
        &self,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_buffer_source_and_object_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm : < & :: js_sys :: Object as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_buffer_source_and_object_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        wrapped_key,
                    );
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrap_algorithm,
                    );
                let unwrapped_key_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrapped_key_algorithm,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_buffer_source_and_object_and_object_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_u8_array_and_object_and_object_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_u8_array_and_object_and_object(
        &self,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_u8_array_and_object_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm : < & :: js_sys :: Object as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_u8_array_and_object_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapped_key);
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrap_algorithm,
                    );
                let unwrapped_key_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrapped_key_algorithm,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_u8_array_and_object_and_object_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_buffer_source_and_str_and_object_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_buffer_source_and_str_and_object(
        &self,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_buffer_source_and_str_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm : < & :: js_sys :: Object as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_buffer_source_and_str_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        wrapped_key,
                    );
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrap_algorithm);
                let unwrapped_key_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrapped_key_algorithm,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_buffer_source_and_str_and_object_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_u8_array_and_str_and_object_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_u8_array_and_str_and_object(
        &self,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_u8_array_and_str_and_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm : < & :: js_sys :: Object as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_u8_array_and_str_and_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapped_key);
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrap_algorithm);
                let unwrapped_key_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrapped_key_algorithm,
                    );
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_u8_array_and_str_and_object_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_buffer_source_and_object_and_str_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_buffer_source_and_object_and_str(
        &self,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_buffer_source_and_object_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_buffer_source_and_object_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        wrapped_key,
                    );
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrap_algorithm,
                    );
                let unwrapped_key_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapped_key_algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_buffer_source_and_object_and_str_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_u8_array_and_object_and_str_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_u8_array_and_object_and_str(
        &self,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_u8_array_and_object_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_u8_array_and_object_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapped_key);
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        unwrap_algorithm,
                    );
                let unwrapped_key_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapped_key_algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_u8_array_and_object_and_str_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_buffer_source_and_str_and_str_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_buffer_source_and_str_and_str(
        &self,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_buffer_source_and_str_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_buffer_source_and_str_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        wrapped_key,
                    );
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrap_algorithm);
                let unwrapped_key_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapped_key_algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_buffer_source_and_str_and_str_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unwrap_key_with_u8_array_and_str_and_str_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `unwrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn unwrap_key_with_u8_array_and_str_and_str(
        &self,
        format: &str,
        wrapped_key: &mut [u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unwrap_key_with_u8_array_and_str_and_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unwrap_key_with_u8_array_and_str_and_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapped_key: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            unwrapped_key_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extractable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key_usages: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(wrapped_key);
            drop(unwrapping_key);
            drop(unwrap_algorithm);
            drop(unwrapped_key_algorithm);
            drop(extractable);
            drop(key_usages);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let wrapped_key =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapped_key);
                let unwrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapping_key);
                let unwrap_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrap_algorithm);
                let unwrapped_key_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(unwrapped_key_algorithm);
                let extractable =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extractable);
                let key_usages =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        key_usages,
                    );
                __widl_f_unwrap_key_with_u8_array_and_str_and_str_SubtleCrypto(
                    self_,
                    format,
                    wrapped_key,
                    unwrapping_key,
                    unwrap_algorithm,
                    unwrapped_key_algorithm,
                    extractable,
                    key_usages,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_object_and_buffer_source_and_buffer_source_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_object_and_buffer_source_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_object_and_buffer_source_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_object_and_buffer_source_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_object_and_buffer_source_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_str_and_buffer_source_and_buffer_source_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_str_and_buffer_source_and_buffer_source(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_str_and_buffer_source_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_str_and_buffer_source_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_str_and_buffer_source_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_object_and_u8_array_and_buffer_source_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_object_and_u8_array_and_buffer_source(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_object_and_u8_array_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_object_and_u8_array_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_object_and_u8_array_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_str_and_u8_array_and_buffer_source_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_str_and_u8_array_and_buffer_source(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_str_and_u8_array_and_buffer_source_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_str_and_u8_array_and_buffer_source_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_str_and_u8_array_and_buffer_source_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_object_and_buffer_source_and_u8_array_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_object_and_buffer_source_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_object_and_buffer_source_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_object_and_buffer_source_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_object_and_buffer_source_and_u8_array_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_str_and_buffer_source_and_u8_array_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_str_and_buffer_source_and_u8_array(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_str_and_buffer_source_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_str_and_buffer_source_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_str_and_buffer_source_and_u8_array_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_object_and_u8_array_and_u8_array_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_object_and_u8_array_and_u8_array(
        &self,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_object_and_u8_array_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_object_and_u8_array_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_object_and_u8_array_and_u8_array_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_verify_with_str_and_u8_array_and_u8_array_SubtleCrypto(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `verify()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn verify_with_str_and_u8_array_and_u8_array(
        &self,
        algorithm: &str,
        key: &CryptoKey,
        signature: &mut [u8],
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_verify_with_str_and_u8_array_and_u8_array_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_verify_with_str_and_u8_array_and_u8_array_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            signature: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(algorithm);
            drop(key);
            drop(signature);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let algorithm = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(algorithm);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let signature =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(signature);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_verify_with_str_and_u8_array_and_u8_array_SubtleCrypto(
                    self_, algorithm, key, signature, data,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_wrap_key_with_object_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `wrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn wrap_key_with_object(
        &self,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_wrap_key_with_object_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_wrap_key_with_object_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrap_algorithm: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(key);
            drop(wrapping_key);
            drop(wrap_algorithm);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let wrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapping_key);
                let wrap_algorithm =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        wrap_algorithm,
                    );
                __widl_f_wrap_key_with_object_SubtleCrypto(
                    self_,
                    format,
                    key,
                    wrapping_key,
                    wrap_algorithm,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_wrap_key_with_str_SubtleCrypto() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&SubtleCrypto as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&CryptoKey as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SubtleCrypto {
    #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
    #[allow(bad_style)]
    #[doc = "The `wrapKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)\n\n*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    #[allow(clippy::all)]
    pub fn wrap_key_with_str(
        &self,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CryptoKey", feature = "SubtleCrypto",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_wrap_key_with_str_SubtleCrypto(
                self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_wrap_key_with_str_SubtleCrypto(
            self_: <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            format: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrapping_key: <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrap_algorithm: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(format);
            drop(key);
            drop(wrapping_key);
            drop(wrap_algorithm);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SubtleCrypto as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let format = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(format);
                let key = <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let wrapping_key =
                    <&CryptoKey as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrapping_key);
                let wrap_algorithm =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrap_algorithm);
                __widl_f_wrap_key_with_str_SubtleCrypto(
                    self_,
                    format,
                    key,
                    wrapping_key,
                    wrap_algorithm,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4573ddaa55b5b52f: [u8; 6360usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x96\x18\0\0\0\0.\0\0\x02\x0CSubtleCrypto\x1E__widl_instanceof_SubtleCrypto\0\0\0\0;__widl_f_decrypt_with_object_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07decrypt\0\0\08__widl_f_decrypt_with_str_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07decrypt\0\0\06__widl_f_decrypt_with_object_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07decrypt\0\0\03__widl_f_decrypt_with_str_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07decrypt\0\0\0-__widl_f_derive_bits_with_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x08base_key\x06length\nderiveBits\0\0\0*__widl_f_derive_bits_with_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x08base_key\x06length\nderiveBits\0\0\07__widl_f_derive_key_with_object_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\talgorithm\x08base_key\x10derived_key_type\x0Bextractable\nkey_usages\tderiveKey\0\0\04__widl_f_derive_key_with_str_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\talgorithm\x08base_key\x10derived_key_type\x0Bextractable\nkey_usages\tderiveKey\0\0\04__widl_f_derive_key_with_object_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\talgorithm\x08base_key\x10derived_key_type\x0Bextractable\nkey_usages\tderiveKey\0\0\01__widl_f_derive_key_with_str_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\talgorithm\x08base_key\x10derived_key_type\x0Bextractable\nkey_usages\tderiveKey\0\0\0:__widl_f_digest_with_object_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x03\x05self_\talgorithm\x04data\x06digest\0\0\07__widl_f_digest_with_str_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x03\x05self_\talgorithm\x04data\x06digest\0\0\05__widl_f_digest_with_object_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x03\x05self_\talgorithm\x04data\x06digest\0\0\02__widl_f_digest_with_str_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x03\x05self_\talgorithm\x04data\x06digest\0\0\0;__widl_f_encrypt_with_object_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07encrypt\0\0\08__widl_f_encrypt_with_str_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07encrypt\0\0\06__widl_f_encrypt_with_object_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07encrypt\0\0\03__widl_f_encrypt_with_str_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x07encrypt\0\0\0 __widl_f_export_key_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x03\x05self_\x06format\x03key\texportKey\0\0\0.__widl_f_generate_key_with_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x0Bextractable\nkey_usages\x0BgenerateKey\0\0\0+__widl_f_generate_key_with_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x0Bextractable\nkey_usages\x0BgenerateKey\0\0\0,__widl_f_import_key_with_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\x06format\x08key_data\talgorithm\x0Bextractable\nkey_usages\timportKey\0\0\0)__widl_f_import_key_with_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x06\x05self_\x06format\x08key_data\talgorithm\x0Bextractable\nkey_usages\timportKey\0\0\08__widl_f_sign_with_object_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x04sign\0\0\05__widl_f_sign_with_str_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x04sign\0\0\03__widl_f_sign_with_object_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x04sign\0\0\00__widl_f_sign_with_str_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x04\x05self_\talgorithm\x03key\x04data\x04sign\0\0\0I__widl_f_unwrap_key_with_buffer_source_and_object_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0D__widl_f_unwrap_key_with_u8_array_and_object_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0F__widl_f_unwrap_key_with_buffer_source_and_str_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0A__widl_f_unwrap_key_with_u8_array_and_str_and_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0F__widl_f_unwrap_key_with_buffer_source_and_object_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0A__widl_f_unwrap_key_with_u8_array_and_object_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0C__widl_f_unwrap_key_with_buffer_source_and_str_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0>__widl_f_unwrap_key_with_u8_array_and_str_and_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x08\x05self_\x06format\x0Bwrapped_key\x0Eunwrapping_key\x10unwrap_algorithm\x17unwrapped_key_algorithm\x0Bextractable\nkey_usages\tunwrapKey\0\0\0L__widl_f_verify_with_object_and_buffer_source_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0I__widl_f_verify_with_str_and_buffer_source_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0G__widl_f_verify_with_object_and_u8_array_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0D__widl_f_verify_with_str_and_u8_array_and_buffer_source_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0G__widl_f_verify_with_object_and_buffer_source_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0D__widl_f_verify_with_str_and_buffer_source_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0B__widl_f_verify_with_object_and_u8_array_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0?__widl_f_verify_with_str_and_u8_array_and_u8_array_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\talgorithm\x03key\tsignature\x04data\x06verify\0\0\0*__widl_f_wrap_key_with_object_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\x06format\x03key\x0Cwrapping_key\x0Ewrap_algorithm\x07wrapKey\0\0\0'__widl_f_wrap_key_with_str_SubtleCrypto\x01\0\0\x01\x0CSubtleCrypto\x01\0\0\x01\x05\x05self_\x06format\x03key\x0Cwrapping_key\x0Ewrap_algorithm\x07wrapKey\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

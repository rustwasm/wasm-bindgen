use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CredentialsContainer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CredentialsContainer {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CredentialsContainer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CredentialsContainer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(67u32);
            inform(114u32);
            inform(101u32);
            inform(100u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(105u32);
            inform(97u32);
            inform(108u32);
            inform(115u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for CredentialsContainer {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CredentialsContainer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CredentialsContainer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CredentialsContainer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CredentialsContainer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CredentialsContainer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CredentialsContainer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CredentialsContainer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CredentialsContainer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CredentialsContainer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CredentialsContainer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CredentialsContainer {
        #[inline]
        fn from(obj: JsValue) -> CredentialsContainer {
            CredentialsContainer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CredentialsContainer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CredentialsContainer> for CredentialsContainer {
        #[inline]
        fn as_ref(&self) -> &CredentialsContainer {
            self
        }
    }
    impl From<CredentialsContainer> for JsValue {
        #[inline]
        fn from(obj: CredentialsContainer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CredentialsContainer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CredentialsContainer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CredentialsContainer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CredentialsContainer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CredentialsContainer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CredentialsContainer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CredentialsContainer> for ::js_sys::Object {
    #[inline]
    fn from(obj: CredentialsContainer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CredentialsContainer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CredentialsContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(feature = "CredentialsContainer",))]
    #[allow(bad_style)]
    #[doc = "The `create()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn create(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CredentialsContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_CredentialsContainer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CredentialCreationOptions",
    feature = "CredentialsContainer",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_with_options_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <&CredentialCreationOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(
        feature = "CredentialCreationOptions",
        feature = "CredentialsContainer",
    ))]
    #[allow(bad_style)]
    #[doc = "The `create()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)\n\n*This API requires the following crate features to be activated: `CredentialCreationOptions`, `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn create_with_options(
        &self,
        options: &CredentialCreationOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CredentialCreationOptions",
            feature = "CredentialsContainer",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_with_options_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CredentialCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_with_options_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&CredentialCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&CredentialCreationOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_create_with_options_CredentialsContainer(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CredentialsContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(feature = "CredentialsContainer",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn get(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CredentialsContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_CredentialsContainer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CredentialRequestOptions", feature = "CredentialsContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_options_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <&CredentialRequestOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(feature = "CredentialRequestOptions", feature = "CredentialsContainer",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)\n\n*This API requires the following crate features to be activated: `CredentialRequestOptions`, `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn get_with_options(
        &self,
        options: &CredentialRequestOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CredentialRequestOptions", feature = "CredentialsContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_options_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&CredentialRequestOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_options_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&CredentialRequestOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&CredentialRequestOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_get_with_options_CredentialsContainer(self_, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CredentialsContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prevent_silent_access_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(feature = "CredentialsContainer",))]
    #[allow(bad_style)]
    #[doc = "The `preventSilentAccess()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)\n\n*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn prevent_silent_access(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CredentialsContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prevent_silent_access_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prevent_silent_access_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prevent_silent_access_CredentialsContainer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Credential", feature = "CredentialsContainer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_store_CredentialsContainer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CredentialsContainer as WasmDescribe>::describe();
    <&Credential as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl CredentialsContainer {
    #[cfg(all(feature = "Credential", feature = "CredentialsContainer",))]
    #[allow(bad_style)]
    #[doc = "The `store()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)\n\n*This API requires the following crate features to be activated: `Credential`, `CredentialsContainer`*"]
    #[allow(clippy::all)]
    pub fn store(
        &self,
        credential: &Credential,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Credential", feature = "CredentialsContainer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_store_CredentialsContainer(
                self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                credential: <&Credential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_store_CredentialsContainer(
            self_: <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            credential: <&Credential as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(credential);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CredentialsContainer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let credential =
                    <&Credential as wasm_bindgen::convert::IntoWasmAbi>::into_abi(credential);
                __widl_f_store_CredentialsContainer(self_, credential)
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
pub static __WASM_BINDGEN_GENERATED_b975f95bcc4d7677: [u8; 734usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9C\x02\0\0\0\0\x07\0\0\x02\x14CredentialsContainer&__widl_instanceof_CredentialsContainer\0\0\0\0$__widl_f_create_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x01\x05self_\x06create\0\0\01__widl_f_create_with_options_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x02\x05self_\x07options\x06create\0\0\0!__widl_f_get_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x01\x05self_\x03get\0\0\0.__widl_f_get_with_options_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x02\x05self_\x07options\x03get\0\0\03__widl_f_prevent_silent_access_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x01\x05self_\x13preventSilentAccess\0\0\0#__widl_f_store_CredentialsContainer\x01\0\0\x01\x14CredentialsContainer\x01\0\0\x01\x02\x05self_\ncredential\x05store\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

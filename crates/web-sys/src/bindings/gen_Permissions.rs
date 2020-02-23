use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Permissions` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Permissions {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Permissions: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Permissions {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(80u32);
            inform(101u32);
            inform(114u32);
            inform(109u32);
            inform(105u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for Permissions {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Permissions {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Permissions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Permissions {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Permissions {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Permissions {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Permissions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Permissions {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Permissions {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Permissions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Permissions {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Permissions {
        #[inline]
        fn from(obj: JsValue) -> Permissions {
            Permissions { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Permissions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Permissions> for Permissions {
        #[inline]
        fn as_ref(&self) -> &Permissions {
            self
        }
    }
    impl From<Permissions> for JsValue {
        #[inline]
        fn from(obj: Permissions) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Permissions {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Permissions(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Permissions(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Permissions(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Permissions { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Permissions) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Permissions> for ::js_sys::Object {
    #[inline]
    fn from(obj: Permissions) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Permissions {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Permissions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_Permissions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Permissions as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Permissions {
    #[cfg(all(feature = "Permissions",))]
    #[allow(bad_style)]
    #[doc = "The `query()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
    #[allow(clippy::all)]
    pub fn query(
        &self,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Permissions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_Permissions(
                self_: <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                permission: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_Permissions(
            self_: <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            permission: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(permission);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let permission =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(permission);
                __widl_f_query_Permissions(self_, permission)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Permissions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_revoke_Permissions() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Permissions as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Permissions {
    #[cfg(all(feature = "Permissions",))]
    #[allow(bad_style)]
    #[doc = "The `revoke()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)\n\n*This API requires the following crate features to be activated: `Permissions`*"]
    #[allow(clippy::all)]
    pub fn revoke(
        &self,
        permission: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Permissions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_revoke_Permissions(
                self_: <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                permission: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_revoke_Permissions(
            self_: <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            permission: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(permission);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Permissions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let permission =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(permission);
                __widl_f_revoke_Permissions(self_, permission)
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
pub static __WASM_BINDGEN_GENERATED_03fb0541c967e065: [u8; 301usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xEB\0\0\0\0\0\x03\0\0\x02\x0BPermissions\x1D__widl_instanceof_Permissions\0\0\0\0\x1A__widl_f_query_Permissions\x01\0\0\x01\x0BPermissions\x01\0\0\x01\x02\x05self_\npermission\x05query\0\0\0\x1B__widl_f_revoke_Permissions\x01\0\0\x01\x0BPermissions\x01\0\0\x01\x02\x05self_\npermission\x06revoke\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

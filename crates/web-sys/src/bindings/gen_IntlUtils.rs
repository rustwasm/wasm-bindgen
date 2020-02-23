use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IntlUtils` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils)\n\n*This API requires the following crate features to be activated: `IntlUtils`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IntlUtils {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IntlUtils: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IntlUtils {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(73u32);
            inform(110u32);
            inform(116u32);
            inform(108u32);
            inform(85u32);
            inform(116u32);
            inform(105u32);
            inform(108u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for IntlUtils {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IntlUtils {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IntlUtils {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IntlUtils {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IntlUtils {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IntlUtils {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IntlUtils {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IntlUtils {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IntlUtils {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IntlUtils>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IntlUtils {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IntlUtils {
        #[inline]
        fn from(obj: JsValue) -> IntlUtils {
            IntlUtils { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IntlUtils {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IntlUtils> for IntlUtils {
        #[inline]
        fn as_ref(&self) -> &IntlUtils {
            self
        }
    }
    impl From<IntlUtils> for JsValue {
        #[inline]
        fn from(obj: IntlUtils) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IntlUtils {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IntlUtils(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IntlUtils(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IntlUtils(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IntlUtils { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IntlUtils) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IntlUtils> for ::js_sys::Object {
    #[inline]
    fn from(obj: IntlUtils) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IntlUtils {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DisplayNameResult", feature = "IntlUtils",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_display_names_IntlUtils() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IntlUtils as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <DisplayNameResult as WasmDescribe>::describe();
}
impl IntlUtils {
    #[cfg(all(feature = "DisplayNameResult", feature = "IntlUtils",))]
    #[allow(bad_style)]
    #[doc = "The `getDisplayNames()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)\n\n*This API requires the following crate features to be activated: `DisplayNameResult`, `IntlUtils`*"]
    #[allow(clippy::all)]
    pub fn get_display_names(
        &self,
        locales: &::wasm_bindgen::JsValue,
    ) -> Result<DisplayNameResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DisplayNameResult", feature = "IntlUtils",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_display_names_IntlUtils(
                self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_display_names_IntlUtils(
            self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(locales);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let locales =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        locales,
                    );
                __widl_f_get_display_names_IntlUtils(self_, locales)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "DisplayNameOptions",
    feature = "DisplayNameResult",
    feature = "IntlUtils",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_display_names_with_options_IntlUtils() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IntlUtils as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&DisplayNameOptions as WasmDescribe>::describe();
    <DisplayNameResult as WasmDescribe>::describe();
}
impl IntlUtils {
    #[cfg(all(
        feature = "DisplayNameOptions",
        feature = "DisplayNameResult",
        feature = "IntlUtils",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getDisplayNames()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)\n\n*This API requires the following crate features to be activated: `DisplayNameOptions`, `DisplayNameResult`, `IntlUtils`*"]
    #[allow(clippy::all)]
    pub fn get_display_names_with_options(
        &self,
        locales: &::wasm_bindgen::JsValue,
        options: &DisplayNameOptions,
    ) -> Result<DisplayNameResult, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "DisplayNameOptions",
            feature = "DisplayNameResult",
            feature = "IntlUtils",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_display_names_with_options_IntlUtils(
                self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&DisplayNameOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_display_names_with_options_IntlUtils(
            self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&DisplayNameOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(locales);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let locales =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        locales,
                    );
                let options =
                    <&DisplayNameOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_get_display_names_with_options_IntlUtils(self_, locales, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DisplayNameResult as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IntlUtils", feature = "LocaleInfo",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_locale_info_IntlUtils() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IntlUtils as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <LocaleInfo as WasmDescribe>::describe();
}
impl IntlUtils {
    #[cfg(all(feature = "IntlUtils", feature = "LocaleInfo",))]
    #[allow(bad_style)]
    #[doc = "The `getLocaleInfo()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getLocaleInfo)\n\n*This API requires the following crate features to be activated: `IntlUtils`, `LocaleInfo`*"]
    #[allow(clippy::all)]
    pub fn get_locale_info(
        &self,
        locales: &::wasm_bindgen::JsValue,
    ) -> Result<LocaleInfo, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IntlUtils", feature = "LocaleInfo",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_locale_info_IntlUtils(
                self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <LocaleInfo as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_locale_info_IntlUtils(
            self_: <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            locales: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <LocaleInfo as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(locales);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IntlUtils as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let locales =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        locales,
                    );
                __widl_f_get_locale_info_IntlUtils(self_, locales)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<LocaleInfo as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_27ae8fa92b2aa167: [u8; 431usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}m\x01\0\0\0\0\x04\0\0\x02\tIntlUtils\x1B__widl_instanceof_IntlUtils\0\0\0\0$__widl_f_get_display_names_IntlUtils\x01\0\0\x01\tIntlUtils\x01\0\0\x01\x02\x05self_\x07locales\x0FgetDisplayNames\0\0\01__widl_f_get_display_names_with_options_IntlUtils\x01\0\0\x01\tIntlUtils\x01\0\0\x01\x03\x05self_\x07locales\x07options\x0FgetDisplayNames\0\0\0\"__widl_f_get_locale_info_IntlUtils\x01\0\0\x01\tIntlUtils\x01\0\0\x01\x02\x05self_\x07locales\rgetLocaleInfo\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

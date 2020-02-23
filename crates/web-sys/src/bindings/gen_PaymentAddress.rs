use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaymentAddress` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaymentAddress {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaymentAddress: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaymentAddress {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(80u32);
            inform(97u32);
            inform(121u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(65u32);
            inform(100u32);
            inform(100u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for PaymentAddress {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaymentAddress {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaymentAddress {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaymentAddress {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaymentAddress {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaymentAddress {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaymentAddress {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaymentAddress {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaymentAddress {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaymentAddress>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaymentAddress {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaymentAddress {
        #[inline]
        fn from(obj: JsValue) -> PaymentAddress {
            PaymentAddress { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaymentAddress {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaymentAddress> for PaymentAddress {
        #[inline]
        fn as_ref(&self) -> &PaymentAddress {
            self
        }
    }
    impl From<PaymentAddress> for JsValue {
        #[inline]
        fn from(obj: PaymentAddress) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaymentAddress {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaymentAddress(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaymentAddress(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaymentAddress(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaymentAddress { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaymentAddress) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaymentAddress> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaymentAddress) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaymentAddress {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/toJSON)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_PaymentAddress(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_country_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `country` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/country)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn country(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_country_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_country_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_country_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_address_line_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `addressLine` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/addressLine)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn address_line(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_address_line_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_address_line_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_address_line_PaymentAddress(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_region_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `region` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/region)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn region(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_region_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_region_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_region_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_city_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `city` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/city)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn city(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_city_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_city_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_city_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dependent_locality_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `dependentLocality` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/dependentLocality)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn dependent_locality(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dependent_locality_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dependent_locality_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dependent_locality_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_postal_code_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `postalCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/postalCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn postal_code(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_postal_code_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_postal_code_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_postal_code_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sorting_code_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `sortingCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/sortingCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn sorting_code(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sorting_code_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sorting_code_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sorting_code_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_code_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `languageCode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/languageCode)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn language_code(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_code_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_code_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_code_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_organization_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `organization` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/organization)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn organization(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_organization_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_organization_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_organization_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_recipient_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `recipient` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/recipient)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn recipient(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_recipient_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_recipient_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_recipient_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_phone_PaymentAddress() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentAddress as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentAddress {
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(bad_style)]
    #[doc = "The `phone` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/phone)\n\n*This API requires the following crate features to be activated: `PaymentAddress`*"]
    #[allow(clippy::all)]
    pub fn phone(&self) -> String {
        #[cfg(all(feature = "PaymentAddress",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_phone_PaymentAddress(
                self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_phone_PaymentAddress(
            self_: <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PaymentAddress as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_phone_PaymentAddress(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_d9395aeba99e2556: [u8; 1206usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}t\x04\0\0\0\0\r\0\0\x02\x0EPaymentAddress __widl_instanceof_PaymentAddress\0\0\0\0\x1F__widl_f_to_json_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\x1F__widl_f_country_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x07country\x01\x01\x05self_\x07country\0\0\0$__widl_f_address_line_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x0BaddressLine\x01\x01\x05self_\x0BaddressLine\0\0\0\x1E__widl_f_region_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x06region\x01\x01\x05self_\x06region\0\0\0\x1C__widl_f_city_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x04city\x01\x01\x05self_\x04city\0\0\0*__widl_f_dependent_locality_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x11dependentLocality\x01\x01\x05self_\x11dependentLocality\0\0\0#__widl_f_postal_code_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\npostalCode\x01\x01\x05self_\npostalCode\0\0\0$__widl_f_sorting_code_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x0BsortingCode\x01\x01\x05self_\x0BsortingCode\0\0\0%__widl_f_language_code_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x0ClanguageCode\x01\x01\x05self_\x0ClanguageCode\0\0\0$__widl_f_organization_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x0Corganization\x01\x01\x05self_\x0Corganization\0\0\0!__widl_f_recipient_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\trecipient\x01\x01\x05self_\trecipient\0\0\0\x1D__widl_f_phone_PaymentAddress\0\0\0\x01\x0EPaymentAddress\x01\0\x01\x05phone\x01\x01\x05self_\x05phone\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

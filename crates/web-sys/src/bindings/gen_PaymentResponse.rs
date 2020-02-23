use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PaymentResponse` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PaymentResponse {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PaymentResponse: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PaymentResponse {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(80u32);
            inform(97u32);
            inform(121u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(112u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for PaymentResponse {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PaymentResponse {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PaymentResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaymentResponse {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PaymentResponse {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PaymentResponse {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PaymentResponse {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PaymentResponse {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PaymentResponse {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PaymentResponse>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PaymentResponse {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PaymentResponse {
        #[inline]
        fn from(obj: JsValue) -> PaymentResponse {
            PaymentResponse { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PaymentResponse {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PaymentResponse> for PaymentResponse {
        #[inline]
        fn as_ref(&self) -> &PaymentResponse {
            self
        }
    }
    impl From<PaymentResponse> for JsValue {
        #[inline]
        fn from(obj: PaymentResponse) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PaymentResponse {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PaymentResponse(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PaymentResponse(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PaymentResponse(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaymentResponse { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaymentResponse) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PaymentResponse> for ::js_sys::Object {
    #[inline]
    fn from(obj: PaymentResponse) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PaymentResponse {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_complete_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `complete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn complete(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_complete_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_complete_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_complete_PaymentResponse(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentComplete", feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_complete_with_result_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <PaymentComplete as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentComplete", feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `complete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)\n\n*This API requires the following crate features to be activated: `PaymentComplete`, `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn complete_with_result(&self, result: PaymentComplete) -> ::js_sys::Promise {
        #[cfg(all(feature = "PaymentComplete", feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_complete_with_result_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                result: <PaymentComplete as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_complete_with_result_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            result: <PaymentComplete as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(result);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let result =
                    <PaymentComplete as wasm_bindgen::convert::IntoWasmAbi>::into_abi(result);
                __widl_f_complete_with_result_PaymentResponse(self_, result)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_PaymentResponse(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_id_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `requestId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn request_id(&self) -> String {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_id_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_id_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_id_PaymentResponse(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_name_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `methodName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn method_name(&self) -> String {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_name_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_name_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_method_name_PaymentResponse(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_details_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `details` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn details(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_details_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_details_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_details_PaymentResponse(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentAddress", feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shipping_address_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <Option<PaymentAddress> as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentAddress", feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `shippingAddress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)\n\n*This API requires the following crate features to be activated: `PaymentAddress`, `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn shipping_address(&self) -> Option<PaymentAddress> {
        #[cfg(all(feature = "PaymentAddress", feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shipping_address_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<PaymentAddress> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shipping_address_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<PaymentAddress> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shipping_address_PaymentResponse(self_)
            };
            <Option<PaymentAddress> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shipping_option_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `shippingOption` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn shipping_option(&self) -> Option<String> {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shipping_option_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shipping_option_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shipping_option_PaymentResponse(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_payer_name_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `payerName` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn payer_name(&self) -> Option<String> {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_payer_name_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_payer_name_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_payer_name_PaymentResponse(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_payer_email_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `payerEmail` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn payer_email(&self) -> Option<String> {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_payer_email_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_payer_email_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_payer_email_PaymentResponse(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PaymentResponse",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_payer_phone_PaymentResponse() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PaymentResponse as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl PaymentResponse {
    #[cfg(all(feature = "PaymentResponse",))]
    #[allow(bad_style)]
    #[doc = "The `payerPhone` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)\n\n*This API requires the following crate features to be activated: `PaymentResponse`*"]
    #[allow(clippy::all)]
    pub fn payer_phone(&self) -> Option<String> {
        #[cfg(all(feature = "PaymentResponse",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_payer_phone_PaymentResponse(
                self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_payer_phone_PaymentResponse(
            self_: <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PaymentResponse as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_payer_phone_PaymentResponse(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_92e7178bfbdd8935: [u8; 1161usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}G\x04\0\0\0\0\x0C\0\0\x02\x0FPaymentResponse!__widl_instanceof_PaymentResponse\0\0\0\0!__widl_f_complete_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\0\x01\x01\x05self_\x08complete\0\0\0-__widl_f_complete_with_result_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\0\x01\x02\x05self_\x06result\x08complete\0\0\0 __widl_f_to_json_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0#__widl_f_request_id_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\trequestId\x01\x01\x05self_\trequestId\0\0\0$__widl_f_method_name_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\nmethodName\x01\x01\x05self_\nmethodName\0\0\0 __widl_f_details_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\x07details\x01\x01\x05self_\x07details\0\0\0)__widl_f_shipping_address_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\x0FshippingAddress\x01\x01\x05self_\x0FshippingAddress\0\0\0(__widl_f_shipping_option_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\x0EshippingOption\x01\x01\x05self_\x0EshippingOption\0\0\0#__widl_f_payer_name_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\tpayerName\x01\x01\x05self_\tpayerName\0\0\0$__widl_f_payer_email_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\npayerEmail\x01\x01\x05self_\npayerEmail\0\0\0$__widl_f_payer_phone_PaymentResponse\0\0\0\x01\x0FPaymentResponse\x01\0\x01\npayerPhone\x01\x01\x05self_\npayerPhone\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

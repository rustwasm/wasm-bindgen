use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `SignResponse` dictionary\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
pub struct SignResponse {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl SignResponse {
    #[doc = "Construct a new `SignResponse`\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn new() -> SignResponse {
        let mut _ret = SignResponse {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `clientData` field of this object\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn client_data(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("clientData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `errorCode` field of this object\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("errorCode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `errorMessage` field of this object\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("errorMessage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `keyHandle` field of this object\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn key_handle(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("keyHandle"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `signatureData` field of this object\n\n\n*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn signature_data(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("signatureData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_SignResponse: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<SignResponse> for JsValue {
        #[inline]
        fn from(val: SignResponse) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for SignResponse {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for SignResponse {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for SignResponse {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a SignResponse {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for SignResponse {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            SignResponse {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for SignResponse {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SignResponse {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for SignResponse {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for SignResponse {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<SignResponse>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(SignResponse {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for SignResponse {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SignResponse {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SignResponse) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c4969c1f2bca3f0f: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

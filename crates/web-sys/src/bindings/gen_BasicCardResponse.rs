use super::*;
use js_sys::Object;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `BasicCardResponse` dictionary\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
pub struct BasicCardResponse {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl BasicCardResponse {
    #[doc = "Construct a new `BasicCardResponse`\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    pub fn new(card_number: &str) -> BasicCardResponse {
        let mut _ret = BasicCardResponse {
            obj: ::js_sys::Object::new(),
        };
        _ret.card_number(card_number);
        return _ret;
    }
    #[cfg(all(feature = "PaymentAddress",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `billingAddress` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    pub fn billing_address(&mut self, val: Option<&PaymentAddress>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("billingAddress"),
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
    #[doc = "Configure the `cardNumber` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_number(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("cardNumber"),
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
    #[doc = "Configure the `cardSecurityCode` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn card_security_code(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("cardSecurityCode"),
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
    #[doc = "Configure the `cardholderName` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn cardholder_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("cardholderName"),
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
    #[doc = "Configure the `expiryMonth` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_month(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("expiryMonth"),
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
    #[doc = "Configure the `expiryYear` field of this object\n\n\n*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn expiry_year(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("expiryYear"),
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
const _CONST_BasicCardResponse: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<BasicCardResponse> for JsValue {
        #[inline]
        fn from(val: BasicCardResponse) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for BasicCardResponse {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for BasicCardResponse {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for BasicCardResponse {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a BasicCardResponse {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for BasicCardResponse {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            BasicCardResponse {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for BasicCardResponse {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BasicCardResponse {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for BasicCardResponse {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for BasicCardResponse {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<BasicCardResponse>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(BasicCardResponse {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for BasicCardResponse {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BasicCardResponse {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BasicCardResponse) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8b35b01b39de8cee: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `PaymentMethodChangeEventInit` dictionary\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
pub struct PaymentMethodChangeEventInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl PaymentMethodChangeEventInit {
    #[doc = "Construct a new `PaymentMethodChangeEventInit`\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn new(method_name: &str) -> PaymentMethodChangeEventInit {
        let mut _ret = PaymentMethodChangeEventInit {
            obj: ::js_sys::Object::new(),
        };
        _ret.method_name(method_name);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `bubbles` field of this object\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("bubbles"),
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
    #[doc = "Configure the `cancelable` field of this object\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("cancelable"),
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
    #[doc = "Configure the `composed` field of this object\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("composed"),
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
    #[doc = "Configure the `methodDetails` field of this object\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn method_details(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("methodDetails"),
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
    #[doc = "Configure the `methodName` field of this object\n\n\n*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn method_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("methodName"),
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
const _CONST_PaymentMethodChangeEventInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<PaymentMethodChangeEventInit> for JsValue {
        #[inline]
        fn from(val: PaymentMethodChangeEventInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for PaymentMethodChangeEventInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for PaymentMethodChangeEventInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for PaymentMethodChangeEventInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a PaymentMethodChangeEventInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for PaymentMethodChangeEventInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            PaymentMethodChangeEventInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for PaymentMethodChangeEventInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PaymentMethodChangeEventInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for PaymentMethodChangeEventInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for PaymentMethodChangeEventInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<PaymentMethodChangeEventInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(PaymentMethodChangeEventInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for PaymentMethodChangeEventInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PaymentMethodChangeEventInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PaymentMethodChangeEventInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5be3ed2558bbbe9e: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

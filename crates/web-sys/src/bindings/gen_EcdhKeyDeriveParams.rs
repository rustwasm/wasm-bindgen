use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `EcdhKeyDeriveParams` dictionary\n\n\n*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
pub struct EcdhKeyDeriveParams {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl EcdhKeyDeriveParams {
    #[doc = "Construct a new `EcdhKeyDeriveParams`\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> EcdhKeyDeriveParams {
        let mut _ret = EcdhKeyDeriveParams {
            obj: ::js_sys::Object::new(),
        };
        _ret.name(name);
        _ret.public(public);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("name"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `public` field of this object\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("public"),
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
const _CONST_EcdhKeyDeriveParams: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<EcdhKeyDeriveParams> for JsValue {
        #[inline]
        fn from(val: EcdhKeyDeriveParams) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for EcdhKeyDeriveParams {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for EcdhKeyDeriveParams {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for EcdhKeyDeriveParams {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a EcdhKeyDeriveParams {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for EcdhKeyDeriveParams {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            EcdhKeyDeriveParams {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for EcdhKeyDeriveParams {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a EcdhKeyDeriveParams {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for EcdhKeyDeriveParams {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for EcdhKeyDeriveParams {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<EcdhKeyDeriveParams>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(EcdhKeyDeriveParams {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for EcdhKeyDeriveParams {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            EcdhKeyDeriveParams {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const EcdhKeyDeriveParams) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_712e0ab2e58b9b4b: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `CryptoKeyPair` dictionary\n\n\n*This API requires the following crate features to be activated: `CryptoKeyPair`*"]
pub struct CryptoKeyPair {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl CryptoKeyPair {
    #[doc = "Construct a new `CryptoKeyPair`\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn new(private_key: &CryptoKey, public_key: &CryptoKey) -> CryptoKeyPair {
        let mut _ret = CryptoKeyPair {
            obj: ::js_sys::Object::new(),
        };
        _ret.private_key(private_key);
        _ret.public_key(public_key);
        return _ret;
    }
    #[cfg(all(feature = "CryptoKey",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `privateKey` field of this object\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn private_key(&mut self, val: &CryptoKey) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("privateKey"),
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
    #[doc = "Configure the `publicKey` field of this object\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn public_key(&mut self, val: &CryptoKey) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("publicKey"),
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
const _CONST_CryptoKeyPair: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<CryptoKeyPair> for JsValue {
        #[inline]
        fn from(val: CryptoKeyPair) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for CryptoKeyPair {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for CryptoKeyPair {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for CryptoKeyPair {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a CryptoKeyPair {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for CryptoKeyPair {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            CryptoKeyPair {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for CryptoKeyPair {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CryptoKeyPair {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for CryptoKeyPair {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for CryptoKeyPair {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<CryptoKeyPair>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(CryptoKeyPair {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for CryptoKeyPair {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CryptoKeyPair {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CryptoKeyPair) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_70f02a046a1d1c0e: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

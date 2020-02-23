use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `DhKeyDeriveParams` dictionary\n\n\n*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
pub struct DhKeyDeriveParams {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl DhKeyDeriveParams {
    #[doc = "Construct a new `DhKeyDeriveParams`\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> DhKeyDeriveParams {
        let mut _ret = DhKeyDeriveParams {
            obj: ::js_sys::Object::new(),
        };
        _ret.name(name);
        _ret.public(public);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
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
    #[doc = "Configure the `public` field of this object\n\n\n*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
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
const _CONST_DhKeyDeriveParams: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<DhKeyDeriveParams> for JsValue {
        #[inline]
        fn from(val: DhKeyDeriveParams) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for DhKeyDeriveParams {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for DhKeyDeriveParams {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for DhKeyDeriveParams {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a DhKeyDeriveParams {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for DhKeyDeriveParams {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            DhKeyDeriveParams {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for DhKeyDeriveParams {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DhKeyDeriveParams {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for DhKeyDeriveParams {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for DhKeyDeriveParams {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<DhKeyDeriveParams>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(DhKeyDeriveParams {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for DhKeyDeriveParams {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DhKeyDeriveParams {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DhKeyDeriveParams) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_61708e4bb984b0f3: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

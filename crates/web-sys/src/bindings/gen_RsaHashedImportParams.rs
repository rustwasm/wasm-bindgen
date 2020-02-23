use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `RsaHashedImportParams` dictionary\n\n\n*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
pub struct RsaHashedImportParams {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl RsaHashedImportParams {
    #[doc = "Construct a new `RsaHashedImportParams`\n\n\n*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new(hash: &::wasm_bindgen::JsValue) -> RsaHashedImportParams {
        let mut _ret = RsaHashedImportParams {
            obj: ::js_sys::Object::new(),
        };
        _ret.hash(hash);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `hash` field of this object\n\n\n*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("hash"),
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
const _CONST_RsaHashedImportParams: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<RsaHashedImportParams> for JsValue {
        #[inline]
        fn from(val: RsaHashedImportParams) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for RsaHashedImportParams {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for RsaHashedImportParams {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for RsaHashedImportParams {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a RsaHashedImportParams {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for RsaHashedImportParams {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            RsaHashedImportParams {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for RsaHashedImportParams {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RsaHashedImportParams {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for RsaHashedImportParams {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for RsaHashedImportParams {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<RsaHashedImportParams>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(RsaHashedImportParams {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for RsaHashedImportParams {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RsaHashedImportParams {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RsaHashedImportParams) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c256b6c20aa32203: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

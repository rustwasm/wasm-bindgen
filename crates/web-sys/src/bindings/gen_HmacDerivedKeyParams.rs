use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `HmacDerivedKeyParams` dictionary\n\n\n*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
pub struct HmacDerivedKeyParams {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl HmacDerivedKeyParams {
    #[doc = "Construct a new `HmacDerivedKeyParams`\n\n\n*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> HmacDerivedKeyParams {
        let mut _ret = HmacDerivedKeyParams {
            obj: ::js_sys::Object::new(),
        };
        _ret.name(name);
        _ret.hash(hash);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
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
    #[allow(clippy::all)]
    #[doc = "Configure the `hash` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
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
    #[allow(clippy::all)]
    #[doc = "Configure the `length` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("length"),
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
const _CONST_HmacDerivedKeyParams: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<HmacDerivedKeyParams> for JsValue {
        #[inline]
        fn from(val: HmacDerivedKeyParams) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for HmacDerivedKeyParams {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for HmacDerivedKeyParams {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for HmacDerivedKeyParams {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a HmacDerivedKeyParams {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for HmacDerivedKeyParams {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            HmacDerivedKeyParams {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for HmacDerivedKeyParams {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HmacDerivedKeyParams {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for HmacDerivedKeyParams {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for HmacDerivedKeyParams {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<HmacDerivedKeyParams>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(HmacDerivedKeyParams {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for HmacDerivedKeyParams {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HmacDerivedKeyParams {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HmacDerivedKeyParams) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7e71a56d3fb3b8d8: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

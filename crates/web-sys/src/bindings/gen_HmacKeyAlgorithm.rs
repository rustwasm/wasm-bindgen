use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `HmacKeyAlgorithm` dictionary\n\n\n*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
pub struct HmacKeyAlgorithm {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl HmacKeyAlgorithm {
    #[doc = "Construct a new `HmacKeyAlgorithm`\n\n\n*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn new(name: &str, hash: &KeyAlgorithm, length: u32) -> HmacKeyAlgorithm {
        let mut _ret = HmacKeyAlgorithm {
            obj: ::js_sys::Object::new(),
        };
        _ret.name(name);
        _ret.hash(hash);
        _ret.length(length);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
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
    #[cfg(all(feature = "KeyAlgorithm",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `hash` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self {
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
    #[doc = "Configure the `length` field of this object\n\n\n*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
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
const _CONST_HmacKeyAlgorithm: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<HmacKeyAlgorithm> for JsValue {
        #[inline]
        fn from(val: HmacKeyAlgorithm) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for HmacKeyAlgorithm {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for HmacKeyAlgorithm {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for HmacKeyAlgorithm {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a HmacKeyAlgorithm {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for HmacKeyAlgorithm {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            HmacKeyAlgorithm {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for HmacKeyAlgorithm {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HmacKeyAlgorithm {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for HmacKeyAlgorithm {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for HmacKeyAlgorithm {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<HmacKeyAlgorithm>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(HmacKeyAlgorithm {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for HmacKeyAlgorithm {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HmacKeyAlgorithm {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HmacKeyAlgorithm) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_252b3a5f47703b87: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

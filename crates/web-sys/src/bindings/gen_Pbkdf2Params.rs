use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `Pbkdf2Params` dictionary\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
pub struct Pbkdf2Params {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl Pbkdf2Params {
    #[doc = "Construct a new `Pbkdf2Params`\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        iterations: u32,
        salt: &::js_sys::Object,
    ) -> Pbkdf2Params {
        let mut _ret = Pbkdf2Params {
            obj: ::js_sys::Object::new(),
        };
        _ret.name(name);
        _ret.hash(hash);
        _ret.iterations(iterations);
        _ret.salt(salt);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
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
    #[doc = "Configure the `hash` field of this object\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
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
    #[doc = "Configure the `iterations` field of this object\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn iterations(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("iterations"),
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
    #[doc = "Configure the `salt` field of this object\n\n\n*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("salt"),
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
const _CONST_Pbkdf2Params: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<Pbkdf2Params> for JsValue {
        #[inline]
        fn from(val: Pbkdf2Params) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for Pbkdf2Params {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for Pbkdf2Params {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for Pbkdf2Params {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a Pbkdf2Params {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for Pbkdf2Params {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            Pbkdf2Params {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for Pbkdf2Params {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Pbkdf2Params {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for Pbkdf2Params {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for Pbkdf2Params {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<Pbkdf2Params>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(Pbkdf2Params {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for Pbkdf2Params {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Pbkdf2Params {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Pbkdf2Params) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_67f91ef924f871c1: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

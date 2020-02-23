use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `AuthenticationExtensionsClientInputs` dictionary\n\n\n*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
pub struct AuthenticationExtensionsClientInputs {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl AuthenticationExtensionsClientInputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientInputs`\n\n\n*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn new() -> AuthenticationExtensionsClientInputs {
        let mut _ret = AuthenticationExtensionsClientInputs {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `appid` field of this object\n\n\n*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn appid(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("appid"),
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
const _CONST_AuthenticationExtensionsClientInputs: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<AuthenticationExtensionsClientInputs> for JsValue {
        #[inline]
        fn from(val: AuthenticationExtensionsClientInputs) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for AuthenticationExtensionsClientInputs {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for AuthenticationExtensionsClientInputs {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for AuthenticationExtensionsClientInputs {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a AuthenticationExtensionsClientInputs {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for AuthenticationExtensionsClientInputs {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            AuthenticationExtensionsClientInputs {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for AuthenticationExtensionsClientInputs {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AuthenticationExtensionsClientInputs {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for AuthenticationExtensionsClientInputs {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for AuthenticationExtensionsClientInputs {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<AuthenticationExtensionsClientInputs>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(AuthenticationExtensionsClientInputs {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for AuthenticationExtensionsClientInputs {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AuthenticationExtensionsClientInputs {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AuthenticationExtensionsClientInputs) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0c982d8e9e106bfb: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `PushSubscriptionJSON` dictionary\n\n\n*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
pub struct PushSubscriptionJson {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl PushSubscriptionJson {
    #[doc = "Construct a new `PushSubscriptionJSON`\n\n\n*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    pub fn new() -> PushSubscriptionJson {
        let mut _ret = PushSubscriptionJson {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `endpoint` field of this object\n\n\n*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("endpoint"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "PushSubscriptionKeys",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `keys` field of this object\n\n\n*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("keys"),
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
const _CONST_PushSubscriptionJson: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<PushSubscriptionJson> for JsValue {
        #[inline]
        fn from(val: PushSubscriptionJson) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for PushSubscriptionJson {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for PushSubscriptionJson {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for PushSubscriptionJson {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a PushSubscriptionJson {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for PushSubscriptionJson {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            PushSubscriptionJson {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for PushSubscriptionJson {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PushSubscriptionJson {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for PushSubscriptionJson {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for PushSubscriptionJson {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<PushSubscriptionJson>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(PushSubscriptionJson {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for PushSubscriptionJson {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PushSubscriptionJson {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PushSubscriptionJson) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a87cb50e96a4c05b: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

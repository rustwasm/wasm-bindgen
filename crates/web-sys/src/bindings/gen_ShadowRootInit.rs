use super::*;
use js_sys::Object;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ShadowRootInit` dictionary\n\n\n*This API requires the following crate features to be activated: `ShadowRootInit`*"]
pub struct ShadowRootInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ShadowRootInit {
    #[doc = "Construct a new `ShadowRootInit`\n\n\n*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn new(mode: ShadowRootMode) -> ShadowRootInit {
        let mut _ret = ShadowRootInit {
            obj: ::js_sys::Object::new(),
        };
        _ret.mode(mode);
        return _ret;
    }
    #[cfg(all(feature = "ShadowRootMode",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `mode` field of this object\n\n\n*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn mode(&mut self, val: ShadowRootMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("mode"),
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
const _CONST_ShadowRootInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ShadowRootInit> for JsValue {
        #[inline]
        fn from(val: ShadowRootInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ShadowRootInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ShadowRootInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ShadowRootInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ShadowRootInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ShadowRootInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ShadowRootInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ShadowRootInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ShadowRootInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ShadowRootInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ShadowRootInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ShadowRootInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ShadowRootInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ShadowRootInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ShadowRootInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ShadowRootInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_3010f44274df6d95: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

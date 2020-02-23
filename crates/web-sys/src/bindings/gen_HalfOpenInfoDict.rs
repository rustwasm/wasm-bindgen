use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `HalfOpenInfoDict` dictionary\n\n\n*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
pub struct HalfOpenInfoDict {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl HalfOpenInfoDict {
    #[doc = "Construct a new `HalfOpenInfoDict`\n\n\n*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn new() -> HalfOpenInfoDict {
        let mut _ret = HalfOpenInfoDict {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `speculative` field of this object\n\n\n*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn speculative(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("speculative"),
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
const _CONST_HalfOpenInfoDict: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<HalfOpenInfoDict> for JsValue {
        #[inline]
        fn from(val: HalfOpenInfoDict) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for HalfOpenInfoDict {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for HalfOpenInfoDict {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for HalfOpenInfoDict {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a HalfOpenInfoDict {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for HalfOpenInfoDict {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            HalfOpenInfoDict {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for HalfOpenInfoDict {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HalfOpenInfoDict {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for HalfOpenInfoDict {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for HalfOpenInfoDict {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<HalfOpenInfoDict>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(HalfOpenInfoDict {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for HalfOpenInfoDict {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HalfOpenInfoDict {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HalfOpenInfoDict) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_36250d9452c8e67e: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

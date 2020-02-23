use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `BaseComputedKeyframe` dictionary\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
pub struct BaseComputedKeyframe {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl BaseComputedKeyframe {
    #[doc = "Construct a new `BaseComputedKeyframe`\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    pub fn new() -> BaseComputedKeyframe {
        let mut _ret = BaseComputedKeyframe {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[cfg(all(feature = "CompositeOperation",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `composite` field of this object\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("composite"),
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
    #[doc = "Configure the `easing` field of this object\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("easing"),
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
    #[doc = "Configure the `offset` field of this object\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("offset"),
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
    #[doc = "Configure the `simulateComputeValuesFailure` field of this object\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("simulateComputeValuesFailure"),
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
    #[doc = "Configure the `computedOffset` field of this object\n\n\n*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn computed_offset(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("computedOffset"),
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
const _CONST_BaseComputedKeyframe: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<BaseComputedKeyframe> for JsValue {
        #[inline]
        fn from(val: BaseComputedKeyframe) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for BaseComputedKeyframe {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for BaseComputedKeyframe {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for BaseComputedKeyframe {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a BaseComputedKeyframe {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for BaseComputedKeyframe {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            BaseComputedKeyframe {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for BaseComputedKeyframe {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BaseComputedKeyframe {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for BaseComputedKeyframe {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for BaseComputedKeyframe {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<BaseComputedKeyframe>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(BaseComputedKeyframe {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for BaseComputedKeyframe {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BaseComputedKeyframe {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BaseComputedKeyframe) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ad1e010ecd09406c: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

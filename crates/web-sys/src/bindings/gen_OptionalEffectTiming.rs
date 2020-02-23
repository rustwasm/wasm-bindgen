use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `OptionalEffectTiming` dictionary\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
pub struct OptionalEffectTiming {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl OptionalEffectTiming {
    #[doc = "Construct a new `OptionalEffectTiming`\n\n\n*This API requires the following crate features to be activated: `FillMode`, `OptionalEffectTiming`, `PlaybackDirection`*"]
    pub fn new() -> OptionalEffectTiming {
        let mut _ret = OptionalEffectTiming {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `delay` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("delay"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "PlaybackDirection",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `direction` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`, `PlaybackDirection`*"]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("direction"),
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
    #[doc = "Configure the `duration` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("duration"),
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
    #[doc = "Configure the `easing` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
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
    #[doc = "Configure the `endDelay` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("endDelay"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "FillMode",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `fill` field of this object\n\n\n*This API requires the following crate features to be activated: `FillMode`, `OptionalEffectTiming`*"]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("fill"),
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
    #[doc = "Configure the `iterationStart` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("iterationStart"),
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
    #[doc = "Configure the `iterations` field of this object\n\n\n*This API requires the following crate features to be activated: `OptionalEffectTiming`*"]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
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
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_OptionalEffectTiming: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<OptionalEffectTiming> for JsValue {
        #[inline]
        fn from(val: OptionalEffectTiming) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for OptionalEffectTiming {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for OptionalEffectTiming {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for OptionalEffectTiming {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a OptionalEffectTiming {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for OptionalEffectTiming {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            OptionalEffectTiming {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for OptionalEffectTiming {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OptionalEffectTiming {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for OptionalEffectTiming {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for OptionalEffectTiming {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<OptionalEffectTiming>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(OptionalEffectTiming {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for OptionalEffectTiming {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OptionalEffectTiming {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OptionalEffectTiming) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f6bf571146cc77cf: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

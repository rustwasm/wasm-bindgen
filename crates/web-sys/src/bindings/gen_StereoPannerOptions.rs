use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `StereoPannerOptions` dictionary\n\n\n*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
pub struct StereoPannerOptions {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl StereoPannerOptions {
    #[doc = "Construct a new `StereoPannerOptions`\n\n\n*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelInterpretation`, `StereoPannerOptions`*"]
    pub fn new() -> StereoPannerOptions {
        let mut _ret = StereoPannerOptions {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `channelCount` field of this object\n\n\n*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("channelCount"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "ChannelCountMode",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `channelCountMode` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("channelCountMode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "ChannelInterpretation",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `channelInterpretation` field of this object\n\n\n*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("channelInterpretation"),
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
    #[doc = "Configure the `pan` field of this object\n\n\n*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
    pub fn pan(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("pan"),
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
const _CONST_StereoPannerOptions: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<StereoPannerOptions> for JsValue {
        #[inline]
        fn from(val: StereoPannerOptions) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for StereoPannerOptions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for StereoPannerOptions {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for StereoPannerOptions {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a StereoPannerOptions {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for StereoPannerOptions {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            StereoPannerOptions {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for StereoPannerOptions {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a StereoPannerOptions {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for StereoPannerOptions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for StereoPannerOptions {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<StereoPannerOptions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(StereoPannerOptions {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for StereoPannerOptions {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            StereoPannerOptions {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const StereoPannerOptions) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_71886804eb967cf2: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

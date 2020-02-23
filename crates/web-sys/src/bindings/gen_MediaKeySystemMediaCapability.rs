use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `MediaKeySystemMediaCapability` dictionary\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
pub struct MediaKeySystemMediaCapability {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl MediaKeySystemMediaCapability {
    #[doc = "Construct a new `MediaKeySystemMediaCapability`\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn new() -> MediaKeySystemMediaCapability {
        let mut _ret = MediaKeySystemMediaCapability {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `contentType` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("contentType"),
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
    #[doc = "Configure the `robustness` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn robustness(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("robustness"),
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
const _CONST_MediaKeySystemMediaCapability: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<MediaKeySystemMediaCapability> for JsValue {
        #[inline]
        fn from(val: MediaKeySystemMediaCapability) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for MediaKeySystemMediaCapability {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for MediaKeySystemMediaCapability {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for MediaKeySystemMediaCapability {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeySystemMediaCapability {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for MediaKeySystemMediaCapability {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            MediaKeySystemMediaCapability {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for MediaKeySystemMediaCapability {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeySystemMediaCapability {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for MediaKeySystemMediaCapability {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for MediaKeySystemMediaCapability {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<MediaKeySystemMediaCapability>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(MediaKeySystemMediaCapability {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for MediaKeySystemMediaCapability {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeySystemMediaCapability {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeySystemMediaCapability) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0d0c6894fa42fa78: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

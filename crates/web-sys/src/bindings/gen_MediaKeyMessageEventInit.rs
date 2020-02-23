use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `MediaKeyMessageEventInit` dictionary\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
pub struct MediaKeyMessageEventInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl MediaKeyMessageEventInit {
    #[doc = "Construct a new `MediaKeyMessageEventInit`\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn new(
        message: &::js_sys::ArrayBuffer,
        message_type: MediaKeyMessageType,
    ) -> MediaKeyMessageEventInit {
        let mut _ret = MediaKeyMessageEventInit {
            obj: ::js_sys::Object::new(),
        };
        _ret.message(message);
        _ret.message_type(message_type);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `bubbles` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("bubbles"),
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
    #[doc = "Configure the `cancelable` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("cancelable"),
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
    #[doc = "Configure the `composed` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("composed"),
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
    #[doc = "Configure the `message` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
    pub fn message(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("message"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "MediaKeyMessageType",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `messageType` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
    pub fn message_type(&mut self, val: MediaKeyMessageType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("messageType"),
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
const _CONST_MediaKeyMessageEventInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<MediaKeyMessageEventInit> for JsValue {
        #[inline]
        fn from(val: MediaKeyMessageEventInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for MediaKeyMessageEventInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for MediaKeyMessageEventInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for MediaKeyMessageEventInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeyMessageEventInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for MediaKeyMessageEventInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            MediaKeyMessageEventInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for MediaKeyMessageEventInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeyMessageEventInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for MediaKeyMessageEventInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for MediaKeyMessageEventInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<MediaKeyMessageEventInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(MediaKeyMessageEventInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for MediaKeyMessageEventInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeyMessageEventInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeyMessageEventInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5f692e599a847eda: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

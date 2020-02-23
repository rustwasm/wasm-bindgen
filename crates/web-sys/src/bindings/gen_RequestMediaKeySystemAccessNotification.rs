use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `RequestMediaKeySystemAccessNotification` dictionary\n\n\n*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
pub struct RequestMediaKeySystemAccessNotification {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl RequestMediaKeySystemAccessNotification {
    #[doc = "Construct a new `RequestMediaKeySystemAccessNotification`\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    pub fn new(
        key_system: &str,
        status: MediaKeySystemStatus,
    ) -> RequestMediaKeySystemAccessNotification {
        let mut _ret = RequestMediaKeySystemAccessNotification {
            obj: ::js_sys::Object::new(),
        };
        _ret.key_system(key_system);
        _ret.status(status);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `keySystem` field of this object\n\n\n*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    pub fn key_system(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("keySystem"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "MediaKeySystemStatus",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `status` field of this object\n\n\n*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    pub fn status(&mut self, val: MediaKeySystemStatus) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("status"),
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
const _CONST_RequestMediaKeySystemAccessNotification: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<RequestMediaKeySystemAccessNotification> for JsValue {
        #[inline]
        fn from(val: RequestMediaKeySystemAccessNotification) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for RequestMediaKeySystemAccessNotification {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for RequestMediaKeySystemAccessNotification {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for RequestMediaKeySystemAccessNotification {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a RequestMediaKeySystemAccessNotification {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for RequestMediaKeySystemAccessNotification {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            RequestMediaKeySystemAccessNotification {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for RequestMediaKeySystemAccessNotification {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RequestMediaKeySystemAccessNotification {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for RequestMediaKeySystemAccessNotification {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for RequestMediaKeySystemAccessNotification {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<RequestMediaKeySystemAccessNotification>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(RequestMediaKeySystemAccessNotification {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for RequestMediaKeySystemAccessNotification {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RequestMediaKeySystemAccessNotification {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RequestMediaKeySystemAccessNotification) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f9058e15b569d3d0: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

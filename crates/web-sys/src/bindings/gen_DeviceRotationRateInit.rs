use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `DeviceRotationRateInit` dictionary\n\n\n*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
pub struct DeviceRotationRateInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl DeviceRotationRateInit {
    #[doc = "Construct a new `DeviceRotationRateInit`\n\n\n*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn new() -> DeviceRotationRateInit {
        let mut _ret = DeviceRotationRateInit {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `alpha` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("alpha"),
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
    #[doc = "Configure the `beta` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("beta"),
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
    #[doc = "Configure the `gamma` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("gamma"),
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
const _CONST_DeviceRotationRateInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<DeviceRotationRateInit> for JsValue {
        #[inline]
        fn from(val: DeviceRotationRateInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for DeviceRotationRateInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for DeviceRotationRateInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for DeviceRotationRateInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceRotationRateInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for DeviceRotationRateInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            DeviceRotationRateInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for DeviceRotationRateInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceRotationRateInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for DeviceRotationRateInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for DeviceRotationRateInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<DeviceRotationRateInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(DeviceRotationRateInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for DeviceRotationRateInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceRotationRateInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceRotationRateInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_40689071e63062c7: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

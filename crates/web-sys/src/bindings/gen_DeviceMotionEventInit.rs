use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `DeviceMotionEventInit` dictionary\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
pub struct DeviceMotionEventInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl DeviceMotionEventInit {
    #[doc = "Construct a new `DeviceMotionEventInit`\n\n\n*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    pub fn new() -> DeviceMotionEventInit {
        let mut _ret = DeviceMotionEventInit {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `bubbles` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
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
    #[doc = "Configure the `cancelable` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
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
    #[doc = "Configure the `composed` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
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
    #[cfg(all(feature = "DeviceAccelerationInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `acceleration` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("acceleration"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DeviceAccelerationInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `accelerationIncludingGravity` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
    pub fn acceleration_including_gravity(&mut self, val: &DeviceAccelerationInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("accelerationIncludingGravity"),
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
    #[doc = "Configure the `interval` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
    pub fn interval(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("interval"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DeviceRotationRateInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `rotationRate` field of this object\n\n\n*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
    pub fn rotation_rate(&mut self, val: &DeviceRotationRateInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("rotationRate"),
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
const _CONST_DeviceMotionEventInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<DeviceMotionEventInit> for JsValue {
        #[inline]
        fn from(val: DeviceMotionEventInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for DeviceMotionEventInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for DeviceMotionEventInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for DeviceMotionEventInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a DeviceMotionEventInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for DeviceMotionEventInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            DeviceMotionEventInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for DeviceMotionEventInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DeviceMotionEventInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for DeviceMotionEventInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for DeviceMotionEventInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<DeviceMotionEventInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(DeviceMotionEventInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for DeviceMotionEventInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DeviceMotionEventInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DeviceMotionEventInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9dd55c20c67a83e1: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

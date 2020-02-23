use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `PeriodicWaveConstraints` dictionary\n\n\n*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
pub struct PeriodicWaveConstraints {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl PeriodicWaveConstraints {
    #[doc = "Construct a new `PeriodicWaveConstraints`\n\n\n*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub fn new() -> PeriodicWaveConstraints {
        let mut _ret = PeriodicWaveConstraints {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `disableNormalization` field of this object\n\n\n*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("disableNormalization"),
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
const _CONST_PeriodicWaveConstraints: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<PeriodicWaveConstraints> for JsValue {
        #[inline]
        fn from(val: PeriodicWaveConstraints) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for PeriodicWaveConstraints {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for PeriodicWaveConstraints {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for PeriodicWaveConstraints {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a PeriodicWaveConstraints {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for PeriodicWaveConstraints {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            PeriodicWaveConstraints {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for PeriodicWaveConstraints {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PeriodicWaveConstraints {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for PeriodicWaveConstraints {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for PeriodicWaveConstraints {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<PeriodicWaveConstraints>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(PeriodicWaveConstraints {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for PeriodicWaveConstraints {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PeriodicWaveConstraints {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PeriodicWaveConstraints) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_73473b9faa5e8cb3: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ContextAttributes2D` dictionary\n\n\n*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
pub struct ContextAttributes2d {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ContextAttributes2d {
    #[doc = "Construct a new `ContextAttributes2D`\n\n\n*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn new() -> ContextAttributes2d {
        let mut _ret = ContextAttributes2d {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `alpha` field of this object\n\n\n*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
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
    #[doc = "Configure the `willReadFrequently` field of this object\n\n\n*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn will_read_frequently(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("willReadFrequently"),
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
const _CONST_ContextAttributes2d: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ContextAttributes2d> for JsValue {
        #[inline]
        fn from(val: ContextAttributes2d) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ContextAttributes2d {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ContextAttributes2d {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ContextAttributes2d {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ContextAttributes2d {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ContextAttributes2d {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ContextAttributes2d {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ContextAttributes2d {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ContextAttributes2d {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ContextAttributes2d {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ContextAttributes2d {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ContextAttributes2d>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ContextAttributes2d {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ContextAttributes2d {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ContextAttributes2d {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ContextAttributes2d) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_8f3347f2cfab8a81: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

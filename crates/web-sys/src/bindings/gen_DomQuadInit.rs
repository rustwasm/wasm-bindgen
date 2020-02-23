use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `DOMQuadInit` dictionary\n\n\n*This API requires the following crate features to be activated: `DomQuadInit`*"]
pub struct DomQuadInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl DomQuadInit {
    #[doc = "Construct a new `DOMQuadInit`\n\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn new() -> DomQuadInit {
        let mut _ret = DomQuadInit {
            obj: ::js_sys::Object::new(),
        };
        return _ret;
    }
    #[cfg(all(feature = "DomPointInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `p1` field of this object\n\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p1(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.obj.as_ref(), &JsValue::from("p1"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DomPointInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `p2` field of this object\n\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p2(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.obj.as_ref(), &JsValue::from("p2"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DomPointInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `p3` field of this object\n\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p3(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.obj.as_ref(), &JsValue::from("p3"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DomPointInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `p4` field of this object\n\n\n*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p4(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.obj.as_ref(), &JsValue::from("p4"), &JsValue::from(val));
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
const _CONST_DomQuadInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<DomQuadInit> for JsValue {
        #[inline]
        fn from(val: DomQuadInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for DomQuadInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for DomQuadInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for DomQuadInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a DomQuadInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for DomQuadInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            DomQuadInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for DomQuadInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomQuadInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for DomQuadInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for DomQuadInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<DomQuadInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(DomQuadInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for DomQuadInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomQuadInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomQuadInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_c1eddaab79805231: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `IntersectionObserverEntryInit` dictionary\n\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
pub struct IntersectionObserverEntryInit {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl IntersectionObserverEntryInit {
    #[doc = "Construct a new `IntersectionObserverEntryInit`\n\n\n*This API requires the following crate features to be activated: `DomRectInit`, `Element`, `IntersectionObserverEntryInit`*"]
    pub fn new(
        bounding_client_rect: &DomRectInit,
        intersection_rect: &DomRectInit,
        root_bounds: &DomRectInit,
        target: &Element,
        time: f64,
    ) -> IntersectionObserverEntryInit {
        let mut _ret = IntersectionObserverEntryInit {
            obj: ::js_sys::Object::new(),
        };
        _ret.bounding_client_rect(bounding_client_rect);
        _ret.intersection_rect(intersection_rect);
        _ret.root_bounds(root_bounds);
        _ret.target(target);
        _ret.time(time);
        return _ret;
    }
    #[cfg(all(feature = "DomRectInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `boundingClientRect` field of this object\n\n\n*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn bounding_client_rect(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("boundingClientRect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DomRectInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `intersectionRect` field of this object\n\n\n*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn intersection_rect(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("intersectionRect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "DomRectInit",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `rootBounds` field of this object\n\n\n*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn root_bounds(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("rootBounds"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(all(feature = "Element",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `target` field of this object\n\n\n*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    pub fn target(&mut self, val: &Element) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("target"),
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
    #[doc = "Configure the `time` field of this object\n\n\n*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub fn time(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("time"),
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
const _CONST_IntersectionObserverEntryInit: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<IntersectionObserverEntryInit> for JsValue {
        #[inline]
        fn from(val: IntersectionObserverEntryInit) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for IntersectionObserverEntryInit {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for IntersectionObserverEntryInit {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for IntersectionObserverEntryInit {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a IntersectionObserverEntryInit {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for IntersectionObserverEntryInit {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            IntersectionObserverEntryInit {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for IntersectionObserverEntryInit {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IntersectionObserverEntryInit {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for IntersectionObserverEntryInit {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for IntersectionObserverEntryInit {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<IntersectionObserverEntryInit>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(IntersectionObserverEntryInit {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for IntersectionObserverEntryInit {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IntersectionObserverEntryInit {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IntersectionObserverEntryInit) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1f36057fae050d17: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

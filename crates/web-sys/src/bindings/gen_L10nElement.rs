use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `L10nElement` dictionary\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
pub struct L10nElement {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl L10nElement {
    #[doc = "Construct a new `L10nElement`\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn new(l10n_id: &str, local_name: &str, namespace_uri: &str) -> L10nElement {
        let mut _ret = L10nElement {
            obj: ::js_sys::Object::new(),
        };
        _ret.l10n_id(l10n_id);
        _ret.local_name(local_name);
        _ret.namespace_uri(namespace_uri);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `l10nArgs` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_args(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("l10nArgs"),
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
    #[doc = "Configure the `l10nAttrs` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("l10nAttrs"),
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
    #[doc = "Configure the `l10nId` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn l10n_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("l10nId"),
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
    #[doc = "Configure the `localName` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn local_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("localName"),
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
    #[doc = "Configure the `namespaceURI` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn namespace_uri(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("namespaceURI"),
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
    #[doc = "Configure the `type` field of this object\n\n\n*This API requires the following crate features to be activated: `L10nElement`*"]
    pub fn type_(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("type"),
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
const _CONST_L10nElement: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<L10nElement> for JsValue {
        #[inline]
        fn from(val: L10nElement) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for L10nElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for L10nElement {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for L10nElement {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a L10nElement {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for L10nElement {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            L10nElement {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for L10nElement {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a L10nElement {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for L10nElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for L10nElement {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<L10nElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(L10nElement {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for L10nElement {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            L10nElement {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const L10nElement) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1b8e5320e4e6a207: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `ClientRectsAndTexts` dictionary\n\n\n*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
pub struct ClientRectsAndTexts {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl ClientRectsAndTexts {
    #[doc = "Construct a new `ClientRectsAndTexts`\n\n\n*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    pub fn new(
        rect_list: &DomRectList,
        text_list: &::wasm_bindgen::JsValue,
    ) -> ClientRectsAndTexts {
        let mut _ret = ClientRectsAndTexts {
            obj: ::js_sys::Object::new(),
        };
        _ret.rect_list(rect_list);
        _ret.text_list(text_list);
        return _ret;
    }
    #[cfg(all(feature = "DomRectList",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `rectList` field of this object\n\n\n*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    pub fn rect_list(&mut self, val: &DomRectList) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("rectList"),
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
    #[doc = "Configure the `textList` field of this object\n\n\n*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    pub fn text_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("textList"),
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
const _CONST_ClientRectsAndTexts: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<ClientRectsAndTexts> for JsValue {
        #[inline]
        fn from(val: ClientRectsAndTexts) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for ClientRectsAndTexts {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for ClientRectsAndTexts {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for ClientRectsAndTexts {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a ClientRectsAndTexts {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for ClientRectsAndTexts {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            ClientRectsAndTexts {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for ClientRectsAndTexts {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ClientRectsAndTexts {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for ClientRectsAndTexts {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for ClientRectsAndTexts {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<ClientRectsAndTexts>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(ClientRectsAndTexts {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for ClientRectsAndTexts {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ClientRectsAndTexts {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ClientRectsAndTexts) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9fe6f027eee488e1: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CDATASection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CDATASection)\n\n*This API requires the following crate features to be activated: `CdataSection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CdataSection {
    obj: Text,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CdataSection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CdataSection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(67u32);
            inform(68u32);
            inform(65u32);
            inform(84u32);
            inform(65u32);
            inform(83u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CdataSection {
        type Target = Text;
        #[inline]
        fn deref(&self) -> &Text {
            &self.obj
        }
    }
    impl IntoWasmAbi for CdataSection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CdataSection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CdataSection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CdataSection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CdataSection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CdataSection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CdataSection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CdataSection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CdataSection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CdataSection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CdataSection {
        #[inline]
        fn from(obj: JsValue) -> CdataSection {
            CdataSection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CdataSection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CdataSection> for CdataSection {
        #[inline]
        fn as_ref(&self) -> &CdataSection {
            self
        }
    }
    impl From<CdataSection> for JsValue {
        #[inline]
        fn from(obj: CdataSection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CdataSection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CDATASection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CDATASection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CDATASection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CdataSection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CdataSection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CdataSection> for Text {
    #[inline]
    fn from(obj: CdataSection) -> Text {
        use wasm_bindgen::JsCast;
        Text::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Text> for CdataSection {
    #[inline]
    fn as_ref(&self) -> &Text {
        use wasm_bindgen::JsCast;
        Text::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CdataSection> for CharacterData {
    #[inline]
    fn from(obj: CdataSection) -> CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CharacterData> for CdataSection {
    #[inline]
    fn as_ref(&self) -> &CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CdataSection> for Node {
    #[inline]
    fn from(obj: CdataSection) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for CdataSection {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CdataSection> for EventTarget {
    #[inline]
    fn from(obj: CdataSection) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for CdataSection {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CdataSection> for ::js_sys::Object {
    #[inline]
    fn from(obj: CdataSection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CdataSection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_40c649d5ae87e689: [u8; 153usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}W\0\0\0\0\0\x01\0\0\x02\x0CCDATASection\x1E__widl_instanceof_CDATASection\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

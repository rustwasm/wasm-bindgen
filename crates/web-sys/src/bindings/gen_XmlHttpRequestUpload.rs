use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XMLHttpRequestUpload` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestUpload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestUpload`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XmlHttpRequestUpload {
    obj: XmlHttpRequestEventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XmlHttpRequestUpload: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XmlHttpRequestUpload {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(20u32);
            inform(88u32);
            inform(77u32);
            inform(76u32);
            inform(72u32);
            inform(116u32);
            inform(116u32);
            inform(112u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
            inform(85u32);
            inform(112u32);
            inform(108u32);
            inform(111u32);
            inform(97u32);
            inform(100u32);
        }
    }
    impl core::ops::Deref for XmlHttpRequestUpload {
        type Target = XmlHttpRequestEventTarget;
        #[inline]
        fn deref(&self) -> &XmlHttpRequestEventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for XmlHttpRequestUpload {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XmlHttpRequestUpload {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XmlHttpRequestUpload {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XmlHttpRequestUpload {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XmlHttpRequestUpload {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XmlHttpRequestUpload {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XmlHttpRequestUpload {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XmlHttpRequestUpload {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XmlHttpRequestUpload>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XmlHttpRequestUpload {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XmlHttpRequestUpload {
        #[inline]
        fn from(obj: JsValue) -> XmlHttpRequestUpload {
            XmlHttpRequestUpload { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XmlHttpRequestUpload {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XmlHttpRequestUpload> for XmlHttpRequestUpload {
        #[inline]
        fn as_ref(&self) -> &XmlHttpRequestUpload {
            self
        }
    }
    impl From<XmlHttpRequestUpload> for JsValue {
        #[inline]
        fn from(obj: XmlHttpRequestUpload) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XmlHttpRequestUpload {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XMLHttpRequestUpload(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XMLHttpRequestUpload(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XMLHttpRequestUpload(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XmlHttpRequestUpload { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XmlHttpRequestUpload) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XmlHttpRequestUpload> for XmlHttpRequestEventTarget {
    #[inline]
    fn from(obj: XmlHttpRequestUpload) -> XmlHttpRequestEventTarget {
        use wasm_bindgen::JsCast;
        XmlHttpRequestEventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<XmlHttpRequestEventTarget> for XmlHttpRequestUpload {
    #[inline]
    fn as_ref(&self) -> &XmlHttpRequestEventTarget {
        use wasm_bindgen::JsCast;
        XmlHttpRequestEventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequestUpload> for EventTarget {
    #[inline]
    fn from(obj: XmlHttpRequestUpload) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for XmlHttpRequestUpload {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequestUpload> for ::js_sys::Object {
    #[inline]
    fn from(obj: XmlHttpRequestUpload) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XmlHttpRequestUpload {
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
pub static __WASM_BINDGEN_GENERATED_d8be4ebe0e7a64cc: [u8; 169usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}g\0\0\0\0\0\x01\0\0\x02\x14XMLHttpRequestUpload&__widl_instanceof_XMLHttpRequestUpload\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

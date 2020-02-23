use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGUnitTypes` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGUnitTypes)\n\n*This API requires the following crate features to be activated: `SvgUnitTypes`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgUnitTypes {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgUnitTypes: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgUnitTypes {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(85u32);
            inform(110u32);
            inform(105u32);
            inform(116u32);
            inform(84u32);
            inform(121u32);
            inform(112u32);
            inform(101u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for SvgUnitTypes {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgUnitTypes {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgUnitTypes {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgUnitTypes {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgUnitTypes {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgUnitTypes {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgUnitTypes {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgUnitTypes {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgUnitTypes {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgUnitTypes>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgUnitTypes {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgUnitTypes {
        #[inline]
        fn from(obj: JsValue) -> SvgUnitTypes {
            SvgUnitTypes { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgUnitTypes {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgUnitTypes> for SvgUnitTypes {
        #[inline]
        fn as_ref(&self) -> &SvgUnitTypes {
            self
        }
    }
    impl From<SvgUnitTypes> for JsValue {
        #[inline]
        fn from(obj: SvgUnitTypes) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgUnitTypes {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGUnitTypes(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGUnitTypes(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGUnitTypes(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgUnitTypes { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgUnitTypes) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgUnitTypes> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgUnitTypes) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgUnitTypes {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
impl SvgUnitTypes {
    pub const SVG_UNIT_TYPE_UNKNOWN: u16 = 0i64 as u16;
}
impl SvgUnitTypes {
    pub const SVG_UNIT_TYPE_USERSPACEONUSE: u16 = 1u64 as u16;
}
impl SvgUnitTypes {
    pub const SVG_UNIT_TYPE_OBJECTBOUNDINGBOX: u16 = 2u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5ba14b383d338c74: [u8; 153usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}W\0\0\0\0\0\x01\0\0\x02\x0CSVGUnitTypes\x1E__widl_instanceof_SVGUnitTypes\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

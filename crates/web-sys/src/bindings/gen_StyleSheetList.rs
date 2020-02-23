use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `StyleSheetList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)\n\n*This API requires the following crate features to be activated: `StyleSheetList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct StyleSheetList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_StyleSheetList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for StyleSheetList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(83u32);
            inform(104u32);
            inform(101u32);
            inform(101u32);
            inform(116u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for StyleSheetList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for StyleSheetList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for StyleSheetList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a StyleSheetList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for StyleSheetList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            StyleSheetList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for StyleSheetList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a StyleSheetList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for StyleSheetList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<StyleSheetList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(StyleSheetList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for StyleSheetList {
        #[inline]
        fn from(obj: JsValue) -> StyleSheetList {
            StyleSheetList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for StyleSheetList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<StyleSheetList> for StyleSheetList {
        #[inline]
        fn as_ref(&self) -> &StyleSheetList {
            self
        }
    }
    impl From<StyleSheetList> for JsValue {
        #[inline]
        fn from(obj: StyleSheetList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for StyleSheetList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_StyleSheetList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_StyleSheetList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_StyleSheetList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            StyleSheetList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const StyleSheetList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<StyleSheetList> for ::js_sys::Object {
    #[inline]
    fn from(obj: StyleSheetList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for StyleSheetList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_StyleSheetList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&StyleSheetList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl StyleSheetList {
    #[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)\n\n*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<StyleSheet> {
        #[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_StyleSheetList(
                self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_StyleSheetList(
            self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_StyleSheetList(self_, index)
            };
            <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_StyleSheetList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&StyleSheetList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl StyleSheetList {
    #[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<StyleSheet> {
        #[cfg(all(feature = "StyleSheet", feature = "StyleSheetList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_StyleSheetList(
                self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_StyleSheetList(
            self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_StyleSheetList(self_, index)
            };
            <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheetList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_StyleSheetList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheetList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl StyleSheetList {
    #[cfg(all(feature = "StyleSheetList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)\n\n*This API requires the following crate features to be activated: `StyleSheetList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "StyleSheetList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_StyleSheetList(
                self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_StyleSheetList(
            self_: <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheetList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_StyleSheetList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_aed7c051e4b062bd: [u8; 379usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}9\x01\0\0\0\0\x04\0\0\x02\x0EStyleSheetList __widl_instanceof_StyleSheetList\0\0\0\0\x1C__widl_f_item_StyleSheetList\0\0\0\x01\x0EStyleSheetList\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x1B__widl_f_get_StyleSheetList\0\0\0\x01\x0EStyleSheetList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1E__widl_f_length_StyleSheetList\0\0\0\x01\x0EStyleSheetList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

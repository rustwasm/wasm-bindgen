use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `StyleSheet` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct StyleSheet {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_StyleSheet: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for StyleSheet {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
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
        }
    }
    impl core::ops::Deref for StyleSheet {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for StyleSheet {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for StyleSheet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a StyleSheet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for StyleSheet {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            StyleSheet {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for StyleSheet {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a StyleSheet {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for StyleSheet {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<StyleSheet>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(StyleSheet {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for StyleSheet {
        #[inline]
        fn from(obj: JsValue) -> StyleSheet {
            StyleSheet { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for StyleSheet {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<StyleSheet> for StyleSheet {
        #[inline]
        fn as_ref(&self) -> &StyleSheet {
            self
        }
    }
    impl From<StyleSheet> for JsValue {
        #[inline]
        fn from(obj: StyleSheet) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for StyleSheet {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_StyleSheet(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_StyleSheet(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_StyleSheet(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            StyleSheet { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const StyleSheet) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<StyleSheet> for ::js_sys::Object {
    #[inline]
    fn from(obj: StyleSheet) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for StyleSheet {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_StyleSheet(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_StyleSheet(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Node", feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_owner_node_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "Node", feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `ownerNode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)\n\n*This API requires the following crate features to be activated: `Node`, `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn owner_node(&self) -> Option<Node> {
        #[cfg(all(feature = "Node", feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_owner_node_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_owner_node_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_owner_node_StyleSheet(self_)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_style_sheet_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `parentStyleSheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn parent_style_sheet(&self) -> Option<StyleSheet> {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_style_sheet_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_style_sheet_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_style_sheet_StyleSheet(self_)
            };
            <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_title_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `title` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn title(&self) -> Option<String> {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_title_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_title_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_title_StyleSheet(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaList", feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <MediaList as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "MediaList", feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)\n\n*This API requires the following crate features to be activated: `MediaList`, `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> MediaList {
        #[cfg(all(feature = "MediaList", feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_StyleSheet(self_)
            };
            <MediaList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&StyleSheet as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_StyleSheet(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_StyleSheet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&StyleSheet as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl StyleSheet {
    #[cfg(all(feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)\n\n*This API requires the following crate features to be activated: `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_StyleSheet(
                self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_StyleSheet(
            self_: <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(disabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&StyleSheet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_StyleSheet(self_, disabled)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b348d98bf1362ec9: [u8; 758usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB4\x02\0\0\0\0\t\0\0\x02\nStyleSheet\x1C__widl_instanceof_StyleSheet\0\0\0\0\x18__widl_f_type_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x18__widl_f_href_StyleSheet\x01\0\0\x01\nStyleSheet\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\x1E__widl_f_owner_node_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\townerNode\x01\x01\x05self_\townerNode\0\0\0&__widl_f_parent_style_sheet_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\x10parentStyleSheet\x01\x01\x05self_\x10parentStyleSheet\0\0\0\x19__widl_f_title_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\x05title\x01\x01\x05self_\x05title\0\0\0\x19__widl_f_media_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0\x1C__widl_f_disabled_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0 __widl_f_set_disabled_StyleSheet\0\0\0\x01\nStyleSheet\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

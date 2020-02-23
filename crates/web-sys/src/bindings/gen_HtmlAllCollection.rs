use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLAllCollection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlAllCollection {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlAllCollection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlAllCollection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(65u32);
            inform(108u32);
            inform(108u32);
            inform(67u32);
            inform(111u32);
            inform(108u32);
            inform(108u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for HtmlAllCollection {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlAllCollection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlAllCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlAllCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlAllCollection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlAllCollection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlAllCollection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlAllCollection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlAllCollection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlAllCollection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlAllCollection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlAllCollection {
        #[inline]
        fn from(obj: JsValue) -> HtmlAllCollection {
            HtmlAllCollection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlAllCollection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlAllCollection> for HtmlAllCollection {
        #[inline]
        fn as_ref(&self) -> &HtmlAllCollection {
            self
        }
    }
    impl From<HtmlAllCollection> for JsValue {
        #[inline]
        fn from(obj: HtmlAllCollection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlAllCollection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLAllCollection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLAllCollection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLAllCollection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlAllCollection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlAllCollection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlAllCollection> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlAllCollection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlAllCollection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_with_index_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    #[allow(clippy::all)]
    pub fn item_with_index(&self, index: u32) -> Option<Node> {
        #[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_with_index_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_with_index_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_with_index_HTMLAllCollection(self_, index)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_with_name_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    #[allow(clippy::all)]
    pub fn item_with_name(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlAllCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_with_name_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_with_name_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_item_with_name_HTMLAllCollection(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlAllCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_HTMLAllCollection(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Node> as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<Node> {
        #[cfg(all(feature = "HtmlAllCollection", feature = "Node",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_HTMLAllCollection(self_, index)
            };
            <Option<Node> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlAllCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_HTMLAllCollection(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAllCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_HTMLAllCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAllCollection as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlAllCollection {
    #[cfg(all(feature = "HtmlAllCollection",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)\n\n*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "HtmlAllCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_HTMLAllCollection(
                self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_HTMLAllCollection(
            self_: <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAllCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_HTMLAllCollection(self_)
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
pub static __WASM_BINDGEN_GENERATED_77d954d5a9ae794a: [u8; 688usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x02\0\0\0\0\x07\0\0\x02\x11HTMLAllCollection#__widl_instanceof_HTMLAllCollection\0\0\0\0*__widl_f_item_with_index_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0)__widl_f_item_with_name_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\0\x01\x02\x05self_\x04name\x04item\0\0\0%__widl_f_named_item_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0)__widl_f_get_with_index_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0(__widl_f_get_with_name_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0!__widl_f_length_HTMLAllCollection\0\0\0\x01\x11HTMLAllCollection\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

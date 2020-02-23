use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PluginArray` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PluginArray {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PluginArray: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PluginArray {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(80u32);
            inform(108u32);
            inform(117u32);
            inform(103u32);
            inform(105u32);
            inform(110u32);
            inform(65u32);
            inform(114u32);
            inform(114u32);
            inform(97u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for PluginArray {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for PluginArray {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PluginArray {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PluginArray {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PluginArray {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PluginArray {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PluginArray {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PluginArray {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PluginArray {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PluginArray>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PluginArray {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PluginArray {
        #[inline]
        fn from(obj: JsValue) -> PluginArray {
            PluginArray { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PluginArray {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PluginArray> for PluginArray {
        #[inline]
        fn as_ref(&self) -> &PluginArray {
            self
        }
    }
    impl From<PluginArray> for JsValue {
        #[inline]
        fn from(obj: PluginArray) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PluginArray {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PluginArray(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PluginArray(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PluginArray(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PluginArray { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PluginArray) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PluginArray> for ::js_sys::Object {
    #[inline]
    fn from(obj: PluginArray) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PluginArray {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Plugin", feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PluginArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Plugin> as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<Plugin> {
        #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_PluginArray(self_, index)
            };
            <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin", feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PluginArray as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Plugin> as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<Plugin> {
        #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_PluginArray(self_, name)
            };
            <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_refresh_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PluginArray as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `refresh()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn refresh(&self) {
        #[cfg(all(feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_refresh_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_refresh_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_refresh_PluginArray(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_refresh_with_reload_documents_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PluginArray as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `refresh()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn refresh_with_reload_documents(&self, reload_documents: bool) {
        #[cfg(all(feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_refresh_with_reload_documents_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                reload_documents: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_refresh_with_reload_documents_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            reload_documents: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(reload_documents);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let reload_documents =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(reload_documents);
                __widl_f_refresh_with_reload_documents_PluginArray(self_, reload_documents)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Plugin", feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PluginArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Plugin> as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<Plugin> {
        #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_PluginArray(self_, index)
            };
            <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin", feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PluginArray as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Plugin> as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<Plugin> {
        #[cfg(all(feature = "Plugin", feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_PluginArray(self_, name)
            };
            <Option<Plugin> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PluginArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_PluginArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PluginArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl PluginArray {
    #[cfg(all(feature = "PluginArray",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)\n\n*This API requires the following crate features to be activated: `PluginArray`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "PluginArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_PluginArray(
                self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_PluginArray(
            self_: <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PluginArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_PluginArray(self_)
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
pub static __WASM_BINDGEN_GENERATED_710893a1e0f30128: [u8; 690usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}p\x02\0\0\0\0\x08\0\0\x02\x0BPluginArray\x1D__widl_instanceof_PluginArray\0\0\0\0\x19__widl_f_item_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x1F__widl_f_named_item_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0\x1C__widl_f_refresh_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\0\x01\x01\x05self_\x07refresh\0\0\02__widl_f_refresh_with_reload_documents_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\0\x01\x02\x05self_\x10reload_documents\x07refresh\0\0\0#__widl_f_get_with_index_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\"__widl_f_get_with_name_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0\x1B__widl_f_length_PluginArray\0\0\0\x01\x0BPluginArray\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

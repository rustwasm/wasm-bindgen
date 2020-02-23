use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `NamedNodeMap` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)\n\n*This API requires the following crate features to be activated: `NamedNodeMap`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct NamedNodeMap {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_NamedNodeMap: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for NamedNodeMap {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(78u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(100u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(77u32);
            inform(97u32);
            inform(112u32);
        }
    }
    impl core::ops::Deref for NamedNodeMap {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for NamedNodeMap {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for NamedNodeMap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a NamedNodeMap {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for NamedNodeMap {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            NamedNodeMap {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for NamedNodeMap {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a NamedNodeMap {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for NamedNodeMap {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<NamedNodeMap>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(NamedNodeMap {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for NamedNodeMap {
        #[inline]
        fn from(obj: JsValue) -> NamedNodeMap {
            NamedNodeMap { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for NamedNodeMap {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<NamedNodeMap> for NamedNodeMap {
        #[inline]
        fn as_ref(&self) -> &NamedNodeMap {
            self
        }
    }
    impl From<NamedNodeMap> for JsValue {
        #[inline]
        fn from(obj: NamedNodeMap) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for NamedNodeMap {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_NamedNodeMap(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_NamedNodeMap(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_NamedNodeMap(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            NamedNodeMap { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const NamedNodeMap) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<NamedNodeMap> for ::js_sys::Object {
    #[inline]
    fn from(obj: NamedNodeMap) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for NamedNodeMap {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_named_item_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `getNamedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn get_named_item(&self, name: &str) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_named_item_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_named_item_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_named_item_NamedNodeMap(self_, name)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_named_item_ns_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `getNamedItemNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn get_named_item_ns(&self, namespace_uri: Option<&str>, local_name: &str) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_named_item_ns_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_named_item_ns_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace_uri);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace_uri =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace_uri);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_get_named_item_ns_NamedNodeMap(self_, namespace_uri, local_name)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_NamedNodeMap(self_, index)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_named_item_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Attr as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `removeNamedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn remove_named_item(&self, name: &str) -> Result<Attr, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_named_item_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_named_item_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_remove_named_item_NamedNodeMap(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Attr as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_named_item_ns_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Attr as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `removeNamedItemNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn remove_named_item_ns(
        &self,
        namespace_uri: Option<&str>,
        local_name: &str,
    ) -> Result<Attr, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_named_item_ns_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_named_item_ns_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            namespace_uri: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            local_name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Attr as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(namespace_uri);
            drop(local_name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let namespace_uri =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(namespace_uri);
                let local_name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(local_name);
                __widl_f_remove_named_item_ns_NamedNodeMap(self_, namespace_uri, local_name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Attr as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_named_item_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <&Attr as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `setNamedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn set_named_item(&self, arg: &Attr) -> Result<Option<Attr>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_named_item_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arg: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_named_item_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arg: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let arg = <&Attr as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arg);
                __widl_f_set_named_item_NamedNodeMap(self_, arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_named_item_ns_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <&Attr as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `setNamedItemNS()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn set_named_item_ns(&self, arg: &Attr) -> Result<Option<Attr>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_named_item_ns_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                arg: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_named_item_ns_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            arg: <&Attr as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(arg);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let arg = <&Attr as wasm_bindgen::convert::IntoWasmAbi>::into_abi(arg);
                __widl_f_set_named_item_ns_NamedNodeMap(self_, arg)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_NamedNodeMap(self_, name)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Attr> as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<Attr> {
        #[cfg(all(feature = "Attr", feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_NamedNodeMap(self_, index)
            };
            <Option<Attr> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NamedNodeMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_NamedNodeMap() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&NamedNodeMap as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl NamedNodeMap {
    #[cfg(all(feature = "NamedNodeMap",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)\n\n*This API requires the following crate features to be activated: `NamedNodeMap`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "NamedNodeMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_NamedNodeMap(
                self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_NamedNodeMap(
            self_: <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&NamedNodeMap as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_NamedNodeMap(self_)
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
pub static __WASM_BINDGEN_GENERATED_e588160f2a5d968c: [u8; 1031usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC5\x03\0\0\0\0\x0B\0\0\x02\x0CNamedNodeMap\x1E__widl_instanceof_NamedNodeMap\0\0\0\0$__widl_f_get_named_item_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x02\x05self_\x04name\x0CgetNamedItem\0\0\0'__widl_f_get_named_item_ns_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x03\x05self_\rnamespace_uri\nlocal_name\x0EgetNamedItemNS\0\0\0\x1A__widl_f_item_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0'__widl_f_remove_named_item_NamedNodeMap\x01\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x02\x05self_\x04name\x0FremoveNamedItem\0\0\0*__widl_f_remove_named_item_ns_NamedNodeMap\x01\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x03\x05self_\rnamespace_uri\nlocal_name\x11removeNamedItemNS\0\0\0$__widl_f_set_named_item_NamedNodeMap\x01\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x02\x05self_\x03arg\x0CsetNamedItem\0\0\0'__widl_f_set_named_item_ns_NamedNodeMap\x01\0\0\x01\x0CNamedNodeMap\x01\0\0\x01\x02\x05self_\x03arg\x0EsetNamedItemNS\0\0\0#__widl_f_get_with_name_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0$__widl_f_get_with_index_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1C__widl_f_length_NamedNodeMap\0\0\0\x01\x0CNamedNodeMap\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

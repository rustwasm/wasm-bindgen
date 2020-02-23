use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Plugin` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Plugin {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Plugin: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Plugin {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(6u32);
            inform(80u32);
            inform(108u32);
            inform(117u32);
            inform(103u32);
            inform(105u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Plugin {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Plugin {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Plugin {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Plugin {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Plugin {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Plugin {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Plugin {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Plugin {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Plugin {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Plugin>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Plugin {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Plugin {
        #[inline]
        fn from(obj: JsValue) -> Plugin {
            Plugin { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Plugin {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Plugin> for Plugin {
        #[inline]
        fn as_ref(&self) -> &Plugin {
            self
        }
    }
    impl From<Plugin> for JsValue {
        #[inline]
        fn from(obj: Plugin) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Plugin {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Plugin(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Plugin(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Plugin(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Plugin { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Plugin) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Plugin> for ::js_sys::Object {
    #[inline]
    fn from(obj: Plugin) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Plugin {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MimeType", feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Plugin as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "MimeType", feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/item)\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_Plugin(self_, index)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Plugin as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "MimeType", feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/namedItem)\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_Plugin(self_, name)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Plugin as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "MimeType", feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_Plugin(self_, index)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Plugin as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "MimeType", feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_Plugin(self_, name)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_description_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Plugin as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `description` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/description)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
    #[allow(clippy::all)]
    pub fn description(&self) -> String {
        #[cfg(all(feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_description_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_description_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_description_Plugin(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_filename_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Plugin as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `filename` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/filename)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
    #[allow(clippy::all)]
    pub fn filename(&self) -> String {
        #[cfg(all(feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_filename_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_filename_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_filename_Plugin(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_version_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Plugin as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `version` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/version)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
    #[allow(clippy::all)]
    pub fn version(&self) -> String {
        #[cfg(all(feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_version_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_version_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_version_Plugin(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Plugin as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/name)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_Plugin(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Plugin",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_Plugin() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Plugin as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl Plugin {
    #[cfg(all(feature = "Plugin",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/length)\n\n*This API requires the following crate features to be activated: `Plugin`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "Plugin",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_Plugin(
                self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_Plugin(
            self_: <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Plugin as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_Plugin(self_)
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
pub static __WASM_BINDGEN_GENERATED_07a9a2bde7803b79: [u8; 723usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x91\x02\0\0\0\0\n\0\0\x02\x06Plugin\x18__widl_instanceof_Plugin\0\0\0\0\x14__widl_f_item_Plugin\0\0\0\x01\x06Plugin\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x1A__widl_f_named_item_Plugin\0\0\0\x01\x06Plugin\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0\x1E__widl_f_get_with_index_Plugin\0\0\0\x01\x06Plugin\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1D__widl_f_get_with_name_Plugin\0\0\0\x01\x06Plugin\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0\x1B__widl_f_description_Plugin\0\0\0\x01\x06Plugin\x01\0\x01\x0Bdescription\x01\x01\x05self_\x0Bdescription\0\0\0\x18__widl_f_filename_Plugin\0\0\0\x01\x06Plugin\x01\0\x01\x08filename\x01\x01\x05self_\x08filename\0\0\0\x17__widl_f_version_Plugin\0\0\0\x01\x06Plugin\x01\0\x01\x07version\x01\x01\x05self_\x07version\0\0\0\x14__widl_f_name_Plugin\0\0\0\x01\x06Plugin\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x16__widl_f_length_Plugin\0\0\0\x01\x06Plugin\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

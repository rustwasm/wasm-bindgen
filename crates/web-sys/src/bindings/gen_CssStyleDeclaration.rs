use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSStyleDeclaration` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssStyleDeclaration {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssStyleDeclaration: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssStyleDeclaration {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(68u32);
            inform(101u32);
            inform(99u32);
            inform(108u32);
            inform(97u32);
            inform(114u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for CssStyleDeclaration {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssStyleDeclaration {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssStyleDeclaration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssStyleDeclaration {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssStyleDeclaration {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssStyleDeclaration {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssStyleDeclaration {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssStyleDeclaration {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssStyleDeclaration {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssStyleDeclaration>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssStyleDeclaration {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssStyleDeclaration {
        #[inline]
        fn from(obj: JsValue) -> CssStyleDeclaration {
            CssStyleDeclaration { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssStyleDeclaration {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssStyleDeclaration> for CssStyleDeclaration {
        #[inline]
        fn as_ref(&self) -> &CssStyleDeclaration {
            self
        }
    }
    impl From<CssStyleDeclaration> for JsValue {
        #[inline]
        fn from(obj: CssStyleDeclaration) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssStyleDeclaration {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSStyleDeclaration(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSStyleDeclaration(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSStyleDeclaration(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssStyleDeclaration { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssStyleDeclaration) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssStyleDeclaration> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssStyleDeclaration) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssStyleDeclaration {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_property_priority_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `getPropertyPriority()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn get_property_priority(&self, property: &str) -> String {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_property_priority_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_property_priority_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(property);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                __widl_f_get_property_priority_CSSStyleDeclaration(self_, property)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_property_value_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `getPropertyValue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn get_property_value(&self, property: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_property_value_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_property_value_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(property);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                __widl_f_get_property_value_CSSStyleDeclaration(self_, property)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> String {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_CSSStyleDeclaration(self_, index)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_property_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `removeProperty()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn remove_property(&self, property: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_property_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_property_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(property);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                __widl_f_remove_property_CSSStyleDeclaration(self_, property)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_property_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `setProperty()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn set_property(&self, property: &str, value: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_property_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_property_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(property);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_property_CSSStyleDeclaration(self_, property, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_property_with_priority_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `setProperty()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn set_property_with_priority(
        &self,
        property: &str,
        value: &str,
        priority: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_property_with_priority_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                priority: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_property_with_priority_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            property: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            priority: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(property);
            drop(value);
            drop(priority);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let property = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(property);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let priority = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(priority);
                __widl_f_set_property_with_priority_CSSStyleDeclaration(
                    self_, property, value, priority,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<String> {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_CSSStyleDeclaration(self_, index)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_css_text_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `cssText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn css_text(&self) -> String {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_css_text_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_css_text_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_css_text_CSSStyleDeclaration(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_css_text_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `cssText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn set_css_text(&self, css_text: &str) {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_css_text_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                css_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_css_text_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            css_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(css_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let css_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(css_text);
                __widl_f_set_css_text_CSSStyleDeclaration(self_, css_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_CSSStyleDeclaration(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssRule", feature = "CssStyleDeclaration",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parent_rule_CSSStyleDeclaration() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssStyleDeclaration as WasmDescribe>::describe();
    <Option<CssRule> as WasmDescribe>::describe();
}
impl CssStyleDeclaration {
    #[cfg(all(feature = "CssRule", feature = "CssStyleDeclaration",))]
    #[allow(bad_style)]
    #[doc = "The `parentRule` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)\n\n*This API requires the following crate features to be activated: `CssRule`, `CssStyleDeclaration`*"]
    #[allow(clippy::all)]
    pub fn parent_rule(&self) -> Option<CssRule> {
        #[cfg(all(feature = "CssRule", feature = "CssStyleDeclaration",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parent_rule_CSSStyleDeclaration(
                self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parent_rule_CSSStyleDeclaration(
            self_: <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssStyleDeclaration as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parent_rule_CSSStyleDeclaration(self_)
            };
            <Option<CssRule> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bb41c8881ca9e0b0: [u8; 1291usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC9\x04\0\0\0\0\x0C\0\0\x02\x13CSSStyleDeclaration%__widl_instanceof_CSSStyleDeclaration\0\0\0\02__widl_f_get_property_priority_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x02\x05self_\x08property\x13getPropertyPriority\0\0\0/__widl_f_get_property_value_CSSStyleDeclaration\x01\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x02\x05self_\x08property\x10getPropertyValue\0\0\0!__widl_f_item_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0,__widl_f_remove_property_CSSStyleDeclaration\x01\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x02\x05self_\x08property\x0EremoveProperty\0\0\0)__widl_f_set_property_CSSStyleDeclaration\x01\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x03\x05self_\x08property\x05value\x0BsetProperty\0\0\07__widl_f_set_property_with_priority_CSSStyleDeclaration\x01\0\0\x01\x13CSSStyleDeclaration\x01\0\0\x01\x04\x05self_\x08property\x05value\x08priority\x0BsetProperty\0\0\0 __widl_f_get_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0%__widl_f_css_text_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\x01\x07cssText\x01\x01\x05self_\x07cssText\0\0\0)__widl_f_set_css_text_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\x02\x07cssText\x01\x02\x05self_\x08css_text\x07cssText\0\0\0#__widl_f_length_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0(__widl_f_parent_rule_CSSStyleDeclaration\0\0\0\x01\x13CSSStyleDeclaration\x01\0\x01\nparentRule\x01\x01\x05self_\nparentRule\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

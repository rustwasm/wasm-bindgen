use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFormControlsCollection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection)\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFormControlsCollection {
    obj: HtmlCollection,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFormControlsCollection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFormControlsCollection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(26u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(70u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(114u32);
            inform(111u32);
            inform(108u32);
            inform(115u32);
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
    impl core::ops::Deref for HtmlFormControlsCollection {
        type Target = HtmlCollection;
        #[inline]
        fn deref(&self) -> &HtmlCollection {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFormControlsCollection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFormControlsCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFormControlsCollection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFormControlsCollection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFormControlsCollection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFormControlsCollection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFormControlsCollection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFormControlsCollection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFormControlsCollection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFormControlsCollection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFormControlsCollection {
        #[inline]
        fn from(obj: JsValue) -> HtmlFormControlsCollection {
            HtmlFormControlsCollection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFormControlsCollection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFormControlsCollection> for HtmlFormControlsCollection {
        #[inline]
        fn as_ref(&self) -> &HtmlFormControlsCollection {
            self
        }
    }
    impl From<HtmlFormControlsCollection> for JsValue {
        #[inline]
        fn from(obj: HtmlFormControlsCollection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFormControlsCollection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFormControlsCollection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFormControlsCollection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFormControlsCollection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFormControlsCollection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFormControlsCollection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFormControlsCollection> for HtmlCollection {
    #[inline]
    fn from(obj: HtmlFormControlsCollection) -> HtmlCollection {
        use wasm_bindgen::JsCast;
        HtmlCollection::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlCollection> for HtmlFormControlsCollection {
    #[inline]
    fn as_ref(&self) -> &HtmlCollection {
        use wasm_bindgen::JsCast;
        HtmlCollection::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFormControlsCollection> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFormControlsCollection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFormControlsCollection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFormControlsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_HTMLFormControlsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormControlsCollection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlFormControlsCollection {
    #[cfg(all(feature = "HtmlFormControlsCollection",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlFormControlsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_HTMLFormControlsCollection(
                self_: <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_HTMLFormControlsCollection(
            self_: <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_HTMLFormControlsCollection(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormControlsCollection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_HTMLFormControlsCollection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormControlsCollection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlFormControlsCollection {
    #[cfg(all(feature = "HtmlFormControlsCollection",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    #[allow(clippy::all)]
    pub fn get(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlFormControlsCollection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_HTMLFormControlsCollection(
                self_: <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_HTMLFormControlsCollection(
            self_: <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormControlsCollection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_HTMLFormControlsCollection(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bcc502257b6e11c6: [u8; 382usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}<\x01\0\0\0\0\x03\0\0\x02\x1AHTMLFormControlsCollection,__widl_instanceof_HTMLFormControlsCollection\0\0\0\0.__widl_f_named_item_HTMLFormControlsCollection\0\0\0\x01\x1AHTMLFormControlsCollection\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0'__widl_f_get_HTMLFormControlsCollection\0\0\0\x01\x1AHTMLFormControlsCollection\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

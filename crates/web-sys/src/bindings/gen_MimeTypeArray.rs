use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MimeTypeArray` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MimeTypeArray {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MimeTypeArray: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MimeTypeArray {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(77u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(84u32);
            inform(121u32);
            inform(112u32);
            inform(101u32);
            inform(65u32);
            inform(114u32);
            inform(114u32);
            inform(97u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for MimeTypeArray {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for MimeTypeArray {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MimeTypeArray {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MimeTypeArray {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MimeTypeArray {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MimeTypeArray {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MimeTypeArray {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MimeTypeArray {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MimeTypeArray {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MimeTypeArray>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MimeTypeArray {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MimeTypeArray {
        #[inline]
        fn from(obj: JsValue) -> MimeTypeArray {
            MimeTypeArray { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MimeTypeArray {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MimeTypeArray> for MimeTypeArray {
        #[inline]
        fn as_ref(&self) -> &MimeTypeArray {
            self
        }
    }
    impl From<MimeTypeArray> for JsValue {
        #[inline]
        fn from(obj: MimeTypeArray) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MimeTypeArray {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MimeTypeArray(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MimeTypeArray(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MimeTypeArray(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MimeTypeArray { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MimeTypeArray) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MimeTypeArray> for ::js_sys::Object {
    #[inline]
    fn from(obj: MimeTypeArray) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MimeTypeArray {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_MimeTypeArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MimeTypeArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl MimeTypeArray {
    #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_MimeTypeArray(
                self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_MimeTypeArray(
            self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_MimeTypeArray(self_, index)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_MimeTypeArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MimeTypeArray as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl MimeTypeArray {
    #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_MimeTypeArray(
                self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_MimeTypeArray(
            self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_MimeTypeArray(self_, name)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_MimeTypeArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MimeTypeArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl MimeTypeArray {
    #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_MimeTypeArray(
                self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_MimeTypeArray(
            self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_MimeTypeArray(self_, index)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_MimeTypeArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MimeTypeArray as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<MimeType> as WasmDescribe>::describe();
}
impl MimeTypeArray {
    #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<MimeType> {
        #[cfg(all(feature = "MimeType", feature = "MimeTypeArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_MimeTypeArray(
                self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_MimeTypeArray(
            self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_MimeTypeArray(self_, name)
            };
            <Option<MimeType> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MimeTypeArray",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_MimeTypeArray() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MimeTypeArray as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl MimeTypeArray {
    #[cfg(all(feature = "MimeTypeArray",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)\n\n*This API requires the following crate features to be activated: `MimeTypeArray`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "MimeTypeArray",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_MimeTypeArray(
                self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_MimeTypeArray(
            self_: <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MimeTypeArray as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_MimeTypeArray(self_)
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
pub static __WASM_BINDGEN_GENERATED_710ccafdabe0b52f: [u8; 541usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDB\x01\0\0\0\0\x06\0\0\x02\rMimeTypeArray\x1F__widl_instanceof_MimeTypeArray\0\0\0\0\x1B__widl_f_item_MimeTypeArray\0\0\0\x01\rMimeTypeArray\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0!__widl_f_named_item_MimeTypeArray\0\0\0\x01\rMimeTypeArray\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0%__widl_f_get_with_index_MimeTypeArray\0\0\0\x01\rMimeTypeArray\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0$__widl_f_get_with_name_MimeTypeArray\0\0\0\x01\rMimeTypeArray\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0\x1D__widl_f_length_MimeTypeArray\0\0\0\x01\rMimeTypeArray\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

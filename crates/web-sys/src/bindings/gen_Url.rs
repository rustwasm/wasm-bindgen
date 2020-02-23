use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `URL` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Url {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Url: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Url {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(3u32);
            inform(85u32);
            inform(82u32);
            inform(76u32);
        }
    }
    impl core::ops::Deref for Url {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Url {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Url {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Url {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Url {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Url {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Url {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Url {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Url {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Url>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Url {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Url {
        #[inline]
        fn from(obj: JsValue) -> Url {
            Url { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Url {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Url> for Url {
        #[inline]
        fn as_ref(&self) -> &Url {
            self
        }
    }
    impl From<Url> for JsValue {
        #[inline]
        fn from(obj: Url) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Url {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_URL(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_URL(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_URL(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Url { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Url) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Url> for ::js_sys::Object {
    #[inline]
    fn from(obj: Url) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Url {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <Url as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `new URL(..)` constructor, creating a new instance of `URL`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn new(url: &str) -> Result<Url, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_URL(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Url as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_URL(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Url as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_new_URL(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Url as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_base_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Url as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `new URL(..)` constructor, creating a new instance of `URL`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/URL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn new_with_base(url: &str, base: &str) -> Result<Url, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_base_URL(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                base: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Url as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_base_URL(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            base: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Url as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            drop(base);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                let base = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(base);
                __widl_f_new_with_base_URL(url, base)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Url as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Blob", feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_object_url_with_blob_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Blob as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Blob", feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `createObjectURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)\n\n*This API requires the following crate features to be activated: `Blob`, `Url`*"]
    #[allow(clippy::all)]
    pub fn create_object_url_with_blob(blob: &Blob) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_object_url_with_blob_URL(
                blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_object_url_with_blob_URL(
            blob: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(blob);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let blob = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(blob);
                __widl_f_create_object_url_with_blob_URL(blob)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_object_url_with_source_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "MediaSource", feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `createObjectURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)\n\n*This API requires the following crate features to be activated: `MediaSource`, `Url`*"]
    #[allow(clippy::all)]
    pub fn create_object_url_with_source(
        source: &MediaSource,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource", feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_object_url_with_source_URL(
                source: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_object_url_with_source_URL(
            source: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let source = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_create_object_url_with_source_URL(source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_revoke_object_url_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `revokeObjectURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn revoke_object_url(url: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_revoke_object_url_URL(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_revoke_object_url_URL(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_revoke_object_url_URL(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_json_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_URL(self_, href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_protocol_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_protocol(&self, protocol: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_protocol_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_protocol_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(protocol);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let protocol = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(protocol);
                __widl_f_set_protocol_URL(self_, protocol)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_username_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `username` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn username(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_username_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_username_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_username_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_username_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `username` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_username(&self, username: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_username_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_username_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(username);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let username = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(username);
                __widl_f_set_username_URL(self_, username)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_password_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `password` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn password(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_password_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_password_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_password_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_password_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `password` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_password(&self, password: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_password_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_password_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(password);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let password = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(password);
                __widl_f_set_password_URL(self_, password)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_host_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `host` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_host(&self, host: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_host_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_host_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(host);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                __widl_f_set_host_URL(self_, host)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hostname_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn hostname(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hostname_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hostname_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hostname_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hostname_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_hostname(&self, hostname: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hostname_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hostname_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hostname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hostname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hostname);
                __widl_f_set_hostname_URL(self_, hostname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_port_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `port` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_port(&self, port: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_port_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_port_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let port = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_set_port_URL(self_, port)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pathname_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn pathname(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pathname_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pathname_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pathname_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pathname_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_pathname(&self, pathname: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pathname_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pathname_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pathname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pathname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pathname);
                __widl_f_set_pathname_URL(self_, pathname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `search` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn search(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_search_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `search` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_search(&self, search: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_search_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_search_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(search);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let search = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(search);
                __widl_f_set_search_URL(self_, search)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Url", feature = "UrlSearchParams",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_params_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <UrlSearchParams as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url", feature = "UrlSearchParams",))]
    #[allow(bad_style)]
    #[doc = "The `searchParams` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams)\n\n*This API requires the following crate features to be activated: `Url`, `UrlSearchParams`*"]
    #[allow(clippy::all)]
    pub fn search_params(&self) -> UrlSearchParams {
        #[cfg(all(feature = "Url", feature = "UrlSearchParams",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_params_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_params_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_params_URL(self_)
            };
            <UrlSearchParams as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hash_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Url as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `hash` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn hash(&self) -> String {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hash_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hash_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hash_URL(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Url",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hash_URL() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Url as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Url {
    #[cfg(all(feature = "Url",))]
    #[allow(bad_style)]
    #[doc = "The `hash` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)\n\n*This API requires the following crate features to be activated: `Url`*"]
    #[allow(clippy::all)]
    pub fn set_hash(&self, hash: &str) {
        #[cfg(all(feature = "Url",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hash_URL(
                self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hash_URL(
            self_: <&Url as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hash);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Url as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hash = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hash);
                __widl_f_set_hash_URL(self_, hash)
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
pub static __WASM_BINDGEN_GENERATED_752e96f2db18bf60: [u8; 1880usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x16\x07\0\0\0\0\x1D\0\0\x02\x03URL\x15__widl_instanceof_URL\0\0\0\0\x10__widl_f_new_URL\x01\0\0\x01\x03URL\0\x01\x01\x03url\x03new\0\0\0\x1A__widl_f_new_with_base_URL\x01\0\0\x01\x03URL\0\x01\x02\x03url\x04base\x03new\0\0\0(__widl_f_create_object_url_with_blob_URL\x01\0\0\x01\x03URL\x01\x01\0\x01\x01\x04blob\x0FcreateObjectURL\0\0\0*__widl_f_create_object_url_with_source_URL\x01\0\0\x01\x03URL\x01\x01\0\x01\x01\x06source\x0FcreateObjectURL\0\0\0\x1E__widl_f_revoke_object_url_URL\x01\0\0\x01\x03URL\x01\x01\0\x01\x01\x03url\x0FrevokeObjectURL\0\0\0\x14__widl_f_to_json_URL\0\0\0\x01\x03URL\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\0\x11__widl_f_href_URL\0\0\0\x01\x03URL\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\x15__widl_f_set_href_URL\0\0\0\x01\x03URL\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0\x13__widl_f_origin_URL\0\0\0\x01\x03URL\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0\x15__widl_f_protocol_URL\0\0\0\x01\x03URL\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0\x19__widl_f_set_protocol_URL\0\0\0\x01\x03URL\x01\0\x02\x08protocol\x01\x02\x05self_\x08protocol\x08protocol\0\0\0\x15__widl_f_username_URL\0\0\0\x01\x03URL\x01\0\x01\x08username\x01\x01\x05self_\x08username\0\0\0\x19__widl_f_set_username_URL\0\0\0\x01\x03URL\x01\0\x02\x08username\x01\x02\x05self_\x08username\x08username\0\0\0\x15__widl_f_password_URL\0\0\0\x01\x03URL\x01\0\x01\x08password\x01\x01\x05self_\x08password\0\0\0\x19__widl_f_set_password_URL\0\0\0\x01\x03URL\x01\0\x02\x08password\x01\x02\x05self_\x08password\x08password\0\0\0\x11__widl_f_host_URL\0\0\0\x01\x03URL\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0\x15__widl_f_set_host_URL\0\0\0\x01\x03URL\x01\0\x02\x04host\x01\x02\x05self_\x04host\x04host\0\0\0\x15__widl_f_hostname_URL\0\0\0\x01\x03URL\x01\0\x01\x08hostname\x01\x01\x05self_\x08hostname\0\0\0\x19__widl_f_set_hostname_URL\0\0\0\x01\x03URL\x01\0\x02\x08hostname\x01\x02\x05self_\x08hostname\x08hostname\0\0\0\x11__widl_f_port_URL\0\0\0\x01\x03URL\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0\x15__widl_f_set_port_URL\0\0\0\x01\x03URL\x01\0\x02\x04port\x01\x02\x05self_\x04port\x04port\0\0\0\x15__widl_f_pathname_URL\0\0\0\x01\x03URL\x01\0\x01\x08pathname\x01\x01\x05self_\x08pathname\0\0\0\x19__widl_f_set_pathname_URL\0\0\0\x01\x03URL\x01\0\x02\x08pathname\x01\x02\x05self_\x08pathname\x08pathname\0\0\0\x13__widl_f_search_URL\0\0\0\x01\x03URL\x01\0\x01\x06search\x01\x01\x05self_\x06search\0\0\0\x17__widl_f_set_search_URL\0\0\0\x01\x03URL\x01\0\x02\x06search\x01\x02\x05self_\x06search\x06search\0\0\0\x1A__widl_f_search_params_URL\0\0\0\x01\x03URL\x01\0\x01\x0CsearchParams\x01\x01\x05self_\x0CsearchParams\0\0\0\x11__widl_f_hash_URL\0\0\0\x01\x03URL\x01\0\x01\x04hash\x01\x01\x05self_\x04hash\0\0\0\x15__widl_f_set_hash_URL\0\0\0\x01\x03URL\x01\0\x02\x04hash\x01\x02\x05self_\x04hash\x04hash\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

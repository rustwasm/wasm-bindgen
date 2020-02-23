use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `EXT_disjoint_timer_query` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ExtDisjointTimerQuery {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ExtDisjointTimerQuery: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ExtDisjointTimerQuery {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(69u32);
            inform(88u32);
            inform(84u32);
            inform(95u32);
            inform(100u32);
            inform(105u32);
            inform(115u32);
            inform(106u32);
            inform(111u32);
            inform(105u32);
            inform(110u32);
            inform(116u32);
            inform(95u32);
            inform(116u32);
            inform(105u32);
            inform(109u32);
            inform(101u32);
            inform(114u32);
            inform(95u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(114u32);
            inform(121u32);
        }
    }
    impl core::ops::Deref for ExtDisjointTimerQuery {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for ExtDisjointTimerQuery {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ExtDisjointTimerQuery {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ExtDisjointTimerQuery {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ExtDisjointTimerQuery {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ExtDisjointTimerQuery {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ExtDisjointTimerQuery {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ExtDisjointTimerQuery {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ExtDisjointTimerQuery {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ExtDisjointTimerQuery>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ExtDisjointTimerQuery {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ExtDisjointTimerQuery {
        #[inline]
        fn from(obj: JsValue) -> ExtDisjointTimerQuery {
            ExtDisjointTimerQuery { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ExtDisjointTimerQuery {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ExtDisjointTimerQuery> for ExtDisjointTimerQuery {
        #[inline]
        fn as_ref(&self) -> &ExtDisjointTimerQuery {
            self
        }
    }
    impl From<ExtDisjointTimerQuery> for JsValue {
        #[inline]
        fn from(obj: ExtDisjointTimerQuery) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ExtDisjointTimerQuery {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_EXT_disjoint_timer_query(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_EXT_disjoint_timer_query(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_EXT_disjoint_timer_query(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ExtDisjointTimerQuery { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ExtDisjointTimerQuery) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ExtDisjointTimerQuery> for ::js_sys::Object {
    #[inline]
    fn from(obj: ExtDisjointTimerQuery) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ExtDisjointTimerQuery {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&WebGlQuery as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `beginQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/beginQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn begin_query_ext(&self, target: u32, query: &WebGlQuery) {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            drop(query);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let query = <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(query);
                __widl_f_begin_query_ext_EXT_disjoint_timer_query(self_, target, query)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <Option<WebGlQuery> as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `createQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/createQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn create_query_ext(&self) -> Option<WebGlQuery> {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlQuery> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlQuery> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_query_ext_EXT_disjoint_timer_query(self_)
            };
            <Option<WebGlQuery> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <Option<&WebGlQuery> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `deleteQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/deleteQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn delete_query_ext(&self, query: Option<&WebGlQuery>) {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                query: <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            query: <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(query);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let query =
                    <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(query);
                __widl_f_delete_query_ext_EXT_disjoint_timer_query(self_, query)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery",))]
    #[allow(bad_style)]
    #[doc = "The `endQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/endQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    #[allow(clippy::all)]
    pub fn end_query_ext(&self, target: u32) {
        #[cfg(all(feature = "ExtDisjointTimerQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_end_query_ext_EXT_disjoint_timer_query(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery",))]
    #[allow(bad_style)]
    #[doc = "The `getQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    #[allow(clippy::all)]
    pub fn get_query_ext(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "ExtDisjointTimerQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(target);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_query_ext_EXT_disjoint_timer_query(self_, target, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_query_object_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <&WebGlQuery as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `getQueryObjectEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryObjectEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn get_query_object_ext(&self, query: &WebGlQuery, pname: u32) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_query_object_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_query_object_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pname: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(query);
            drop(pname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let query = <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(query);
                let pname = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pname);
                __widl_f_get_query_object_ext_EXT_disjoint_timer_query(self_, query, pname)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_query_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <Option<&WebGlQuery> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `isQueryEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/isQueryEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn is_query_ext(&self, query: Option<&WebGlQuery>) -> bool {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_query_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                query: <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_query_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            query: <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(query);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let query =
                    <Option<&WebGlQuery> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(query);
                __widl_f_is_query_ext_EXT_disjoint_timer_query(self_, query)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_query_counter_ext_EXT_disjoint_timer_query() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&ExtDisjointTimerQuery as WasmDescribe>::describe();
    <&WebGlQuery as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ExtDisjointTimerQuery {
    #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
    #[allow(bad_style)]
    #[doc = "The `queryCounterEXT()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/queryCounterEXT)\n\n*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    #[allow(clippy::all)]
    pub fn query_counter_ext(&self, query: &WebGlQuery, target: u32) {
        #[cfg(all(feature = "ExtDisjointTimerQuery", feature = "WebGlQuery",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_query_counter_ext_EXT_disjoint_timer_query(
                self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_query_counter_ext_EXT_disjoint_timer_query(
            self_: <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            query: <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(query);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ExtDisjointTimerQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let query = <&WebGlQuery as wasm_bindgen::convert::IntoWasmAbi>::into_abi(query);
                let target = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_query_counter_ext_EXT_disjoint_timer_query(self_, query, target)
            };
            ()
        }
    }
}
impl ExtDisjointTimerQuery {
    pub const QUERY_COUNTER_BITS_EXT: u32 = 34916u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const CURRENT_QUERY_EXT: u32 = 34917u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const QUERY_RESULT_EXT: u32 = 34918u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const QUERY_RESULT_AVAILABLE_EXT: u32 = 34919u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const TIME_ELAPSED_EXT: u32 = 35007u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const TIMESTAMP_EXT: u32 = 36392u64 as u32;
}
impl ExtDisjointTimerQuery {
    pub const GPU_DISJOINT_EXT: u32 = 36795u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a3a4fc01e9ada3c5: [u8; 1106usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x10\x04\0\0\0\0\t\0\0\x02\x18EXT_disjoint_timer_query*__widl_instanceof_EXT_disjoint_timer_query\0\0\0\01__widl_f_begin_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x03\x05self_\x06target\x05query\rbeginQueryEXT\0\0\02__widl_f_create_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x01\x05self_\x0EcreateQueryEXT\0\0\02__widl_f_delete_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x02\x05self_\x05query\x0EdeleteQueryEXT\0\0\0/__widl_f_end_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x02\x05self_\x06target\x0BendQueryEXT\0\0\0/__widl_f_get_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x03\x05self_\x06target\x05pname\x0BgetQueryEXT\0\0\06__widl_f_get_query_object_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x03\x05self_\x05query\x05pname\x11getQueryObjectEXT\0\0\0.__widl_f_is_query_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x02\x05self_\x05query\nisQueryEXT\0\0\03__widl_f_query_counter_ext_EXT_disjoint_timer_query\0\0\0\x01\x18EXT_disjoint_timer_query\x01\0\0\x01\x03\x05self_\x05query\x06target\x0FqueryCounterEXT\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

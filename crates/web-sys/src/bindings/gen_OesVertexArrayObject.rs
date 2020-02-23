use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OES_vertex_array_object` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OesVertexArrayObject {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OesVertexArrayObject: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OesVertexArrayObject {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(23u32);
            inform(79u32);
            inform(69u32);
            inform(83u32);
            inform(95u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(95u32);
            inform(97u32);
            inform(114u32);
            inform(114u32);
            inform(97u32);
            inform(121u32);
            inform(95u32);
            inform(111u32);
            inform(98u32);
            inform(106u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for OesVertexArrayObject {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for OesVertexArrayObject {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OesVertexArrayObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OesVertexArrayObject {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OesVertexArrayObject {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OesVertexArrayObject {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OesVertexArrayObject {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OesVertexArrayObject {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OesVertexArrayObject {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OesVertexArrayObject>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OesVertexArrayObject {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OesVertexArrayObject {
        #[inline]
        fn from(obj: JsValue) -> OesVertexArrayObject {
            OesVertexArrayObject { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OesVertexArrayObject {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OesVertexArrayObject> for OesVertexArrayObject {
        #[inline]
        fn as_ref(&self) -> &OesVertexArrayObject {
            self
        }
    }
    impl From<OesVertexArrayObject> for JsValue {
        #[inline]
        fn from(obj: OesVertexArrayObject) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OesVertexArrayObject {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OES_vertex_array_object(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OES_vertex_array_object(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OES_vertex_array_object(idx) != 0
            }
        }
        #[inline]
        fn is_type_of(val: &JsValue) -> bool {
            let is_type_of: fn(&JsValue) -> bool = |_| false;
            is_type_of(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OesVertexArrayObject { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OesVertexArrayObject) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OesVertexArrayObject> for ::js_sys::Object {
    #[inline]
    fn from(obj: OesVertexArrayObject) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OesVertexArrayObject {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bind_vertex_array_oes_OES_vertex_array_object() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OesVertexArrayObject as WasmDescribe>::describe();
    <Option<&WebGlVertexArrayObject> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OesVertexArrayObject {
    #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
    #[allow(bad_style)]
    #[doc = "The `bindVertexArrayOES()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    #[allow(clippy::all)]
    pub fn bind_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) {
        #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bind_vertex_array_oes_OES_vertex_array_object(
                self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bind_vertex_array_oes_OES_vertex_array_object(
            self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(array_object);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array_object = < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( array_object ) ;
                __widl_f_bind_vertex_array_oes_OES_vertex_array_object(self_, array_object)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_vertex_array_oes_OES_vertex_array_object() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OesVertexArrayObject as WasmDescribe>::describe();
    <Option<WebGlVertexArrayObject> as WasmDescribe>::describe();
}
impl OesVertexArrayObject {
    #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
    #[allow(bad_style)]
    #[doc = "The `createVertexArrayOES()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    #[allow(clippy::all)]
    pub fn create_vertex_array_oes(&self) -> Option<WebGlVertexArrayObject> {
        #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_vertex_array_oes_OES_vertex_array_object(
                self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<WebGlVertexArrayObject> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_vertex_array_oes_OES_vertex_array_object(
            self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<WebGlVertexArrayObject> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_vertex_array_oes_OES_vertex_array_object(self_)
            };
            <Option<WebGlVertexArrayObject> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_vertex_array_oes_OES_vertex_array_object() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OesVertexArrayObject as WasmDescribe>::describe();
    <Option<&WebGlVertexArrayObject> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OesVertexArrayObject {
    #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
    #[allow(bad_style)]
    #[doc = "The `deleteVertexArrayOES()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    #[allow(clippy::all)]
    pub fn delete_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) {
        #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_vertex_array_oes_OES_vertex_array_object(
                self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_vertex_array_oes_OES_vertex_array_object(
            self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(array_object);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array_object = < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( array_object ) ;
                __widl_f_delete_vertex_array_oes_OES_vertex_array_object(self_, array_object)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_vertex_array_oes_OES_vertex_array_object() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OesVertexArrayObject as WasmDescribe>::describe();
    <Option<&WebGlVertexArrayObject> as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl OesVertexArrayObject {
    #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
    #[allow(bad_style)]
    #[doc = "The `isVertexArrayOES()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    #[allow(clippy::all)]
    pub fn is_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) -> bool {
        #[cfg(all(feature = "OesVertexArrayObject", feature = "WebGlVertexArrayObject",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_vertex_array_oes_OES_vertex_array_object(
                self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_vertex_array_oes_OES_vertex_array_object(
            self_: <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            array_object : < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(array_object);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OesVertexArrayObject as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let array_object = < Option < & WebGlVertexArrayObject > as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( array_object ) ;
                __widl_f_is_vertex_array_oes_OES_vertex_array_object(self_, array_object)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
impl OesVertexArrayObject {
    pub const VERTEX_ARRAY_BINDING_OES: u32 = 34229u64 as u32;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0a2912ca1a61a75c: [u8; 682usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}h\x02\0\0\0\0\x05\0\0\x02\x17OES_vertex_array_object)__widl_instanceof_OES_vertex_array_object\0\0\0\06__widl_f_bind_vertex_array_oes_OES_vertex_array_object\0\0\0\x01\x17OES_vertex_array_object\x01\0\0\x01\x02\x05self_\x0Carray_object\x12bindVertexArrayOES\0\0\08__widl_f_create_vertex_array_oes_OES_vertex_array_object\0\0\0\x01\x17OES_vertex_array_object\x01\0\0\x01\x01\x05self_\x14createVertexArrayOES\0\0\08__widl_f_delete_vertex_array_oes_OES_vertex_array_object\0\0\0\x01\x17OES_vertex_array_object\x01\0\0\x01\x02\x05self_\x0Carray_object\x14deleteVertexArrayOES\0\0\04__widl_f_is_vertex_array_oes_OES_vertex_array_object\0\0\0\x01\x17OES_vertex_array_object\x01\0\0\x01\x02\x05self_\x0Carray_object\x10isVertexArrayOES\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

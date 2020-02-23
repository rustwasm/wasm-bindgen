use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WorkerLocation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WorkerLocation {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WorkerLocation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WorkerLocation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
            inform(76u32);
            inform(111u32);
            inform(99u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for WorkerLocation {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for WorkerLocation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WorkerLocation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WorkerLocation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WorkerLocation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WorkerLocation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WorkerLocation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WorkerLocation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WorkerLocation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WorkerLocation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WorkerLocation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WorkerLocation {
        #[inline]
        fn from(obj: JsValue) -> WorkerLocation {
            WorkerLocation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WorkerLocation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WorkerLocation> for WorkerLocation {
        #[inline]
        fn as_ref(&self) -> &WorkerLocation {
            self
        }
    }
    impl From<WorkerLocation> for JsValue {
        #[inline]
        fn from(obj: WorkerLocation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WorkerLocation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WorkerLocation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WorkerLocation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WorkerLocation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WorkerLocation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WorkerLocation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WorkerLocation> for ::js_sys::Object {
    #[inline]
    fn from(obj: WorkerLocation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WorkerLocation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/href)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/origin)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/protocol)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/host)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hostname_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hostname)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn hostname(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hostname_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hostname_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hostname_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/port)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pathname_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/pathname)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn pathname(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pathname_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pathname_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pathname_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `search` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/search)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn search(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WorkerLocation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hash_WorkerLocation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WorkerLocation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl WorkerLocation {
    #[cfg(all(feature = "WorkerLocation",))]
    #[allow(bad_style)]
    #[doc = "The `hash` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hash)\n\n*This API requires the following crate features to be activated: `WorkerLocation`*"]
    #[allow(clippy::all)]
    pub fn hash(&self) -> String {
        #[cfg(all(feature = "WorkerLocation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hash_WorkerLocation(
                self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hash_WorkerLocation(
            self_: <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WorkerLocation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hash_WorkerLocation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f8ac0da952aed965: [u8; 853usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x13\x03\0\0\0\0\n\0\0\x02\x0EWorkerLocation __widl_instanceof_WorkerLocation\0\0\0\0\x1C__widl_f_href_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\x1E__widl_f_origin_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0 __widl_f_protocol_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0\x1C__widl_f_host_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0 __widl_f_hostname_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x08hostname\x01\x01\x05self_\x08hostname\0\0\0\x1C__widl_f_port_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0 __widl_f_pathname_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x08pathname\x01\x01\x05self_\x08pathname\0\0\0\x1E__widl_f_search_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x06search\x01\x01\x05self_\x06search\0\0\0\x1C__widl_f_hash_WorkerLocation\0\0\0\x01\x0EWorkerLocation\x01\0\x01\x04hash\x01\x01\x05self_\x04hash\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

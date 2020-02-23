use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceResourceTiming` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceResourceTiming {
    obj: PerformanceEntry,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceResourceTiming: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceResourceTiming {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(80u32);
            inform(101u32);
            inform(114u32);
            inform(102u32);
            inform(111u32);
            inform(114u32);
            inform(109u32);
            inform(97u32);
            inform(110u32);
            inform(99u32);
            inform(101u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
        }
    }
    impl core::ops::Deref for PerformanceResourceTiming {
        type Target = PerformanceEntry;
        #[inline]
        fn deref(&self) -> &PerformanceEntry {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceResourceTiming {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceResourceTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceResourceTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceResourceTiming {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceResourceTiming {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceResourceTiming {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceResourceTiming {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceResourceTiming {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceResourceTiming>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceResourceTiming {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceResourceTiming {
        #[inline]
        fn from(obj: JsValue) -> PerformanceResourceTiming {
            PerformanceResourceTiming { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceResourceTiming {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceResourceTiming> for PerformanceResourceTiming {
        #[inline]
        fn as_ref(&self) -> &PerformanceResourceTiming {
            self
        }
    }
    impl From<PerformanceResourceTiming> for JsValue {
        #[inline]
        fn from(obj: PerformanceResourceTiming) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceResourceTiming {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceResourceTiming(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceResourceTiming(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceResourceTiming(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceResourceTiming { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceResourceTiming) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceResourceTiming> for PerformanceEntry {
    #[inline]
    fn from(obj: PerformanceResourceTiming) -> PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PerformanceEntry> for PerformanceResourceTiming {
    #[inline]
    fn as_ref(&self) -> &PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PerformanceResourceTiming> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceResourceTiming) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceResourceTiming {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_to_json_PerformanceResourceTiming(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_initiator_type_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `initiatorType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/initiatorType)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn initiator_type(&self) -> String {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_initiator_type_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_initiator_type_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_initiator_type_PerformanceResourceTiming(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_next_hop_protocol_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `nextHopProtocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/nextHopProtocol)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn next_hop_protocol(&self) -> String {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_next_hop_protocol_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_next_hop_protocol_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_next_hop_protocol_PerformanceResourceTiming(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_worker_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `workerStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/workerStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn worker_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_worker_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_worker_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_worker_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `redirectStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn redirect_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_redirect_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_end_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `redirectEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn redirect_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_end_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_end_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_redirect_end_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fetch_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `fetchStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/fetchStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn fetch_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fetch_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fetch_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_fetch_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_domain_lookup_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domainLookupStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn domain_lookup_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_domain_lookup_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_domain_lookup_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_domain_lookup_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_domain_lookup_end_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domainLookupEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn domain_lookup_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_domain_lookup_end_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_domain_lookup_end_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_domain_lookup_end_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `connectStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn connect_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_connect_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_end_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `connectEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn connect_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_end_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_end_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_connect_end_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_secure_connection_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `secureConnectionStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/secureConnectionStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn secure_connection_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_secure_connection_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_secure_connection_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_secure_connection_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `requestStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/requestStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn request_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_request_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_start_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `responseStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStart)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn response_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_start_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_start_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_response_start_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_response_end_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `responseEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseEnd)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn response_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_response_end_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_response_end_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_response_end_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transfer_size_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `transferSize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/transferSize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn transfer_size(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transfer_size_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transfer_size_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_transfer_size_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encoded_body_size_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `encodedBodySize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/encodedBodySize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn encoded_body_size(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encoded_body_size_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encoded_body_size_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_encoded_body_size_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decoded_body_size_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `decodedBodySize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/decodedBodySize)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn decoded_body_size(&self) -> f64 {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decoded_body_size_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decoded_body_size_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_decoded_body_size_PerformanceResourceTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceResourceTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_server_timing_PerformanceResourceTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceResourceTiming as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl PerformanceResourceTiming {
    #[cfg(all(feature = "PerformanceResourceTiming",))]
    #[allow(bad_style)]
    #[doc = "The `serverTiming` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/serverTiming)\n\n*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
    #[allow(clippy::all)]
    pub fn server_timing(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "PerformanceResourceTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_server_timing_PerformanceResourceTiming(
                self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_server_timing_PerformanceResourceTiming(
            self_: <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceResourceTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_server_timing_PerformanceResourceTiming(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b97a0215595503ce: [u8; 2487usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}u\t\0\0\0\0\x14\0\0\x02\x19PerformanceResourceTiming+__widl_instanceof_PerformanceResourceTiming\0\0\0\0*__widl_f_to_json_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\01__widl_f_initiator_type_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\rinitiatorType\x01\x01\x05self_\rinitiatorType\0\0\04__widl_f_next_hop_protocol_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0FnextHopProtocol\x01\x01\x05self_\x0FnextHopProtocol\0\0\0/__widl_f_worker_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0BworkerStart\x01\x01\x05self_\x0BworkerStart\0\0\01__widl_f_redirect_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\rredirectStart\x01\x01\x05self_\rredirectStart\0\0\0/__widl_f_redirect_end_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0BredirectEnd\x01\x01\x05self_\x0BredirectEnd\0\0\0.__widl_f_fetch_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\nfetchStart\x01\x01\x05self_\nfetchStart\0\0\06__widl_f_domain_lookup_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x11domainLookupStart\x01\x01\x05self_\x11domainLookupStart\0\0\04__widl_f_domain_lookup_end_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0FdomainLookupEnd\x01\x01\x05self_\x0FdomainLookupEnd\0\0\00__widl_f_connect_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0CconnectStart\x01\x01\x05self_\x0CconnectStart\0\0\0.__widl_f_connect_end_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\nconnectEnd\x01\x01\x05self_\nconnectEnd\0\0\0:__widl_f_secure_connection_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x15secureConnectionStart\x01\x01\x05self_\x15secureConnectionStart\0\0\00__widl_f_request_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0CrequestStart\x01\x01\x05self_\x0CrequestStart\0\0\01__widl_f_response_start_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\rresponseStart\x01\x01\x05self_\rresponseStart\0\0\0/__widl_f_response_end_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0BresponseEnd\x01\x01\x05self_\x0BresponseEnd\0\0\00__widl_f_transfer_size_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0CtransferSize\x01\x01\x05self_\x0CtransferSize\0\0\04__widl_f_encoded_body_size_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0FencodedBodySize\x01\x01\x05self_\x0FencodedBodySize\0\0\04__widl_f_decoded_body_size_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0FdecodedBodySize\x01\x01\x05self_\x0FdecodedBodySize\0\0\00__widl_f_server_timing_PerformanceResourceTiming\0\0\0\x01\x19PerformanceResourceTiming\x01\0\x01\x0CserverTiming\x01\x01\x05self_\x0CserverTiming\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

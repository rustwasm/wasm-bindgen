use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PerformanceNavigationTiming` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PerformanceNavigationTiming {
    obj: PerformanceResourceTiming,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PerformanceNavigationTiming: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PerformanceNavigationTiming {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
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
            inform(78u32);
            inform(97u32);
            inform(118u32);
            inform(105u32);
            inform(103u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
        }
    }
    impl core::ops::Deref for PerformanceNavigationTiming {
        type Target = PerformanceResourceTiming;
        #[inline]
        fn deref(&self) -> &PerformanceResourceTiming {
            &self.obj
        }
    }
    impl IntoWasmAbi for PerformanceNavigationTiming {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PerformanceNavigationTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PerformanceNavigationTiming {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PerformanceNavigationTiming {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PerformanceNavigationTiming {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PerformanceNavigationTiming {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PerformanceNavigationTiming {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PerformanceNavigationTiming {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PerformanceNavigationTiming>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PerformanceNavigationTiming {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PerformanceNavigationTiming {
        #[inline]
        fn from(obj: JsValue) -> PerformanceNavigationTiming {
            PerformanceNavigationTiming { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PerformanceNavigationTiming {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PerformanceNavigationTiming> for PerformanceNavigationTiming {
        #[inline]
        fn as_ref(&self) -> &PerformanceNavigationTiming {
            self
        }
    }
    impl From<PerformanceNavigationTiming> for JsValue {
        #[inline]
        fn from(obj: PerformanceNavigationTiming) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PerformanceNavigationTiming {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PerformanceNavigationTiming(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PerformanceNavigationTiming(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PerformanceNavigationTiming(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PerformanceNavigationTiming { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PerformanceNavigationTiming) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PerformanceNavigationTiming> for PerformanceResourceTiming {
    #[inline]
    fn from(obj: PerformanceNavigationTiming) -> PerformanceResourceTiming {
        use wasm_bindgen::JsCast;
        PerformanceResourceTiming::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PerformanceResourceTiming> for PerformanceNavigationTiming {
    #[inline]
    fn as_ref(&self) -> &PerformanceResourceTiming {
        use wasm_bindgen::JsCast;
        PerformanceResourceTiming::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PerformanceNavigationTiming> for PerformanceEntry {
    #[inline]
    fn from(obj: PerformanceNavigationTiming) -> PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<PerformanceEntry> for PerformanceNavigationTiming {
    #[inline]
    fn as_ref(&self) -> &PerformanceEntry {
        use wasm_bindgen::JsCast;
        PerformanceEntry::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PerformanceNavigationTiming> for ::js_sys::Object {
    #[inline]
    fn from(obj: PerformanceNavigationTiming) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PerformanceNavigationTiming {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_json_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `toJSON()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/toJSON)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn to_json(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_json_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_json_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_to_json_PerformanceNavigationTiming(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unload_event_start_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `unloadEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn unload_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unload_event_start_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unload_event_start_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_unload_event_start_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_unload_event_end_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `unloadEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn unload_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_unload_event_end_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_unload_event_end_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_unload_event_end_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_interactive_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domInteractive` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domInteractive)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_interactive(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_interactive_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_interactive_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dom_interactive_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_content_loaded_event_start_PerformanceNavigationTiming(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domContentLoadedEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_content_loaded_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_content_loaded_event_start_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_content_loaded_event_start_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dom_content_loaded_event_start_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_content_loaded_event_end_PerformanceNavigationTiming(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domContentLoadedEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_content_loaded_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_content_loaded_event_end_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_content_loaded_event_end_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dom_content_loaded_event_end_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dom_complete_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `domComplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domComplete)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn dom_complete(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dom_complete_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dom_complete_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_dom_complete_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_event_start_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `loadEventStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventStart)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn load_event_start(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_event_start_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_event_start_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_load_event_start_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_event_end_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `loadEventEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventEnd)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn load_event_end(&self) -> f64 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_event_end_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_event_end_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_load_event_end_PerformanceNavigationTiming(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "NavigationType", feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <NavigationType as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "NavigationType", feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/type)\n\n*This API requires the following crate features to be activated: `NavigationType`, `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> NavigationType {
        #[cfg(all(feature = "NavigationType", feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NavigationType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NavigationType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_type_PerformanceNavigationTiming(self_)
            };
            <NavigationType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PerformanceNavigationTiming",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_redirect_count_PerformanceNavigationTiming() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PerformanceNavigationTiming as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl PerformanceNavigationTiming {
    #[cfg(all(feature = "PerformanceNavigationTiming",))]
    #[allow(bad_style)]
    #[doc = "The `redirectCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/redirectCount)\n\n*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
    #[allow(clippy::all)]
    pub fn redirect_count(&self) -> u16 {
        #[cfg(all(feature = "PerformanceNavigationTiming",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_redirect_count_PerformanceNavigationTiming(
                self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_redirect_count_PerformanceNavigationTiming(
            self_: <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PerformanceNavigationTiming as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_redirect_count_PerformanceNavigationTiming(self_)
            };
            <u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4749cfbaf470a358: [u8; 1604usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x02\x06\0\0\0\0\x0C\0\0\x02\x1BPerformanceNavigationTiming-__widl_instanceof_PerformanceNavigationTiming\0\0\0\0,__widl_f_to_json_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\0\x01\x01\x05self_\x06toJSON\0\0\07__widl_f_unload_event_start_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x10unloadEventStart\x01\x01\x05self_\x10unloadEventStart\0\0\05__widl_f_unload_event_end_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x0EunloadEventEnd\x01\x01\x05self_\x0EunloadEventEnd\0\0\04__widl_f_dom_interactive_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x0EdomInteractive\x01\x01\x05self_\x0EdomInteractive\0\0\0C__widl_f_dom_content_loaded_event_start_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x1AdomContentLoadedEventStart\x01\x01\x05self_\x1AdomContentLoadedEventStart\0\0\0A__widl_f_dom_content_loaded_event_end_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x18domContentLoadedEventEnd\x01\x01\x05self_\x18domContentLoadedEventEnd\0\0\01__widl_f_dom_complete_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x0BdomComplete\x01\x01\x05self_\x0BdomComplete\0\0\05__widl_f_load_event_start_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x0EloadEventStart\x01\x01\x05self_\x0EloadEventStart\0\0\03__widl_f_load_event_end_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x0CloadEventEnd\x01\x01\x05self_\x0CloadEventEnd\0\0\0)__widl_f_type_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\03__widl_f_redirect_count_PerformanceNavigationTiming\0\0\0\x01\x1BPerformanceNavigationTiming\x01\0\x01\rredirectCount\x01\x01\x05self_\rredirectCount\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

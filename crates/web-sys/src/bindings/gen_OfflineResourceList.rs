use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OfflineResourceList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OfflineResourceList {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OfflineResourceList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OfflineResourceList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(79u32);
            inform(102u32);
            inform(102u32);
            inform(108u32);
            inform(105u32);
            inform(110u32);
            inform(101u32);
            inform(82u32);
            inform(101u32);
            inform(115u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for OfflineResourceList {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for OfflineResourceList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OfflineResourceList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OfflineResourceList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OfflineResourceList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OfflineResourceList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OfflineResourceList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OfflineResourceList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OfflineResourceList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OfflineResourceList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OfflineResourceList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OfflineResourceList {
        #[inline]
        fn from(obj: JsValue) -> OfflineResourceList {
            OfflineResourceList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OfflineResourceList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OfflineResourceList> for OfflineResourceList {
        #[inline]
        fn as_ref(&self) -> &OfflineResourceList {
            self
        }
    }
    impl From<OfflineResourceList> for JsValue {
        #[inline]
        fn from(obj: OfflineResourceList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OfflineResourceList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OfflineResourceList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OfflineResourceList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OfflineResourceList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OfflineResourceList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OfflineResourceList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OfflineResourceList> for EventTarget {
    #[inline]
    fn from(obj: OfflineResourceList) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for OfflineResourceList {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OfflineResourceList> for ::js_sys::Object {
    #[inline]
    fn from(obj: OfflineResourceList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OfflineResourceList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_swap_cache_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `swapCache()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/swapCache)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn swap_cache(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_swap_cache_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_swap_cache_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_swap_cache_OfflineResourceList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/update)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn update(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_update_OfflineResourceList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_status_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <u16 as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `status` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/status)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn status(&self) -> Result<u16, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_status_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u16 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_status_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_status_OfflineResourceList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<u16 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchecking_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onchecking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onchecking(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchecking_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchecking_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchecking_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchecking_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onchecking` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onchecking(&self, onchecking: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchecking_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchecking : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchecking_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchecking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchecking);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchecking =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchecking,
                    );
                __widl_f_set_onchecking_OfflineResourceList(self_, onchecking)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_OfflineResourceList(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onnoupdate_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onnoupdate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onnoupdate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onnoupdate_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onnoupdate_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onnoupdate_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onnoupdate_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onnoupdate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onnoupdate(&self, onnoupdate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onnoupdate_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onnoupdate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onnoupdate_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onnoupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onnoupdate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onnoupdate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onnoupdate,
                    );
                __widl_f_set_onnoupdate_OfflineResourceList(self_, onnoupdate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondownloading_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `ondownloading` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn ondownloading(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondownloading_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondownloading_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondownloading_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondownloading_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `ondownloading` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_ondownloading(&self, ondownloading: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondownloading_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondownloading : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondownloading_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondownloading: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondownloading);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondownloading =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondownloading,
                    );
                __widl_f_set_ondownloading_OfflineResourceList(self_, ondownloading)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_OfflineResourceList(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupdateready_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onupdateready` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onupdateready(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupdateready_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupdateready_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onupdateready_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupdateready_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onupdateready` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onupdateready(&self, onupdateready: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupdateready_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupdateready : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupdateready_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupdateready: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onupdateready);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onupdateready =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupdateready,
                    );
                __widl_f_set_onupdateready_OfflineResourceList(self_, onupdateready)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncached_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `oncached` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn oncached(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncached_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncached_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncached_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncached_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `oncached` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_oncached(&self, oncached: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncached_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncached: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncached_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncached: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncached);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncached =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncached,
                    );
                __widl_f_set_oncached_OfflineResourceList(self_, oncached)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onobsolete_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onobsolete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn onobsolete(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onobsolete_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onobsolete_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onobsolete_OfflineResourceList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OfflineResourceList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onobsolete_OfflineResourceList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OfflineResourceList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OfflineResourceList {
    #[cfg(all(feature = "OfflineResourceList",))]
    #[allow(bad_style)]
    #[doc = "The `onobsolete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)\n\n*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    #[allow(clippy::all)]
    pub fn set_onobsolete(&self, onobsolete: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OfflineResourceList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onobsolete_OfflineResourceList(
                self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onobsolete : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onobsolete_OfflineResourceList(
            self_: <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onobsolete: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onobsolete);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OfflineResourceList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onobsolete =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onobsolete,
                    );
                __widl_f_set_onobsolete_OfflineResourceList(self_, onobsolete)
            };
            ()
        }
    }
}
impl OfflineResourceList {
    pub const UNCACHED: u16 = 0i64 as u16;
}
impl OfflineResourceList {
    pub const IDLE: u16 = 1u64 as u16;
}
impl OfflineResourceList {
    pub const CHECKING: u16 = 2u64 as u16;
}
impl OfflineResourceList {
    pub const DOWNLOADING: u16 = 3u64 as u16;
}
impl OfflineResourceList {
    pub const UPDATEREADY: u16 = 4u64 as u16;
}
impl OfflineResourceList {
    pub const OBSOLETE: u16 = 5u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ec27f144fc04ca8b: [u8; 2151usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}%\x08\0\0\0\0\x14\0\0\x02\x13OfflineResourceList%__widl_instanceof_OfflineResourceList\0\0\0\0'__widl_f_swap_cache_OfflineResourceList\x01\0\0\x01\x13OfflineResourceList\x01\0\0\x01\x01\x05self_\tswapCache\0\0\0#__widl_f_update_OfflineResourceList\x01\0\0\x01\x13OfflineResourceList\x01\0\0\x01\x01\x05self_\x06update\0\0\0#__widl_f_status_OfflineResourceList\x01\0\0\x01\x13OfflineResourceList\x01\0\x01\x06status\x01\x01\x05self_\x06status\0\0\0'__widl_f_onchecking_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\nonchecking\x01\x01\x05self_\nonchecking\0\0\0+__widl_f_set_onchecking_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\nonchecking\x01\x02\x05self_\nonchecking\nonchecking\0\0\0$__widl_f_onerror_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0(__widl_f_set_onerror_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0'__widl_f_onnoupdate_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\nonnoupdate\x01\x01\x05self_\nonnoupdate\0\0\0+__widl_f_set_onnoupdate_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\nonnoupdate\x01\x02\x05self_\nonnoupdate\nonnoupdate\0\0\0*__widl_f_ondownloading_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\rondownloading\x01\x01\x05self_\rondownloading\0\0\0.__widl_f_set_ondownloading_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\rondownloading\x01\x02\x05self_\rondownloading\rondownloading\0\0\0'__widl_f_onprogress_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0+__widl_f_set_onprogress_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0*__widl_f_onupdateready_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\ronupdateready\x01\x01\x05self_\ronupdateready\0\0\0.__widl_f_set_onupdateready_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\ronupdateready\x01\x02\x05self_\ronupdateready\ronupdateready\0\0\0%__widl_f_oncached_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\x08oncached\x01\x01\x05self_\x08oncached\0\0\0)__widl_f_set_oncached_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\x08oncached\x01\x02\x05self_\x08oncached\x08oncached\0\0\0'__widl_f_onobsolete_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x01\nonobsolete\x01\x01\x05self_\nonobsolete\0\0\0+__widl_f_set_onobsolete_OfflineResourceList\0\0\0\x01\x13OfflineResourceList\x01\0\x02\nonobsolete\x01\x02\x05self_\nonobsolete\nonobsolete\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

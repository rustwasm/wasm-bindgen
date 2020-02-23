use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `XMLHttpRequestEventTarget` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct XmlHttpRequestEventTarget {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_XmlHttpRequestEventTarget: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for XmlHttpRequestEventTarget {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(25u32);
            inform(88u32);
            inform(77u32);
            inform(76u32);
            inform(72u32);
            inform(116u32);
            inform(116u32);
            inform(112u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
            inform(69u32);
            inform(118u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(84u32);
            inform(97u32);
            inform(114u32);
            inform(103u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for XmlHttpRequestEventTarget {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for XmlHttpRequestEventTarget {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for XmlHttpRequestEventTarget {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a XmlHttpRequestEventTarget {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for XmlHttpRequestEventTarget {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            XmlHttpRequestEventTarget {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for XmlHttpRequestEventTarget {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a XmlHttpRequestEventTarget {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for XmlHttpRequestEventTarget {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<XmlHttpRequestEventTarget>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(XmlHttpRequestEventTarget {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for XmlHttpRequestEventTarget {
        #[inline]
        fn from(obj: JsValue) -> XmlHttpRequestEventTarget {
            XmlHttpRequestEventTarget { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for XmlHttpRequestEventTarget {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<XmlHttpRequestEventTarget> for XmlHttpRequestEventTarget {
        #[inline]
        fn as_ref(&self) -> &XmlHttpRequestEventTarget {
            self
        }
    }
    impl From<XmlHttpRequestEventTarget> for JsValue {
        #[inline]
        fn from(obj: XmlHttpRequestEventTarget) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for XmlHttpRequestEventTarget {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_XMLHttpRequestEventTarget(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_XMLHttpRequestEventTarget(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_XMLHttpRequestEventTarget(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            XmlHttpRequestEventTarget { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const XmlHttpRequestEventTarget) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<XmlHttpRequestEventTarget> for EventTarget {
    #[inline]
    fn from(obj: XmlHttpRequestEventTarget) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for XmlHttpRequestEventTarget {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<XmlHttpRequestEventTarget> for ::js_sys::Object {
    #[inline]
    fn from(obj: XmlHttpRequestEventTarget) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for XmlHttpRequestEventTarget {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadstart_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onloadstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadstart_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadstart_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onloadstart_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadstart_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onloadstart(&self, onloadstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadstart_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadstart_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onloadstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadstart,
                    );
                __widl_f_set_onloadstart_XMLHttpRequestEventTarget(self_, onloadstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onprogress_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_XMLHttpRequestEventTarget(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onabort_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_XMLHttpRequestEventTarget(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onerror_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_XMLHttpRequestEventTarget(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onload_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onload_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onload_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onload_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onload_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onload(&self, onload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onload_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onload_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onload,
                    );
                __widl_f_set_onload_XMLHttpRequestEventTarget(self_, onload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontimeout_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn ontimeout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontimeout_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontimeout_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_ontimeout_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontimeout_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_ontimeout(&self, ontimeout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontimeout_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontimeout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontimeout_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontimeout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontimeout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let ontimeout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontimeout,
                    );
                __widl_f_set_ontimeout_XMLHttpRequestEventTarget(self_, ontimeout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadend_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn onloadend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadend_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadend_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onloadend_XMLHttpRequestEventTarget(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "XmlHttpRequestEventTarget",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadend_XMLHttpRequestEventTarget() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&XmlHttpRequestEventTarget as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl XmlHttpRequestEventTarget {
    #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)\n\n*This API requires the following crate features to be activated: `XmlHttpRequestEventTarget`*"]
    #[allow(clippy::all)]
    pub fn set_onloadend(&self, onloadend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "XmlHttpRequestEventTarget",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadend_XMLHttpRequestEventTarget(
                self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadend_XMLHttpRequestEventTarget(
            self_: <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&XmlHttpRequestEventTarget as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onloadend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadend,
                    );
                __widl_f_set_onloadend_XMLHttpRequestEventTarget(self_, onloadend)
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
pub static __WASM_BINDGEN_GENERATED_b3b3aa38b7154dfd: [u8; 1775usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAD\x06\0\0\0\0\x0F\0\0\x02\x19XMLHttpRequestEventTarget+__widl_instanceof_XMLHttpRequestEventTarget\0\0\0\0.__widl_f_onloadstart_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\x0Bonloadstart\x01\x01\x05self_\x0Bonloadstart\0\0\02__widl_f_set_onloadstart_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\x0Bonloadstart\x01\x02\x05self_\x0Bonloadstart\x0Bonloadstart\0\0\0-__widl_f_onprogress_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\01__widl_f_set_onprogress_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0*__widl_f_onabort_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0.__widl_f_set_onabort_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0*__widl_f_onerror_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0.__widl_f_set_onerror_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0)__widl_f_onload_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\x06onload\x01\x01\x05self_\x06onload\0\0\0-__widl_f_set_onload_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\x06onload\x01\x02\x05self_\x06onload\x06onload\0\0\0,__widl_f_ontimeout_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\tontimeout\x01\x01\x05self_\tontimeout\0\0\00__widl_f_set_ontimeout_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\tontimeout\x01\x02\x05self_\tontimeout\tontimeout\0\0\0,__widl_f_onloadend_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x01\tonloadend\x01\x01\x05self_\tonloadend\0\0\00__widl_f_set_onloadend_XMLHttpRequestEventTarget\0\0\0\x01\x19XMLHttpRequestEventTarget\x01\0\x02\tonloadend\x01\x02\x05self_\tonloadend\tonloadend\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

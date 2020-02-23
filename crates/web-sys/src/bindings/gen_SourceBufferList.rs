use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SourceBufferList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SourceBufferList {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SourceBufferList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SourceBufferList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(66u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SourceBufferList {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SourceBufferList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SourceBufferList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SourceBufferList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SourceBufferList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SourceBufferList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SourceBufferList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SourceBufferList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SourceBufferList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SourceBufferList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SourceBufferList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SourceBufferList {
        #[inline]
        fn from(obj: JsValue) -> SourceBufferList {
            SourceBufferList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SourceBufferList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SourceBufferList> for SourceBufferList {
        #[inline]
        fn as_ref(&self) -> &SourceBufferList {
            self
        }
    }
    impl From<SourceBufferList> for JsValue {
        #[inline]
        fn from(obj: SourceBufferList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SourceBufferList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SourceBufferList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SourceBufferList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SourceBufferList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SourceBufferList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SourceBufferList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SourceBufferList> for EventTarget {
    #[inline]
    fn from(obj: SourceBufferList) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SourceBufferList {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SourceBufferList> for ::js_sys::Object {
    #[inline]
    fn from(obj: SourceBufferList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SourceBufferList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SourceBuffer", feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<SourceBuffer> as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<SourceBuffer> {
        #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SourceBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SourceBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_SourceBufferList(self_, index)
            };
            <Option<SourceBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_SourceBufferList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaddsourcebuffer_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `onaddsourcebuffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn onaddsourcebuffer(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaddsourcebuffer_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaddsourcebuffer_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaddsourcebuffer_SourceBufferList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaddsourcebuffer_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `onaddsourcebuffer` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn set_onaddsourcebuffer(&self, onaddsourcebuffer: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaddsourcebuffer_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaddsourcebuffer : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaddsourcebuffer_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaddsourcebuffer : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onaddsourcebuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaddsourcebuffer =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaddsourcebuffer,
                    );
                __widl_f_set_onaddsourcebuffer_SourceBufferList(self_, onaddsourcebuffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onremovesourcebuffer_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `onremovesourcebuffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn onremovesourcebuffer(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onremovesourcebuffer_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onremovesourcebuffer_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onremovesourcebuffer_SourceBufferList(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onremovesourcebuffer_SourceBufferList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBufferList as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBufferList {
    #[cfg(all(feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `onremovesourcebuffer` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)\n\n*This API requires the following crate features to be activated: `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn set_onremovesourcebuffer(&self, onremovesourcebuffer: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onremovesourcebuffer_SourceBufferList(
                self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onremovesourcebuffer : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onremovesourcebuffer_SourceBufferList(
            self_: <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onremovesourcebuffer : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onremovesourcebuffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SourceBufferList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onremovesourcebuffer =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onremovesourcebuffer,
                    );
                __widl_f_set_onremovesourcebuffer_SourceBufferList(self_, onremovesourcebuffer)
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
pub static __WASM_BINDGEN_GENERATED_471777b7df6fed2c: [u8; 843usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\t\x03\0\0\0\0\x07\0\0\x02\x10SourceBufferList\"__widl_instanceof_SourceBufferList\0\0\0\0\x1D__widl_f_get_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0 __widl_f_length_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0+__widl_f_onaddsourcebuffer_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x01\x11onaddsourcebuffer\x01\x01\x05self_\x11onaddsourcebuffer\0\0\0/__widl_f_set_onaddsourcebuffer_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x02\x11onaddsourcebuffer\x01\x02\x05self_\x11onaddsourcebuffer\x11onaddsourcebuffer\0\0\0.__widl_f_onremovesourcebuffer_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x01\x14onremovesourcebuffer\x01\x01\x05self_\x14onremovesourcebuffer\0\0\02__widl_f_set_onremovesourcebuffer_SourceBufferList\0\0\0\x01\x10SourceBufferList\x01\0\x02\x14onremovesourcebuffer\x01\x02\x05self_\x14onremovesourcebuffer\x14onremovesourcebuffer\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

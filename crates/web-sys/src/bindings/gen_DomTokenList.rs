use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMTokenList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomTokenList {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomTokenList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomTokenList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(84u32);
            inform(111u32);
            inform(107u32);
            inform(101u32);
            inform(110u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DomTokenList {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomTokenList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomTokenList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomTokenList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomTokenList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomTokenList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomTokenList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomTokenList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomTokenList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomTokenList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomTokenList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomTokenList {
        #[inline]
        fn from(obj: JsValue) -> DomTokenList {
            DomTokenList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomTokenList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomTokenList> for DomTokenList {
        #[inline]
        fn as_ref(&self) -> &DomTokenList {
            self
        }
    }
    impl From<DomTokenList> for JsValue {
        #[inline]
        fn from(obj: DomTokenList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomTokenList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMTokenList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMTokenList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMTokenList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomTokenList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomTokenList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomTokenList> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomTokenList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomTokenList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add(&self, tokens: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens);
                __widl_f_add_DOMTokenList(self_, tokens)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_0_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomTokenList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_0_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_0_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_add_0_DOMTokenList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_1_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_1(&self, tokens_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_1_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_1_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                __widl_f_add_1_DOMTokenList(self_, tokens_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_2_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_2(&self, tokens_1: &str, tokens_2: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_2_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_2_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                __widl_f_add_2_DOMTokenList(self_, tokens_1, tokens_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_3_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_3(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_3_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_3_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                __widl_f_add_3_DOMTokenList(self_, tokens_1, tokens_2, tokens_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_4_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_4(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_4_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_4_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                __widl_f_add_4_DOMTokenList(self_, tokens_1, tokens_2, tokens_3, tokens_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_5_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_5(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_5_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_5_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                __widl_f_add_5_DOMTokenList(self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_6_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_6(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_6_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_6_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            drop(tokens_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                let tokens_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_6);
                __widl_f_add_6_DOMTokenList(
                    self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5, tokens_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_7_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn add_7(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_7_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_7_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            drop(tokens_6);
            drop(tokens_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                let tokens_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_6);
                let tokens_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_7);
                __widl_f_add_7_DOMTokenList(
                    self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5, tokens_6, tokens_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_contains_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `contains()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn contains(&self, token: &str) -> bool {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_contains_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_contains_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(token);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(token);
                __widl_f_contains_DOMTokenList(self_, token)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/item)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<String> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_DOMTokenList(self_, index)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&::js_sys::Array as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove(&self, tokens: &::js_sys::Array) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens: <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens =
                    <&::js_sys::Array as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens);
                __widl_f_remove_DOMTokenList(self_, tokens)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_0_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomTokenList as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_0(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_0_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_0_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_0_DOMTokenList(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_1_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_1(&self, tokens_1: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_1_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_1_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                __widl_f_remove_1_DOMTokenList(self_, tokens_1)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_2_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_2(&self, tokens_1: &str, tokens_2: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_2_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_2_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                __widl_f_remove_2_DOMTokenList(self_, tokens_1, tokens_2)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_3_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_3(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_3_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_3_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                __widl_f_remove_3_DOMTokenList(self_, tokens_1, tokens_2, tokens_3)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_4_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_4(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_4_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_4_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                __widl_f_remove_4_DOMTokenList(self_, tokens_1, tokens_2, tokens_3, tokens_4)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_5_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_5(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_5_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_5_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                __widl_f_remove_5_DOMTokenList(
                    self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_6_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_6(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_6_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_6_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            drop(tokens_6);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                let tokens_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_6);
                __widl_f_remove_6_DOMTokenList(
                    self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5, tokens_6,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_7_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn remove_7(
        &self,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_7_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tokens_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_7_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_1: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_2: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_3: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_4: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_5: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_6: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tokens_7: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tokens_1);
            drop(tokens_2);
            drop(tokens_3);
            drop(tokens_4);
            drop(tokens_5);
            drop(tokens_6);
            drop(tokens_7);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tokens_1 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_1);
                let tokens_2 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_2);
                let tokens_3 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_3);
                let tokens_4 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_4);
                let tokens_5 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_5);
                let tokens_6 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_6);
                let tokens_7 = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tokens_7);
                __widl_f_remove_7_DOMTokenList(
                    self_, tokens_1, tokens_2, tokens_3, tokens_4, tokens_5, tokens_6, tokens_7,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_replace_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `replace()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn replace(&self, token: &str, new_token: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_replace_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                new_token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_replace_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            new_token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(token);
            drop(new_token);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(token);
                let new_token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(new_token);
                __widl_f_replace_DOMTokenList(self_, token, new_token)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_supports_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `supports()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn supports(&self, token: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_supports_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_supports_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(token);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(token);
                __widl_f_supports_DOMTokenList(self_, token)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_toggle_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `toggle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn toggle(&self, token: &str) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_toggle_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_toggle_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(token);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(token);
                __widl_f_toggle_DOMTokenList(self_, token)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_toggle_with_force_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `toggle()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn toggle_with_force(
        &self,
        token: &str,
        force: bool,
    ) -> Result<bool, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_toggle_with_force_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                force: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_toggle_with_force_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            token: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            force: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(token);
            drop(force);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let token = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(token);
                let force = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(force);
                __widl_f_toggle_with_force_DOMTokenList(self_, token, force)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<String> {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_DOMTokenList(self_, index)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomTokenList as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_DOMTokenList(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomTokenList as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_DOMTokenList(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomTokenList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_DOMTokenList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomTokenList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomTokenList {
    #[cfg(all(feature = "DomTokenList",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)\n\n*This API requires the following crate features to be activated: `DomTokenList`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "DomTokenList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_DOMTokenList(
                self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_DOMTokenList(
            self_: <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomTokenList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_DOMTokenList(self_, value)
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
pub static __WASM_BINDGEN_GENERATED_24154959e5bf8204: [u8; 2619usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF9\t\0\0\0\0\x1D\0\0\x02\x0CDOMTokenList\x1E__widl_instanceof_DOMTokenList\0\0\0\0\x19__widl_f_add_DOMTokenList\x01\x01\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x06tokens\x03add\0\0\0\x1B__widl_f_add_0_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x01\x05self_\x03add\0\0\0\x1B__widl_f_add_1_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x08tokens_1\x03add\0\0\0\x1B__widl_f_add_2_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x03\x05self_\x08tokens_1\x08tokens_2\x03add\0\0\0\x1B__widl_f_add_3_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x04\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x03add\0\0\0\x1B__widl_f_add_4_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x05\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x03add\0\0\0\x1B__widl_f_add_5_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x06\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x03add\0\0\0\x1B__widl_f_add_6_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x07\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x08tokens_6\x03add\0\0\0\x1B__widl_f_add_7_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x08\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x08tokens_6\x08tokens_7\x03add\0\0\0\x1E__widl_f_contains_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x05token\x08contains\0\0\0\x1A__widl_f_item_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0\x1C__widl_f_remove_DOMTokenList\x01\x01\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x06tokens\x06remove\0\0\0\x1E__widl_f_remove_0_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x01\x05self_\x06remove\0\0\0\x1E__widl_f_remove_1_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x08tokens_1\x06remove\0\0\0\x1E__widl_f_remove_2_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x03\x05self_\x08tokens_1\x08tokens_2\x06remove\0\0\0\x1E__widl_f_remove_3_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x04\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x06remove\0\0\0\x1E__widl_f_remove_4_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x05\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x06remove\0\0\0\x1E__widl_f_remove_5_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x06\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x06remove\0\0\0\x1E__widl_f_remove_6_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x07\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x08tokens_6\x06remove\0\0\0\x1E__widl_f_remove_7_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x08\x05self_\x08tokens_1\x08tokens_2\x08tokens_3\x08tokens_4\x08tokens_5\x08tokens_6\x08tokens_7\x06remove\0\0\0\x1D__widl_f_replace_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x03\x05self_\x05token\tnew_token\x07replace\0\0\0\x1E__widl_f_supports_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x05token\x08supports\0\0\0\x1C__widl_f_toggle_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x02\x05self_\x05token\x06toggle\0\0\0'__widl_f_toggle_with_force_DOMTokenList\x01\0\0\x01\x0CDOMTokenList\x01\0\0\x01\x03\x05self_\x05token\x05force\x06toggle\0\0\0\x19__widl_f_get_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1C__widl_f_length_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\x1B__widl_f_value_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0\x1F__widl_f_set_value_DOMTokenList\0\0\0\x01\x0CDOMTokenList\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

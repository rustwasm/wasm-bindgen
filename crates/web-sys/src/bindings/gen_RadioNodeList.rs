use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RadioNodeList` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList)\n\n*This API requires the following crate features to be activated: `RadioNodeList`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RadioNodeList {
    obj: NodeList,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RadioNodeList: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RadioNodeList {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(82u32);
            inform(97u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(76u32);
            inform(105u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for RadioNodeList {
        type Target = NodeList;
        #[inline]
        fn deref(&self) -> &NodeList {
            &self.obj
        }
    }
    impl IntoWasmAbi for RadioNodeList {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RadioNodeList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RadioNodeList {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RadioNodeList {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RadioNodeList {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RadioNodeList {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RadioNodeList {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RadioNodeList {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RadioNodeList>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RadioNodeList {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RadioNodeList {
        #[inline]
        fn from(obj: JsValue) -> RadioNodeList {
            RadioNodeList { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RadioNodeList {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RadioNodeList> for RadioNodeList {
        #[inline]
        fn as_ref(&self) -> &RadioNodeList {
            self
        }
    }
    impl From<RadioNodeList> for JsValue {
        #[inline]
        fn from(obj: RadioNodeList) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RadioNodeList {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RadioNodeList(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RadioNodeList(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RadioNodeList(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RadioNodeList { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RadioNodeList) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RadioNodeList> for NodeList {
    #[inline]
    fn from(obj: RadioNodeList) -> NodeList {
        use wasm_bindgen::JsCast;
        NodeList::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<NodeList> for RadioNodeList {
    #[inline]
    fn as_ref(&self) -> &NodeList {
        use wasm_bindgen::JsCast;
        NodeList::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RadioNodeList> for ::js_sys::Object {
    #[inline]
    fn from(obj: RadioNodeList) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RadioNodeList {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RadioNodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_RadioNodeList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RadioNodeList as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RadioNodeList {
    #[cfg(all(feature = "RadioNodeList",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)\n\n*This API requires the following crate features to be activated: `RadioNodeList`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "RadioNodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_RadioNodeList(
                self_: <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_RadioNodeList(
            self_: <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_RadioNodeList(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RadioNodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_RadioNodeList() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RadioNodeList as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RadioNodeList {
    #[cfg(all(feature = "RadioNodeList",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)\n\n*This API requires the following crate features to be activated: `RadioNodeList`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "RadioNodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_RadioNodeList(
                self_: <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_RadioNodeList(
            self_: <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RadioNodeList as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_RadioNodeList(self_, value)
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
pub static __WASM_BINDGEN_GENERATED_8e3e9751b99d4f4a: [u8; 311usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF5\0\0\0\0\0\x03\0\0\x02\rRadioNodeList\x1F__widl_instanceof_RadioNodeList\0\0\0\0\x1C__widl_f_value_RadioNodeList\0\0\0\x01\rRadioNodeList\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0 __widl_f_set_value_RadioNodeList\0\0\0\x01\rRadioNodeList\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

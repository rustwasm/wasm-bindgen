use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ProcessingInstruction` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ProcessingInstruction {
    obj: CharacterData,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ProcessingInstruction: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ProcessingInstruction {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(110u32);
            inform(103u32);
            inform(73u32);
            inform(110u32);
            inform(115u32);
            inform(116u32);
            inform(114u32);
            inform(117u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for ProcessingInstruction {
        type Target = CharacterData;
        #[inline]
        fn deref(&self) -> &CharacterData {
            &self.obj
        }
    }
    impl IntoWasmAbi for ProcessingInstruction {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ProcessingInstruction {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ProcessingInstruction {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ProcessingInstruction {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ProcessingInstruction {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ProcessingInstruction {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ProcessingInstruction {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ProcessingInstruction {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ProcessingInstruction>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ProcessingInstruction {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ProcessingInstruction {
        #[inline]
        fn from(obj: JsValue) -> ProcessingInstruction {
            ProcessingInstruction { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ProcessingInstruction {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ProcessingInstruction> for ProcessingInstruction {
        #[inline]
        fn as_ref(&self) -> &ProcessingInstruction {
            self
        }
    }
    impl From<ProcessingInstruction> for JsValue {
        #[inline]
        fn from(obj: ProcessingInstruction) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ProcessingInstruction {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ProcessingInstruction(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ProcessingInstruction(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ProcessingInstruction(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ProcessingInstruction { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ProcessingInstruction) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ProcessingInstruction> for CharacterData {
    #[inline]
    fn from(obj: ProcessingInstruction) -> CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CharacterData> for ProcessingInstruction {
    #[inline]
    fn as_ref(&self) -> &CharacterData {
        use wasm_bindgen::JsCast;
        CharacterData::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ProcessingInstruction> for Node {
    #[inline]
    fn from(obj: ProcessingInstruction) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for ProcessingInstruction {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ProcessingInstruction> for EventTarget {
    #[inline]
    fn from(obj: ProcessingInstruction) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ProcessingInstruction {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ProcessingInstruction> for ::js_sys::Object {
    #[inline]
    fn from(obj: ProcessingInstruction) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ProcessingInstruction {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ProcessingInstruction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_ProcessingInstruction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ProcessingInstruction as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl ProcessingInstruction {
    #[cfg(all(feature = "ProcessingInstruction",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "ProcessingInstruction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_ProcessingInstruction(
                self_: <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_ProcessingInstruction(
            self_: <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_ProcessingInstruction(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ProcessingInstruction", feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sheet_ProcessingInstruction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ProcessingInstruction as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl ProcessingInstruction {
    #[cfg(all(feature = "ProcessingInstruction", feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `sheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)\n\n*This API requires the following crate features to be activated: `ProcessingInstruction`, `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn sheet(&self) -> Option<StyleSheet> {
        #[cfg(all(feature = "ProcessingInstruction", feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sheet_ProcessingInstruction(
                self_: <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sheet_ProcessingInstruction(
            self_: <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ProcessingInstruction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sheet_ProcessingInstruction(self_)
            };
            <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_704881e8f86674db: [u8; 352usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1E\x01\0\0\0\0\x03\0\0\x02\x15ProcessingInstruction'__widl_instanceof_ProcessingInstruction\0\0\0\0%__widl_f_target_ProcessingInstruction\0\0\0\x01\x15ProcessingInstruction\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0$__widl_f_sheet_ProcessingInstruction\0\0\0\x01\x15ProcessingInstruction\x01\0\x01\x05sheet\x01\x01\x05self_\x05sheet\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ScriptProcessorNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ScriptProcessorNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ScriptProcessorNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ScriptProcessorNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(83u32);
            inform(99u32);
            inform(114u32);
            inform(105u32);
            inform(112u32);
            inform(116u32);
            inform(80u32);
            inform(114u32);
            inform(111u32);
            inform(99u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(111u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for ScriptProcessorNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for ScriptProcessorNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ScriptProcessorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ScriptProcessorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ScriptProcessorNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ScriptProcessorNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ScriptProcessorNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ScriptProcessorNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ScriptProcessorNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ScriptProcessorNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ScriptProcessorNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ScriptProcessorNode {
        #[inline]
        fn from(obj: JsValue) -> ScriptProcessorNode {
            ScriptProcessorNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ScriptProcessorNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ScriptProcessorNode> for ScriptProcessorNode {
        #[inline]
        fn as_ref(&self) -> &ScriptProcessorNode {
            self
        }
    }
    impl From<ScriptProcessorNode> for JsValue {
        #[inline]
        fn from(obj: ScriptProcessorNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ScriptProcessorNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ScriptProcessorNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ScriptProcessorNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ScriptProcessorNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ScriptProcessorNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ScriptProcessorNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ScriptProcessorNode> for AudioNode {
    #[inline]
    fn from(obj: ScriptProcessorNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for ScriptProcessorNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ScriptProcessorNode> for EventTarget {
    #[inline]
    fn from(obj: ScriptProcessorNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ScriptProcessorNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ScriptProcessorNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: ScriptProcessorNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ScriptProcessorNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaudioprocess_ScriptProcessorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScriptProcessorNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ScriptProcessorNode {
    #[cfg(all(feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `onaudioprocess` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn onaudioprocess(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaudioprocess_ScriptProcessorNode(
                self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaudioprocess_ScriptProcessorNode(
            self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaudioprocess_ScriptProcessorNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaudioprocess_ScriptProcessorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ScriptProcessorNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ScriptProcessorNode {
    #[cfg(all(feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `onaudioprocess` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn set_onaudioprocess(&self, onaudioprocess: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaudioprocess_ScriptProcessorNode(
                self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaudioprocess : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaudioprocess_ScriptProcessorNode(
            self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaudioprocess : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onaudioprocess);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaudioprocess =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaudioprocess,
                    );
                __widl_f_set_onaudioprocess_ScriptProcessorNode(self_, onaudioprocess)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_size_ScriptProcessorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ScriptProcessorNode as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl ScriptProcessorNode {
    #[cfg(all(feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `bufferSize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)\n\n*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn buffer_size(&self) -> i32 {
        #[cfg(all(feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_size_ScriptProcessorNode(
                self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_size_ScriptProcessorNode(
            self_: <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&ScriptProcessorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffer_size_ScriptProcessorNode(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_9392d49739d03314: [u8; 511usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBD\x01\0\0\0\0\x04\0\0\x02\x13ScriptProcessorNode%__widl_instanceof_ScriptProcessorNode\0\0\0\0+__widl_f_onaudioprocess_ScriptProcessorNode\0\0\0\x01\x13ScriptProcessorNode\x01\0\x01\x0Eonaudioprocess\x01\x01\x05self_\x0Eonaudioprocess\0\0\0/__widl_f_set_onaudioprocess_ScriptProcessorNode\0\0\0\x01\x13ScriptProcessorNode\x01\0\x02\x0Eonaudioprocess\x01\x02\x05self_\x0Eonaudioprocess\x0Eonaudioprocess\0\0\0(__widl_f_buffer_size_ScriptProcessorNode\0\0\0\x01\x13ScriptProcessorNode\x01\0\x01\nbufferSize\x01\x01\x05self_\nbufferSize\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

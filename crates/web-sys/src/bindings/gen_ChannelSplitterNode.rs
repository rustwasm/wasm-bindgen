use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ChannelSplitterNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `ChannelSplitterNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ChannelSplitterNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ChannelSplitterNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ChannelSplitterNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(108u32);
            inform(83u32);
            inform(112u32);
            inform(108u32);
            inform(105u32);
            inform(116u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for ChannelSplitterNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for ChannelSplitterNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ChannelSplitterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ChannelSplitterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ChannelSplitterNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ChannelSplitterNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ChannelSplitterNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ChannelSplitterNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ChannelSplitterNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ChannelSplitterNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ChannelSplitterNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ChannelSplitterNode {
        #[inline]
        fn from(obj: JsValue) -> ChannelSplitterNode {
            ChannelSplitterNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ChannelSplitterNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ChannelSplitterNode> for ChannelSplitterNode {
        #[inline]
        fn as_ref(&self) -> &ChannelSplitterNode {
            self
        }
    }
    impl From<ChannelSplitterNode> for JsValue {
        #[inline]
        fn from(obj: ChannelSplitterNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ChannelSplitterNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ChannelSplitterNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ChannelSplitterNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ChannelSplitterNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ChannelSplitterNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ChannelSplitterNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ChannelSplitterNode> for AudioNode {
    #[inline]
    fn from(obj: ChannelSplitterNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for ChannelSplitterNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ChannelSplitterNode> for EventTarget {
    #[inline]
    fn from(obj: ChannelSplitterNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ChannelSplitterNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ChannelSplitterNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: ChannelSplitterNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ChannelSplitterNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ChannelSplitterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl ChannelSplitterNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterNode",))]
    #[allow(bad_style)]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ChannelSplitterNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ChannelSplitterNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                __widl_f_new_ChannelSplitterNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "ChannelSplitterNode",
    feature = "ChannelSplitterOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_ChannelSplitterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&ChannelSplitterOptions as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl ChannelSplitterNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "ChannelSplitterNode",
        feature = "ChannelSplitterOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`, `ChannelSplitterOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ChannelSplitterOptions,
    ) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "ChannelSplitterNode",
            feature = "ChannelSplitterOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_ChannelSplitterNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ChannelSplitterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_ChannelSplitterNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ChannelSplitterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let options =
                    <&ChannelSplitterOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_ChannelSplitterNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_4ef0476807413528: [u8; 338usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x10\x01\0\0\0\0\x03\0\0\x02\x13ChannelSplitterNode%__widl_instanceof_ChannelSplitterNode\0\0\0\0 __widl_f_new_ChannelSplitterNode\x01\0\0\x01\x13ChannelSplitterNode\0\x01\x01\x07context\x03new\0\0\0-__widl_f_new_with_options_ChannelSplitterNode\x01\0\0\x01\x13ChannelSplitterNode\0\x01\x02\x07context\x07options\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

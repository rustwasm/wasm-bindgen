use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ConvolverNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ConvolverNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ConvolverNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ConvolverNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(118u32);
            inform(111u32);
            inform(108u32);
            inform(118u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for ConvolverNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for ConvolverNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ConvolverNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ConvolverNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ConvolverNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ConvolverNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ConvolverNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ConvolverNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ConvolverNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ConvolverNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ConvolverNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ConvolverNode {
        #[inline]
        fn from(obj: JsValue) -> ConvolverNode {
            ConvolverNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ConvolverNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ConvolverNode> for ConvolverNode {
        #[inline]
        fn as_ref(&self) -> &ConvolverNode {
            self
        }
    }
    impl From<ConvolverNode> for JsValue {
        #[inline]
        fn from(obj: ConvolverNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ConvolverNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ConvolverNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ConvolverNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ConvolverNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ConvolverNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ConvolverNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ConvolverNode> for AudioNode {
    #[inline]
    fn from(obj: ConvolverNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for ConvolverNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ConvolverNode> for EventTarget {
    #[inline]
    fn from(obj: ConvolverNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ConvolverNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ConvolverNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: ConvolverNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ConvolverNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <ConvolverNode as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<ConvolverNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ConvolverNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ConvolverNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_ConvolverNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "ConvolverNode",
    feature = "ConvolverOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&ConvolverOptions as WasmDescribe>::describe();
    <ConvolverNode as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "ConvolverNode",
        feature = "ConvolverOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`, `ConvolverOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConvolverOptions,
    ) -> Result<ConvolverNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "ConvolverNode",
            feature = "ConvolverOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_ConvolverNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConvolverOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_ConvolverNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConvolverOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ConvolverOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_ConvolverNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConvolverNode as WasmDescribe>::describe();
    <Option<AudioBuffer> as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `buffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn buffer(&self) -> Option<AudioBuffer> {
        #[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_ConvolverNode(
                self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_ConvolverNode(
            self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffer_ConvolverNode(self_)
            };
            <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_buffer_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ConvolverNode as WasmDescribe>::describe();
    <Option<&AudioBuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `buffer` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn set_buffer(&self, buffer: Option<&AudioBuffer>) {
        #[cfg(all(feature = "AudioBuffer", feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_buffer_ConvolverNode(
                self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer: <Option<&AudioBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_buffer_ConvolverNode(
            self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer: <Option<&AudioBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(buffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer =
                    <Option<&AudioBuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                __widl_f_set_buffer_ConvolverNode(self_, buffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_normalize_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConvolverNode as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `normalize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn normalize(&self) -> bool {
        #[cfg(all(feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_normalize_ConvolverNode(
                self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_normalize_ConvolverNode(
            self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_normalize_ConvolverNode(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_normalize_ConvolverNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ConvolverNode as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConvolverNode {
    #[cfg(all(feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `normalize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)\n\n*This API requires the following crate features to be activated: `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn set_normalize(&self, normalize: bool) {
        #[cfg(all(feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_normalize_ConvolverNode(
                self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                normalize: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_normalize_ConvolverNode(
            self_: <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            normalize: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(normalize);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&ConvolverNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let normalize = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(normalize);
                __widl_f_set_normalize_ConvolverNode(self_, normalize)
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
pub static __WASM_BINDGEN_GENERATED_00fb615363cf4d71: [u8; 649usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}G\x02\0\0\0\0\x07\0\0\x02\rConvolverNode\x1F__widl_instanceof_ConvolverNode\0\0\0\0\x1A__widl_f_new_ConvolverNode\x01\0\0\x01\rConvolverNode\0\x01\x01\x07context\x03new\0\0\0'__widl_f_new_with_options_ConvolverNode\x01\0\0\x01\rConvolverNode\0\x01\x02\x07context\x07options\x03new\0\0\0\x1D__widl_f_buffer_ConvolverNode\0\0\0\x01\rConvolverNode\x01\0\x01\x06buffer\x01\x01\x05self_\x06buffer\0\0\0!__widl_f_set_buffer_ConvolverNode\0\0\0\x01\rConvolverNode\x01\0\x02\x06buffer\x01\x02\x05self_\x06buffer\x06buffer\0\0\0 __widl_f_normalize_ConvolverNode\0\0\0\x01\rConvolverNode\x01\0\x01\tnormalize\x01\x01\x05self_\tnormalize\0\0\0$__widl_f_set_normalize_ConvolverNode\0\0\0\x01\rConvolverNode\x01\0\x02\tnormalize\x01\x02\x05self_\tnormalize\tnormalize\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

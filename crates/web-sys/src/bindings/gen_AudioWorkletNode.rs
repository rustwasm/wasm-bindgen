use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioWorkletNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioWorkletNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioWorkletNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioWorkletNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(108u32);
            inform(101u32);
            inform(116u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AudioWorkletNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioWorkletNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioWorkletNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioWorkletNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioWorkletNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioWorkletNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioWorkletNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioWorkletNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioWorkletNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioWorkletNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioWorkletNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioWorkletNode {
        #[inline]
        fn from(obj: JsValue) -> AudioWorkletNode {
            AudioWorkletNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioWorkletNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioWorkletNode> for AudioWorkletNode {
        #[inline]
        fn as_ref(&self) -> &AudioWorkletNode {
            self
        }
    }
    impl From<AudioWorkletNode> for JsValue {
        #[inline]
        fn from(obj: AudioWorkletNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioWorkletNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioWorkletNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioWorkletNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioWorkletNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioWorkletNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioWorkletNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioWorkletNode> for AudioNode {
    #[inline]
    fn from(obj: AudioWorkletNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for AudioWorkletNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioWorkletNode> for EventTarget {
    #[inline]
    fn from(obj: AudioWorkletNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioWorkletNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioWorkletNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioWorkletNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioWorkletNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioWorkletNode", feature = "BaseAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <AudioWorkletNode as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(feature = "AudioWorkletNode", feature = "BaseAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &BaseAudioContext,
        name: &str,
    ) -> Result<AudioWorkletNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioWorkletNode", feature = "BaseAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AudioWorkletNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AudioWorkletNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_new_AudioWorkletNode(context, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AudioWorkletNode",
    feature = "AudioWorkletNodeOptions",
    feature = "BaseAudioContext",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&AudioWorkletNodeOptions as WasmDescribe>::describe();
    <AudioWorkletNode as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(
        feature = "AudioWorkletNode",
        feature = "AudioWorkletNodeOptions",
        feature = "BaseAudioContext",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`, `AudioWorkletNodeOptions`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        name: &str,
        options: &AudioWorkletNodeOptions,
    ) -> Result<AudioWorkletNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioWorkletNode",
            feature = "AudioWorkletNodeOptions",
            feature = "BaseAudioContext",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_AudioWorkletNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AudioWorkletNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_AudioWorkletNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AudioWorkletNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(name);
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
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let options =
                    <&AudioWorkletNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_AudioWorkletNode(context, name, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioWorkletNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParamMap", feature = "AudioWorkletNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_parameters_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletNode as WasmDescribe>::describe();
    <AudioParamMap as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(feature = "AudioParamMap", feature = "AudioWorkletNode",))]
    #[allow(bad_style)]
    #[doc = "The `parameters` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/parameters)\n\n*This API requires the following crate features to be activated: `AudioParamMap`, `AudioWorkletNode`*"]
    #[allow(clippy::all)]
    pub fn parameters(&self) -> Result<AudioParamMap, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioParamMap", feature = "AudioWorkletNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_parameters_AudioWorkletNode(
                self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParamMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_parameters_AudioWorkletNode(
            self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParamMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_parameters_AudioWorkletNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioParamMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioWorkletNode", feature = "MessagePort",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletNode as WasmDescribe>::describe();
    <MessagePort as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(feature = "AudioWorkletNode", feature = "MessagePort",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/port)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`, `MessagePort`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> Result<MessagePort, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioWorkletNode", feature = "MessagePort",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_AudioWorkletNode(
                self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_AudioWorkletNode(
            self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MessagePort as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_AudioWorkletNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MessagePort as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioWorkletNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprocessorerror_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioWorkletNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(feature = "AudioWorkletNode",))]
    #[allow(bad_style)]
    #[doc = "The `onprocessorerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
    #[allow(clippy::all)]
    pub fn onprocessorerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "AudioWorkletNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprocessorerror_AudioWorkletNode(
                self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprocessorerror_AudioWorkletNode(
            self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprocessorerror_AudioWorkletNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioWorkletNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprocessorerror_AudioWorkletNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioWorkletNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioWorkletNode {
    #[cfg(all(feature = "AudioWorkletNode",))]
    #[allow(bad_style)]
    #[doc = "The `onprocessorerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)\n\n*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
    #[allow(clippy::all)]
    pub fn set_onprocessorerror(&self, onprocessorerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "AudioWorkletNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprocessorerror_AudioWorkletNode(
                self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprocessorerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprocessorerror_AudioWorkletNode(
            self_: <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprocessorerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onprocessorerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioWorkletNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprocessorerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprocessorerror,
                    );
                __widl_f_set_onprocessorerror_AudioWorkletNode(self_, onprocessorerror)
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
pub static __WASM_BINDGEN_GENERATED_934a366ffcc6b78f: [u8; 745usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA7\x02\0\0\0\0\x07\0\0\x02\x10AudioWorkletNode\"__widl_instanceof_AudioWorkletNode\0\0\0\0\x1D__widl_f_new_AudioWorkletNode\x01\0\0\x01\x10AudioWorkletNode\0\x01\x02\x07context\x04name\x03new\0\0\0*__widl_f_new_with_options_AudioWorkletNode\x01\0\0\x01\x10AudioWorkletNode\0\x01\x03\x07context\x04name\x07options\x03new\0\0\0$__widl_f_parameters_AudioWorkletNode\x01\0\0\x01\x10AudioWorkletNode\x01\0\x01\nparameters\x01\x01\x05self_\nparameters\0\0\0\x1E__widl_f_port_AudioWorkletNode\x01\0\0\x01\x10AudioWorkletNode\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0*__widl_f_onprocessorerror_AudioWorkletNode\0\0\0\x01\x10AudioWorkletNode\x01\0\x01\x10onprocessorerror\x01\x01\x05self_\x10onprocessorerror\0\0\0.__widl_f_set_onprocessorerror_AudioWorkletNode\0\0\0\x01\x10AudioWorkletNode\x01\0\x02\x10onprocessorerror\x01\x02\x05self_\x10onprocessorerror\x10onprocessorerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

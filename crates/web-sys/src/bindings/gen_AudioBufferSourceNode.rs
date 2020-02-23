use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioBufferSourceNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioBufferSourceNode {
    obj: AudioScheduledSourceNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioBufferSourceNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioBufferSourceNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(21u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(66u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AudioBufferSourceNode {
        type Target = AudioScheduledSourceNode;
        #[inline]
        fn deref(&self) -> &AudioScheduledSourceNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioBufferSourceNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioBufferSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioBufferSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioBufferSourceNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioBufferSourceNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioBufferSourceNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioBufferSourceNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioBufferSourceNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioBufferSourceNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioBufferSourceNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioBufferSourceNode {
        #[inline]
        fn from(obj: JsValue) -> AudioBufferSourceNode {
            AudioBufferSourceNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioBufferSourceNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioBufferSourceNode> for AudioBufferSourceNode {
        #[inline]
        fn as_ref(&self) -> &AudioBufferSourceNode {
            self
        }
    }
    impl From<AudioBufferSourceNode> for JsValue {
        #[inline]
        fn from(obj: AudioBufferSourceNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioBufferSourceNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioBufferSourceNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioBufferSourceNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioBufferSourceNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioBufferSourceNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioBufferSourceNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioBufferSourceNode> for AudioScheduledSourceNode {
    #[inline]
    fn from(obj: AudioBufferSourceNode) -> AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioScheduledSourceNode> for AudioBufferSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioBufferSourceNode> for AudioNode {
    #[inline]
    fn from(obj: AudioBufferSourceNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for AudioBufferSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioBufferSourceNode> for EventTarget {
    #[inline]
    fn from(obj: AudioBufferSourceNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioBufferSourceNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioBufferSourceNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioBufferSourceNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioBufferSourceNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioBufferSourceNode", feature = "BaseAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <AudioBufferSourceNode as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode", feature = "BaseAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &BaseAudioContext,
    ) -> Result<AudioBufferSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode", feature = "BaseAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AudioBufferSourceNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AudioBufferSourceNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_AudioBufferSourceNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AudioBufferSourceNode",
    feature = "AudioBufferSourceOptions",
    feature = "BaseAudioContext",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&AudioBufferSourceOptions as WasmDescribe>::describe();
    <AudioBufferSourceNode as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(
        feature = "AudioBufferSourceNode",
        feature = "AudioBufferSourceOptions",
        feature = "BaseAudioContext",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioBufferSourceOptions`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &AudioBufferSourceOptions,
    ) -> Result<AudioBufferSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioBufferSourceNode",
            feature = "AudioBufferSourceOptions",
            feature = "BaseAudioContext",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_AudioBufferSourceNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AudioBufferSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_AudioBufferSourceNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AudioBufferSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&AudioBufferSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_AudioBufferSourceNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_AudioBufferSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_start_with_when_AudioBufferSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_and_grain_offset_AudioBufferSourceNode(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when_and_grain_offset(
        &self,
        when: f64,
        grain_offset: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_and_grain_offset_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                grain_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_and_grain_offset_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            grain_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            drop(grain_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                let grain_offset =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(grain_offset);
                __widl_f_start_with_when_and_grain_offset_AudioBufferSourceNode(
                    self_,
                    when,
                    grain_offset,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_and_grain_offset_and_grain_duration_AudioBufferSourceNode(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when_and_grain_offset_and_grain_duration(
        &self,
        when: f64,
        grain_offset: f64,
        grain_duration: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_and_grain_offset_and_grain_duration_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                grain_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                grain_duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_and_grain_offset_and_grain_duration_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            grain_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            grain_duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            drop(grain_offset);
            drop(grain_duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                let grain_offset =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(grain_offset);
                let grain_duration =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(grain_duration);
                __widl_f_start_with_when_and_grain_offset_and_grain_duration_AudioBufferSourceNode(
                    self_,
                    when,
                    grain_offset,
                    grain_duration,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_AudioBufferSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_with_when_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_with_when_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_with_when_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_stop_with_when_AudioBufferSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffer_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <Option<AudioBuffer> as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `buffer` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn buffer(&self) -> Option<AudioBuffer> {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffer_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffer_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffer_AudioBufferSourceNode(self_)
            };
            <Option<AudioBuffer> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_buffer_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <Option<&AudioBuffer> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `buffer` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_buffer(&self, buffer: Option<&AudioBuffer>) {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_buffer_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer: <Option<&AudioBuffer> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_buffer_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer =
                    <Option<&AudioBuffer> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer);
                __widl_f_set_buffer_AudioBufferSourceNode(self_, buffer)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_playback_rate_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `playbackRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/playbackRate)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn playback_rate(&self) -> AudioParam {
        #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_playback_rate_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_playback_rate_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_playback_rate_AudioBufferSourceNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detune_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `detune` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/detune)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn detune(&self) -> AudioParam {
        #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detune_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detune_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detune_AudioBufferSourceNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loop_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn loop_(&self) -> bool {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loop_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loop_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loop_AudioBufferSourceNode(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_loop_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_loop(&self, loop_: bool) {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_loop_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                loop_: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_loop_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            loop_: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(loop_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let loop_ = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(loop_);
                __widl_f_set_loop_AudioBufferSourceNode(self_, loop_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loop_start_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loopStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn loop_start(&self) -> f64 {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loop_start_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loop_start_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loop_start_AudioBufferSourceNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_loop_start_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loopStart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_loop_start(&self, loop_start: f64) {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_loop_start_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                loop_start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_loop_start_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            loop_start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(loop_start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let loop_start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(loop_start);
                __widl_f_set_loop_start_AudioBufferSourceNode(self_, loop_start)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_loop_end_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loopEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn loop_end(&self) -> f64 {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_loop_end_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_loop_end_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_loop_end_AudioBufferSourceNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_loop_end_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `loopEnd` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_loop_end(&self, loop_end: f64) {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_loop_end_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                loop_end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_loop_end_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            loop_end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(loop_end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let loop_end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(loop_end);
                __widl_f_set_loop_end_AudioBufferSourceNode(self_, loop_end)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_AudioBufferSourceNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_AudioBufferSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioBufferSourceNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioBufferSourceNode {
    #[cfg(all(feature = "AudioBufferSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "AudioBufferSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_AudioBufferSourceNode(
                self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_AudioBufferSourceNode(
            self_: <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&AudioBufferSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_AudioBufferSourceNode(self_, onended)
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
pub static __WASM_BINDGEN_GENERATED_aa2390934c4d6a2d: [u8; 2204usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Z\x08\0\0\0\0\x15\0\0\x02\x15AudioBufferSourceNode'__widl_instanceof_AudioBufferSourceNode\0\0\0\0\"__widl_f_new_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\0\x01\x01\x07context\x03new\0\0\0/__widl_f_new_with_options_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\0\x01\x02\x07context\x07options\x03new\0\0\0$__widl_f_start_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x01\x05self_\x05start\0\0\0.__widl_f_start_with_when_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x02\x05self_\x04when\x05start\0\0\0?__widl_f_start_with_when_and_grain_offset_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x03\x05self_\x04when\x0Cgrain_offset\x05start\0\0\0R__widl_f_start_with_when_and_grain_offset_and_grain_duration_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x04\x05self_\x04when\x0Cgrain_offset\x0Egrain_duration\x05start\0\0\0#__widl_f_stop_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x01\x05self_\x04stop\0\0\0-__widl_f_stop_with_when_AudioBufferSourceNode\x01\0\0\x01\x15AudioBufferSourceNode\x01\0\0\x01\x02\x05self_\x04when\x04stop\0\0\0%__widl_f_buffer_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x06buffer\x01\x01\x05self_\x06buffer\0\0\0)__widl_f_set_buffer_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x02\x06buffer\x01\x02\x05self_\x06buffer\x06buffer\0\0\0,__widl_f_playback_rate_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x0CplaybackRate\x01\x01\x05self_\x0CplaybackRate\0\0\0%__widl_f_detune_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x06detune\x01\x01\x05self_\x06detune\0\0\0#__widl_f_loop_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x04loop\x01\x01\x05self_\x04loop\0\0\0'__widl_f_set_loop_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x02\x04loop\x01\x02\x05self_\x05loop_\x04loop\0\0\0)__widl_f_loop_start_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\tloopStart\x01\x01\x05self_\tloopStart\0\0\0-__widl_f_set_loop_start_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x02\tloopStart\x01\x02\x05self_\nloop_start\tloopStart\0\0\0'__widl_f_loop_end_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x07loopEnd\x01\x01\x05self_\x07loopEnd\0\0\0+__widl_f_set_loop_end_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x02\x07loopEnd\x01\x02\x05self_\x08loop_end\x07loopEnd\0\0\0&__widl_f_onended_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0*__widl_f_set_onended_AudioBufferSourceNode\0\0\0\x01\x15AudioBufferSourceNode\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

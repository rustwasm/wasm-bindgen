use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioNode {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for AudioNode {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioNode {
        #[inline]
        fn from(obj: JsValue) -> AudioNode {
            AudioNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioNode> for AudioNode {
        #[inline]
        fn as_ref(&self) -> &AudioNode {
            self
        }
    }
    impl From<AudioNode> for JsValue {
        #[inline]
        fn from(obj: AudioNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioNode> for EventTarget {
    #[inline]
    fn from(obj: AudioNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_with_audio_node_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <AudioNode as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `connect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn connect_with_audio_node(
        &self,
        destination: &AudioNode,
    ) -> Result<AudioNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_with_audio_node_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_with_audio_node_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(destination);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                __widl_f_connect_with_audio_node_AudioNode(self_, destination)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_with_audio_node_and_output_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <AudioNode as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `connect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn connect_with_audio_node_and_output(
        &self,
        destination: &AudioNode,
        output: u32,
    ) -> Result<AudioNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_with_audio_node_and_output_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_with_audio_node_and_output_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(destination);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_connect_with_audio_node_and_output_AudioNode(self_, destination, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_with_audio_node_and_output_and_input_AudioNode(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <AudioNode as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `connect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn connect_with_audio_node_and_output_and_input(
        &self,
        destination: &AudioNode,
        output: u32,
        input: u32,
    ) -> Result<AudioNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_with_audio_node_and_output_and_input_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_with_audio_node_and_output_and_input_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(destination);
            drop(output);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                let input = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_connect_with_audio_node_and_output_and_input_AudioNode(
                    self_,
                    destination,
                    output,
                    input,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_with_audio_param_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioParam as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `connect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)\n\n*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn connect_with_audio_param(
        &self,
        destination: &AudioParam,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_with_audio_param_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_with_audio_param_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                __widl_f_connect_with_audio_param_AudioNode(self_, destination)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_connect_with_audio_param_and_output_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioParam as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `connect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)\n\n*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn connect_with_audio_param_and_output(
        &self,
        destination: &AudioParam,
        output: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_connect_with_audio_param_and_output_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_connect_with_audio_param_and_output_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_connect_with_audio_param_and_output_AudioNode(self_, destination, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn disconnect(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disconnect_AudioNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_output_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_output(&self, output: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_output_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_output_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_disconnect_with_output_AudioNode(self_, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_audio_node_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_audio_node(
        &self,
        destination: &AudioNode,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_audio_node_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_audio_node_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                __widl_f_disconnect_with_audio_node_AudioNode(self_, destination)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_audio_node_and_output_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_audio_node_and_output(
        &self,
        destination: &AudioNode,
        output: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_audio_node_and_output_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_audio_node_and_output_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_disconnect_with_audio_node_and_output_AudioNode(self_, destination, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_audio_node_and_output_and_input_AudioNode(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_audio_node_and_output_and_input(
        &self,
        destination: &AudioNode,
        output: u32,
        input: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_audio_node_and_output_and_input_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_audio_node_and_output_and_input_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(output);
            drop(input);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                let input = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input);
                __widl_f_disconnect_with_audio_node_and_output_and_input_AudioNode(
                    self_,
                    destination,
                    output,
                    input,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_audio_param_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioParam as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_audio_param(
        &self,
        destination: &AudioParam,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_audio_param_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_audio_param_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                __widl_f_disconnect_with_audio_param_AudioNode(self_, destination)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disconnect_with_audio_param_and_output_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioNode as WasmDescribe>::describe();
    <&AudioParam as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
    #[allow(bad_style)]
    #[doc = "The `disconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)\n\n*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
    #[allow(clippy::all)]
    pub fn disconnect_with_audio_param_and_output(
        &self,
        destination: &AudioParam,
        output: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "AudioParam",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disconnect_with_audio_param_and_output_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disconnect_with_audio_param_and_output_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            destination: <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(destination);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let destination =
                    <&AudioParam as wasm_bindgen::convert::IntoWasmAbi>::into_abi(destination);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_disconnect_with_audio_param_and_output_AudioNode(
                    self_,
                    destination,
                    output,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "BaseAudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_context_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <BaseAudioContext as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "BaseAudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `context` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/context)\n\n*This API requires the following crate features to be activated: `AudioNode`, `BaseAudioContext`*"]
    #[allow(clippy::all)]
    pub fn context(&self) -> BaseAudioContext {
        #[cfg(all(feature = "AudioNode", feature = "BaseAudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_context_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BaseAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_context_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BaseAudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_context_AudioNode(self_)
            };
            <BaseAudioContext as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_of_inputs_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `numberOfInputs` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfInputs)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn number_of_inputs(&self) -> u32 {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_of_inputs_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_of_inputs_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_of_inputs_AudioNode(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_number_of_outputs_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `numberOfOutputs` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfOutputs)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn number_of_outputs(&self) -> u32 {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_number_of_outputs_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_number_of_outputs_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_number_of_outputs_AudioNode(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_channel_count_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `channelCount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn channel_count(&self) -> u32 {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_channel_count_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_channel_count_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_channel_count_AudioNode(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_channel_count_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode",))]
    #[allow(bad_style)]
    #[doc = "The `channelCount` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)\n\n*This API requires the following crate features to be activated: `AudioNode`*"]
    #[allow(clippy::all)]
    pub fn set_channel_count(&self, channel_count: u32) {
        #[cfg(all(feature = "AudioNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_channel_count_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_channel_count_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(channel_count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let channel_count =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(channel_count);
                __widl_f_set_channel_count_AudioNode(self_, channel_count)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_channel_count_mode_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <ChannelCountMode as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
    #[allow(bad_style)]
    #[doc = "The `channelCountMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)\n\n*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
    #[allow(clippy::all)]
    pub fn channel_count_mode(&self) -> ChannelCountMode {
        #[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_channel_count_mode_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelCountMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_channel_count_mode_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelCountMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_channel_count_mode_AudioNode(self_)
            };
            <ChannelCountMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_channel_count_mode_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <ChannelCountMode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
    #[allow(bad_style)]
    #[doc = "The `channelCountMode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)\n\n*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
    #[allow(clippy::all)]
    pub fn set_channel_count_mode(&self, channel_count_mode: ChannelCountMode) {
        #[cfg(all(feature = "AudioNode", feature = "ChannelCountMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_channel_count_mode_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_count_mode: <ChannelCountMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_channel_count_mode_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_count_mode: <ChannelCountMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(channel_count_mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let channel_count_mode =
                    <ChannelCountMode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        channel_count_mode,
                    );
                __widl_f_set_channel_count_mode_AudioNode(self_, channel_count_mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_channel_interpretation_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <ChannelInterpretation as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
    #[allow(bad_style)]
    #[doc = "The `channelInterpretation` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)\n\n*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
    #[allow(clippy::all)]
    pub fn channel_interpretation(&self) -> ChannelInterpretation {
        #[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_channel_interpretation_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelInterpretation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_channel_interpretation_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelInterpretation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_channel_interpretation_AudioNode(self_)
            };
            <ChannelInterpretation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_channel_interpretation_AudioNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <ChannelInterpretation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioNode {
    #[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
    #[allow(bad_style)]
    #[doc = "The `channelInterpretation` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)\n\n*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
    #[allow(clippy::all)]
    pub fn set_channel_interpretation(&self, channel_interpretation: ChannelInterpretation) {
        #[cfg(all(feature = "AudioNode", feature = "ChannelInterpretation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_channel_interpretation_AudioNode(
                self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                channel_interpretation : < ChannelInterpretation as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_channel_interpretation_AudioNode(
            self_: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            channel_interpretation : < ChannelInterpretation as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(channel_interpretation);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let channel_interpretation =
                    <ChannelInterpretation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        channel_interpretation,
                    );
                __widl_f_set_channel_interpretation_AudioNode(self_, channel_interpretation)
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
pub static __WASM_BINDGEN_GENERATED_15023459d6ab6ade: [u8; 2320usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCE\x08\0\0\0\0\x16\0\0\x02\tAudioNode\x1B__widl_instanceof_AudioNode\0\0\0\0*__widl_f_connect_with_audio_node_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x02\x05self_\x0Bdestination\x07connect\0\0\05__widl_f_connect_with_audio_node_and_output_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x03\x05self_\x0Bdestination\x06output\x07connect\0\0\0?__widl_f_connect_with_audio_node_and_output_and_input_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x04\x05self_\x0Bdestination\x06output\x05input\x07connect\0\0\0+__widl_f_connect_with_audio_param_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x02\x05self_\x0Bdestination\x07connect\0\0\06__widl_f_connect_with_audio_param_and_output_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x03\x05self_\x0Bdestination\x06output\x07connect\0\0\0\x1D__widl_f_disconnect_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x01\x05self_\ndisconnect\0\0\0)__widl_f_disconnect_with_output_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x02\x05self_\x06output\ndisconnect\0\0\0-__widl_f_disconnect_with_audio_node_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x02\x05self_\x0Bdestination\ndisconnect\0\0\08__widl_f_disconnect_with_audio_node_and_output_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x03\x05self_\x0Bdestination\x06output\ndisconnect\0\0\0B__widl_f_disconnect_with_audio_node_and_output_and_input_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x04\x05self_\x0Bdestination\x06output\x05input\ndisconnect\0\0\0.__widl_f_disconnect_with_audio_param_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x02\x05self_\x0Bdestination\ndisconnect\0\0\09__widl_f_disconnect_with_audio_param_and_output_AudioNode\x01\0\0\x01\tAudioNode\x01\0\0\x01\x03\x05self_\x0Bdestination\x06output\ndisconnect\0\0\0\x1A__widl_f_context_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x07context\x01\x01\x05self_\x07context\0\0\0#__widl_f_number_of_inputs_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x0EnumberOfInputs\x01\x01\x05self_\x0EnumberOfInputs\0\0\0$__widl_f_number_of_outputs_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x0FnumberOfOutputs\x01\x01\x05self_\x0FnumberOfOutputs\0\0\0 __widl_f_channel_count_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x0CchannelCount\x01\x01\x05self_\x0CchannelCount\0\0\0$__widl_f_set_channel_count_AudioNode\0\0\0\x01\tAudioNode\x01\0\x02\x0CchannelCount\x01\x02\x05self_\rchannel_count\x0CchannelCount\0\0\0%__widl_f_channel_count_mode_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x10channelCountMode\x01\x01\x05self_\x10channelCountMode\0\0\0)__widl_f_set_channel_count_mode_AudioNode\0\0\0\x01\tAudioNode\x01\0\x02\x10channelCountMode\x01\x02\x05self_\x12channel_count_mode\x10channelCountMode\0\0\0)__widl_f_channel_interpretation_AudioNode\0\0\0\x01\tAudioNode\x01\0\x01\x15channelInterpretation\x01\x01\x05self_\x15channelInterpretation\0\0\0-__widl_f_set_channel_interpretation_AudioNode\0\0\0\x01\tAudioNode\x01\0\x02\x15channelInterpretation\x01\x02\x05self_\x16channel_interpretation\x15channelInterpretation\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

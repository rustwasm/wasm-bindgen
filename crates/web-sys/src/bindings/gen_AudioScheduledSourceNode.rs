use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[doc = "The `AudioScheduledSourceNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioScheduledSourceNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioScheduledSourceNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioScheduledSourceNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(24u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(83u32);
            inform(99u32);
            inform(104u32);
            inform(101u32);
            inform(100u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
            inform(100u32);
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
    impl core::ops::Deref for AudioScheduledSourceNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioScheduledSourceNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioScheduledSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioScheduledSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioScheduledSourceNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioScheduledSourceNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioScheduledSourceNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioScheduledSourceNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioScheduledSourceNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioScheduledSourceNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioScheduledSourceNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioScheduledSourceNode {
        #[inline]
        fn from(obj: JsValue) -> AudioScheduledSourceNode {
            AudioScheduledSourceNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioScheduledSourceNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioScheduledSourceNode> for AudioScheduledSourceNode {
        #[inline]
        fn as_ref(&self) -> &AudioScheduledSourceNode {
            self
        }
    }
    impl From<AudioScheduledSourceNode> for JsValue {
        #[inline]
        fn from(obj: AudioScheduledSourceNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioScheduledSourceNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioScheduledSourceNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioScheduledSourceNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioScheduledSourceNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioScheduledSourceNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioScheduledSourceNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioScheduledSourceNode> for AudioNode {
    #[inline]
    fn from(obj: AudioScheduledSourceNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for AudioScheduledSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioScheduledSourceNode> for EventTarget {
    #[inline]
    fn from(obj: AudioScheduledSourceNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioScheduledSourceNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioScheduledSourceNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioScheduledSourceNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioScheduledSourceNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_start_AudioScheduledSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_start_with_when_AudioScheduledSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_stop_AudioScheduledSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_with_when_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_with_when_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_with_when_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_stop_with_when_AudioScheduledSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                __widl_f_onended_AudioScheduledSourceNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioScheduledSourceNode",))]
#[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_AudioScheduledSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioScheduledSourceNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioScheduledSourceNode {
    #[cfg(all(feature = "AudioScheduledSourceNode",))]
    #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)\n\n*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "AudioScheduledSourceNode",))]
        #[deprecated(note = "doesn't exist in Safari, use parent class methods instead")]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_AudioScheduledSourceNode(
                self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_AudioScheduledSourceNode(
            self_: <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&AudioScheduledSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        self,
                    );
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_AudioScheduledSourceNode(self_, onended)
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
pub static __WASM_BINDGEN_GENERATED_5bd8f3e9ab2b8515: [u8; 773usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC3\x02\0\0\0\0\x07\0\0\x02\x18AudioScheduledSourceNode*__widl_instanceof_AudioScheduledSourceNode\0\0\0\0'__widl_f_start_AudioScheduledSourceNode\x01\0\0\x01\x18AudioScheduledSourceNode\x01\0\0\x01\x01\x05self_\x05start\0\0\01__widl_f_start_with_when_AudioScheduledSourceNode\x01\0\0\x01\x18AudioScheduledSourceNode\x01\0\0\x01\x02\x05self_\x04when\x05start\0\0\0&__widl_f_stop_AudioScheduledSourceNode\x01\0\0\x01\x18AudioScheduledSourceNode\x01\0\0\x01\x01\x05self_\x04stop\0\0\00__widl_f_stop_with_when_AudioScheduledSourceNode\x01\0\0\x01\x18AudioScheduledSourceNode\x01\0\0\x01\x02\x05self_\x04when\x04stop\0\0\0)__widl_f_onended_AudioScheduledSourceNode\0\0\0\x01\x18AudioScheduledSourceNode\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0-__widl_f_set_onended_AudioScheduledSourceNode\0\0\0\x01\x18AudioScheduledSourceNode\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

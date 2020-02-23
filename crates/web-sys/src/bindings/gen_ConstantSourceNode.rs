use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ConstantSourceNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ConstantSourceNode {
    obj: AudioScheduledSourceNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ConstantSourceNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ConstantSourceNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(115u32);
            inform(116u32);
            inform(97u32);
            inform(110u32);
            inform(116u32);
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
    impl core::ops::Deref for ConstantSourceNode {
        type Target = AudioScheduledSourceNode;
        #[inline]
        fn deref(&self) -> &AudioScheduledSourceNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for ConstantSourceNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ConstantSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ConstantSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ConstantSourceNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ConstantSourceNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ConstantSourceNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ConstantSourceNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ConstantSourceNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ConstantSourceNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ConstantSourceNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ConstantSourceNode {
        #[inline]
        fn from(obj: JsValue) -> ConstantSourceNode {
            ConstantSourceNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ConstantSourceNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ConstantSourceNode> for ConstantSourceNode {
        #[inline]
        fn as_ref(&self) -> &ConstantSourceNode {
            self
        }
    }
    impl From<ConstantSourceNode> for JsValue {
        #[inline]
        fn from(obj: ConstantSourceNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ConstantSourceNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ConstantSourceNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ConstantSourceNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ConstantSourceNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ConstantSourceNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ConstantSourceNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ConstantSourceNode> for AudioScheduledSourceNode {
    #[inline]
    fn from(obj: ConstantSourceNode) -> AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioScheduledSourceNode> for ConstantSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ConstantSourceNode> for AudioNode {
    #[inline]
    fn from(obj: ConstantSourceNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for ConstantSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ConstantSourceNode> for EventTarget {
    #[inline]
    fn from(obj: ConstantSourceNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ConstantSourceNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ConstantSourceNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: ConstantSourceNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ConstantSourceNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <ConstantSourceNode as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<ConstantSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ConstantSourceNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ConstantSourceNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_ConstantSourceNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "ConstantSourceNode",
    feature = "ConstantSourceOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&ConstantSourceOptions as WasmDescribe>::describe();
    <ConstantSourceNode as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "ConstantSourceNode",
        feature = "ConstantSourceOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`, `ConstantSourceOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConstantSourceOptions,
    ) -> Result<ConstantSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "ConstantSourceNode",
            feature = "ConstantSourceOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_ConstantSourceNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ConstantSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_ConstantSourceNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ConstantSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&ConstantSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_ConstantSourceNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "AudioParam", feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `offset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)\n\n*This API requires the following crate features to be activated: `AudioParam`, `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn offset(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_ConstantSourceNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_ConstantSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_start_with_when_ConstantSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_ConstantSourceNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_with_when_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn stop_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_with_when_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_with_when_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_stop_with_when_ConstantSourceNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_ConstantSourceNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_ConstantSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&ConstantSourceNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl ConstantSourceNode {
    #[cfg(all(feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)\n\n*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_ConstantSourceNode(
                self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_ConstantSourceNode(
            self_: <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&ConstantSourceNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_ConstantSourceNode(self_, onended)
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
pub static __WASM_BINDGEN_GENERATED_6d5b6c08ecfc1ddc: [u8; 942usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}l\x03\0\0\0\0\n\0\0\x02\x12ConstantSourceNode$__widl_instanceof_ConstantSourceNode\0\0\0\0\x1F__widl_f_new_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\0\x01\x01\x07context\x03new\0\0\0,__widl_f_new_with_options_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\0\x01\x02\x07context\x07options\x03new\0\0\0\"__widl_f_offset_ConstantSourceNode\0\0\0\x01\x12ConstantSourceNode\x01\0\x01\x06offset\x01\x01\x05self_\x06offset\0\0\0!__widl_f_start_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\x01\0\0\x01\x01\x05self_\x05start\0\0\0+__widl_f_start_with_when_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\x01\0\0\x01\x02\x05self_\x04when\x05start\0\0\0 __widl_f_stop_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\x01\0\0\x01\x01\x05self_\x04stop\0\0\0*__widl_f_stop_with_when_ConstantSourceNode\x01\0\0\x01\x12ConstantSourceNode\x01\0\0\x01\x02\x05self_\x04when\x04stop\0\0\0#__widl_f_onended_ConstantSourceNode\0\0\0\x01\x12ConstantSourceNode\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0'__widl_f_set_onended_ConstantSourceNode\0\0\0\x01\x12ConstantSourceNode\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

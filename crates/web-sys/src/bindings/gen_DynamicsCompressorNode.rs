use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DynamicsCompressorNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DynamicsCompressorNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DynamicsCompressorNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DynamicsCompressorNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(68u32);
            inform(121u32);
            inform(110u32);
            inform(97u32);
            inform(109u32);
            inform(105u32);
            inform(99u32);
            inform(115u32);
            inform(67u32);
            inform(111u32);
            inform(109u32);
            inform(112u32);
            inform(114u32);
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
    impl core::ops::Deref for DynamicsCompressorNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for DynamicsCompressorNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DynamicsCompressorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DynamicsCompressorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DynamicsCompressorNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DynamicsCompressorNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DynamicsCompressorNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DynamicsCompressorNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DynamicsCompressorNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DynamicsCompressorNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DynamicsCompressorNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DynamicsCompressorNode {
        #[inline]
        fn from(obj: JsValue) -> DynamicsCompressorNode {
            DynamicsCompressorNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DynamicsCompressorNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DynamicsCompressorNode> for DynamicsCompressorNode {
        #[inline]
        fn as_ref(&self) -> &DynamicsCompressorNode {
            self
        }
    }
    impl From<DynamicsCompressorNode> for JsValue {
        #[inline]
        fn from(obj: DynamicsCompressorNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DynamicsCompressorNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DynamicsCompressorNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DynamicsCompressorNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DynamicsCompressorNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DynamicsCompressorNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DynamicsCompressorNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DynamicsCompressorNode> for AudioNode {
    #[inline]
    fn from(obj: DynamicsCompressorNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for DynamicsCompressorNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DynamicsCompressorNode> for EventTarget {
    #[inline]
    fn from(obj: DynamicsCompressorNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for DynamicsCompressorNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DynamicsCompressorNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: DynamicsCompressorNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DynamicsCompressorNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <DynamicsCompressorNode as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &BaseAudioContext,
    ) -> Result<DynamicsCompressorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_DynamicsCompressorNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_DynamicsCompressorNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_DynamicsCompressorNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "DynamicsCompressorNode",
    feature = "DynamicsCompressorOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&DynamicsCompressorOptions as WasmDescribe>::describe();
    <DynamicsCompressorNode as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "DynamicsCompressorNode",
        feature = "DynamicsCompressorOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`, `DynamicsCompressorOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &DynamicsCompressorOptions,
    ) -> Result<DynamicsCompressorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "DynamicsCompressorNode",
            feature = "DynamicsCompressorOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_DynamicsCompressorNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&DynamicsCompressorOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_DynamicsCompressorNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&DynamicsCompressorOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&DynamicsCompressorOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_options_DynamicsCompressorNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_threshold_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `threshold` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn threshold(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_threshold_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_threshold_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_threshold_DynamicsCompressorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_knee_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `knee` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn knee(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_knee_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_knee_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_knee_DynamicsCompressorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ratio_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `ratio` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn ratio(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ratio_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ratio_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ratio_DynamicsCompressorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reduction_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `reduction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)\n\n*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn reduction(&self) -> f32 {
        #[cfg(all(feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reduction_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reduction_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reduction_DynamicsCompressorNode(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_attack_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `attack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn attack(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_attack_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_attack_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_attack_DynamicsCompressorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_release_DynamicsCompressorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DynamicsCompressorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl DynamicsCompressorNode {
    #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `release` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)\n\n*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn release(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_release_DynamicsCompressorNode(
                self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_release_DynamicsCompressorNode(
            self_: <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&DynamicsCompressorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_release_DynamicsCompressorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ca3c6970c9883656: [u8; 932usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}b\x03\0\0\0\0\t\0\0\x02\x16DynamicsCompressorNode(__widl_instanceof_DynamicsCompressorNode\0\0\0\0#__widl_f_new_DynamicsCompressorNode\x01\0\0\x01\x16DynamicsCompressorNode\0\x01\x01\x07context\x03new\0\0\00__widl_f_new_with_options_DynamicsCompressorNode\x01\0\0\x01\x16DynamicsCompressorNode\0\x01\x02\x07context\x07options\x03new\0\0\0)__widl_f_threshold_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\tthreshold\x01\x01\x05self_\tthreshold\0\0\0$__widl_f_knee_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\x04knee\x01\x01\x05self_\x04knee\0\0\0%__widl_f_ratio_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\x05ratio\x01\x01\x05self_\x05ratio\0\0\0)__widl_f_reduction_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\treduction\x01\x01\x05self_\treduction\0\0\0&__widl_f_attack_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\x06attack\x01\x01\x05self_\x06attack\0\0\0'__widl_f_release_DynamicsCompressorNode\0\0\0\x01\x16DynamicsCompressorNode\x01\0\x01\x07release\x01\x01\x05self_\x07release\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

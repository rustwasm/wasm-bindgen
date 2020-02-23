use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GamepadServiceTest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GamepadServiceTest {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GamepadServiceTest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GamepadServiceTest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(18u32);
            inform(71u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(112u32);
            inform(97u32);
            inform(100u32);
            inform(83u32);
            inform(101u32);
            inform(114u32);
            inform(118u32);
            inform(105u32);
            inform(99u32);
            inform(101u32);
            inform(84u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for GamepadServiceTest {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for GamepadServiceTest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GamepadServiceTest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GamepadServiceTest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GamepadServiceTest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GamepadServiceTest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GamepadServiceTest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GamepadServiceTest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GamepadServiceTest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GamepadServiceTest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GamepadServiceTest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GamepadServiceTest {
        #[inline]
        fn from(obj: JsValue) -> GamepadServiceTest {
            GamepadServiceTest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GamepadServiceTest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GamepadServiceTest> for GamepadServiceTest {
        #[inline]
        fn as_ref(&self) -> &GamepadServiceTest {
            self
        }
    }
    impl From<GamepadServiceTest> for JsValue {
        #[inline]
        fn from(obj: GamepadServiceTest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GamepadServiceTest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GamepadServiceTest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GamepadServiceTest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GamepadServiceTest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GamepadServiceTest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GamepadServiceTest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GamepadServiceTest> for ::js_sys::Object {
    #[inline]
    fn from(obj: GamepadServiceTest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GamepadServiceTest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "GamepadHand",
    feature = "GamepadMappingType",
    feature = "GamepadServiceTest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_gamepad_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(7u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <GamepadMappingType as WasmDescribe>::describe();
    <GamepadHand as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(
        feature = "GamepadHand",
        feature = "GamepadMappingType",
        feature = "GamepadServiceTest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `addGamepad()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/addGamepad)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadMappingType`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn add_gamepad(
        &self,
        id: &str,
        mapping: GamepadMappingType,
        hand: GamepadHand,
        num_buttons: u32,
        num_axes: u32,
        num_haptics: u32,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "GamepadHand",
            feature = "GamepadMappingType",
            feature = "GamepadServiceTest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_gamepad_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mapping: <GamepadMappingType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hand: <GamepadHand as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_buttons: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_axes: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                num_haptics: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_gamepad_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mapping: <GamepadMappingType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hand: <GamepadHand as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            num_buttons: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            num_axes: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            num_haptics: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(id);
            drop(mapping);
            drop(hand);
            drop(num_buttons);
            drop(num_axes);
            drop(num_haptics);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                let mapping =
                    <GamepadMappingType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mapping);
                let hand = <GamepadHand as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hand);
                let num_buttons =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_buttons);
                let num_axes = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_axes);
                let num_haptics =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(num_haptics);
                __widl_f_add_gamepad_GamepadServiceTest(
                    self_,
                    id,
                    mapping,
                    hand,
                    num_buttons,
                    num_axes,
                    num_haptics,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_axis_move_event_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `newAxisMoveEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newAxisMoveEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn new_axis_move_event(&self, index: u32, axis: u32, value: f64) {
        #[cfg(all(feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_axis_move_event_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                axis: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_axis_move_event_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            axis: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(axis);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let axis = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(axis);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_new_axis_move_event_GamepadServiceTest(self_, index, axis, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_button_event_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `newButtonEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn new_button_event(&self, index: u32, button: u32, pressed: bool, touched: bool) {
        #[cfg(all(feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_button_event_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                touched: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_button_event_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            touched: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(button);
            drop(pressed);
            drop(touched);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let button = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                let pressed = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pressed);
                let touched = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touched);
                __widl_f_new_button_event_GamepadServiceTest(self_, index, button, pressed, touched)
            };
            ()
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_button_value_event_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `newButtonValueEvent()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newButtonValueEvent)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn new_button_value_event(
        &self,
        index: u32,
        button: u32,
        pressed: bool,
        touched: bool,
        value: f64,
    ) {
        #[cfg(all(feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_button_value_event_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                touched: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_button_value_event_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            button: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pressed: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            touched: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(button);
            drop(pressed);
            drop(touched);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let button = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(button);
                let pressed = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pressed);
                let touched = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(touched);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_new_button_value_event_GamepadServiceTest(
                    self_, index, button, pressed, touched, value,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_pose_move_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(8u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <Option<&mut [f32]> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `newPoseMove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/newPoseMove)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn new_pose_move(
        &self,
        index: u32,
        orient: Option<&mut [f32]>,
        pos: Option<&mut [f32]>,
        ang_velocity: Option<&mut [f32]>,
        ang_acceleration: Option<&mut [f32]>,
        lin_velocity: Option<&mut [f32]>,
        lin_acceleration: Option<&mut [f32]>,
    ) {
        #[cfg(all(feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_pose_move_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                orient: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pos: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ang_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ang_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lin_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lin_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_pose_move_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            orient: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pos: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ang_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ang_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lin_velocity: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lin_acceleration: <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(orient);
            drop(pos);
            drop(ang_velocity);
            drop(ang_acceleration);
            drop(lin_velocity);
            drop(lin_acceleration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let orient =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(orient);
                let pos = <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pos);
                let ang_velocity =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ang_velocity,
                    );
                let ang_acceleration =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ang_acceleration,
                    );
                let lin_velocity =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lin_velocity,
                    );
                let lin_acceleration =
                    <Option<&mut [f32]> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        lin_acceleration,
                    );
                __widl_f_new_pose_move_GamepadServiceTest(
                    self_,
                    index,
                    orient,
                    pos,
                    ang_velocity,
                    ang_acceleration,
                    lin_velocity,
                    lin_acceleration,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_gamepad_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `removeGamepad()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/removeGamepad)\n\n*This API requires the following crate features to be activated: `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn remove_gamepad(&self, index: u32) {
        #[cfg(all(feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_gamepad_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_gamepad_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_gamepad_GamepadServiceTest(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_mapping_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <GamepadMappingType as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `noMapping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noMapping)\n\n*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn no_mapping(&self) -> GamepadMappingType {
        #[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_mapping_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_mapping_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_mapping_GamepadServiceTest(self_)
            };
            <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_standard_mapping_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <GamepadMappingType as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `standardMapping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/standardMapping)\n\n*This API requires the following crate features to be activated: `GamepadMappingType`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn standard_mapping(&self) -> GamepadMappingType {
        #[cfg(all(feature = "GamepadMappingType", feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_standard_mapping_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_standard_mapping_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_standard_mapping_GamepadServiceTest(self_)
            };
            <GamepadMappingType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_hand_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <GamepadHand as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `noHand` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/noHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn no_hand(&self) -> GamepadHand {
        #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_hand_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_hand_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_hand_GamepadServiceTest(self_)
            };
            <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_left_hand_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <GamepadHand as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `leftHand` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/leftHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn left_hand(&self) -> GamepadHand {
        #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_left_hand_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_left_hand_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_left_hand_GamepadServiceTest(self_)
            };
            <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_right_hand_GamepadServiceTest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GamepadServiceTest as WasmDescribe>::describe();
    <GamepadHand as WasmDescribe>::describe();
}
impl GamepadServiceTest {
    #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
    #[allow(bad_style)]
    #[doc = "The `rightHand` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadServiceTest/rightHand)\n\n*This API requires the following crate features to be activated: `GamepadHand`, `GamepadServiceTest`*"]
    #[allow(clippy::all)]
    pub fn right_hand(&self) -> GamepadHand {
        #[cfg(all(feature = "GamepadHand", feature = "GamepadServiceTest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_right_hand_GamepadServiceTest(
                self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_right_hand_GamepadServiceTest(
            self_: <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&GamepadServiceTest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_right_hand_GamepadServiceTest(self_)
            };
            <GamepadHand as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_352e571b679492e8: [u8; 1444usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}b\x05\0\0\0\0\x0C\0\0\x02\x12GamepadServiceTest$__widl_instanceof_GamepadServiceTest\0\0\0\0'__widl_f_add_gamepad_GamepadServiceTest\x01\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x07\x05self_\x02id\x07mapping\x04hand\x0Bnum_buttons\x08num_axes\x0Bnum_haptics\naddGamepad\0\0\0/__widl_f_new_axis_move_event_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x04\x05self_\x05index\x04axis\x05value\x10newAxisMoveEvent\0\0\0,__widl_f_new_button_event_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x05\x05self_\x05index\x06button\x07pressed\x07touched\x0EnewButtonEvent\0\0\02__widl_f_new_button_value_event_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x06\x05self_\x05index\x06button\x07pressed\x07touched\x05value\x13newButtonValueEvent\0\0\0)__widl_f_new_pose_move_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x08\x05self_\x05index\x06orient\x03pos\x0Cang_velocity\x10ang_acceleration\x0Clin_velocity\x10lin_acceleration\x0BnewPoseMove\0\0\0*__widl_f_remove_gamepad_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\0\x01\x02\x05self_\x05index\rremoveGamepad\0\0\0&__widl_f_no_mapping_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\x01\tnoMapping\x01\x01\x05self_\tnoMapping\0\0\0,__widl_f_standard_mapping_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\x01\x0FstandardMapping\x01\x01\x05self_\x0FstandardMapping\0\0\0#__widl_f_no_hand_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\x01\x06noHand\x01\x01\x05self_\x06noHand\0\0\0%__widl_f_left_hand_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\x01\x08leftHand\x01\x01\x05self_\x08leftHand\0\0\0&__widl_f_right_hand_GamepadServiceTest\0\0\0\x01\x12GamepadServiceTest\x01\0\x01\trightHand\x01\x01\x05self_\trightHand\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

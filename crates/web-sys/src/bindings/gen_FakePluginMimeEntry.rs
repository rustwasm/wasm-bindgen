use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `FakePluginMimeEntry` dictionary\n\n\n*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
pub struct FakePluginMimeEntry {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl FakePluginMimeEntry {
    #[doc = "Construct a new `FakePluginMimeEntry`\n\n\n*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn new(type_: &str) -> FakePluginMimeEntry {
        let mut _ret = FakePluginMimeEntry {
            obj: ::js_sys::Object::new(),
        };
        _ret.type_(type_);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `description` field of this object\n\n\n*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("description"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `extension` field of this object\n\n\n*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn extension(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("extension"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `type` field of this object\n\n\n*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("type"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_FakePluginMimeEntry: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<FakePluginMimeEntry> for JsValue {
        #[inline]
        fn from(val: FakePluginMimeEntry) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for FakePluginMimeEntry {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for FakePluginMimeEntry {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for FakePluginMimeEntry {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a FakePluginMimeEntry {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for FakePluginMimeEntry {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            FakePluginMimeEntry {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for FakePluginMimeEntry {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a FakePluginMimeEntry {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for FakePluginMimeEntry {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for FakePluginMimeEntry {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<FakePluginMimeEntry>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(FakePluginMimeEntry {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for FakePluginMimeEntry {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            FakePluginMimeEntry {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const FakePluginMimeEntry) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_26a777c72ad2133a: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

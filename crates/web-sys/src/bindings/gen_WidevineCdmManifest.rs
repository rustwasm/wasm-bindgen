use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `WidevineCDMManifest` dictionary\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
pub struct WidevineCdmManifest {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl WidevineCdmManifest {
    #[doc = "Construct a new `WidevineCDMManifest`\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn new(
        description: &str,
        name: &str,
        version: &str,
        x_cdm_codecs: &str,
        x_cdm_host_versions: &str,
        x_cdm_interface_versions: &str,
        x_cdm_module_versions: &str,
    ) -> WidevineCdmManifest {
        let mut _ret = WidevineCdmManifest {
            obj: ::js_sys::Object::new(),
        };
        _ret.description(description);
        _ret.name(name);
        _ret.version(version);
        _ret.x_cdm_codecs(x_cdm_codecs);
        _ret.x_cdm_host_versions(x_cdm_host_versions);
        _ret.x_cdm_interface_versions(x_cdm_interface_versions);
        _ret.x_cdm_module_versions(x_cdm_module_versions);
        return _ret;
    }
    #[allow(clippy::all)]
    #[doc = "Configure the `description` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
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
    #[doc = "Configure the `name` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("name"),
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
    #[doc = "Configure the `version` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("version"),
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
    #[doc = "Configure the `x-cdm-codecs` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_codecs(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("x-cdm-codecs"),
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
    #[doc = "Configure the `x-cdm-host-versions` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_host_versions(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("x-cdm-host-versions"),
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
    #[doc = "Configure the `x-cdm-interface-versions` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_interface_versions(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("x-cdm-interface-versions"),
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
    #[doc = "Configure the `x-cdm-module-versions` field of this object\n\n\n*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
    pub fn x_cdm_module_versions(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("x-cdm-module-versions"),
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
const _CONST_WidevineCdmManifest: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<WidevineCdmManifest> for JsValue {
        #[inline]
        fn from(val: WidevineCdmManifest) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for WidevineCdmManifest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for WidevineCdmManifest {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for WidevineCdmManifest {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a WidevineCdmManifest {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for WidevineCdmManifest {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            WidevineCdmManifest {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for WidevineCdmManifest {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WidevineCdmManifest {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for WidevineCdmManifest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for WidevineCdmManifest {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<WidevineCdmManifest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(WidevineCdmManifest {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for WidevineCdmManifest {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WidevineCdmManifest {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WidevineCdmManifest) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_b2005b6e191484e8: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};

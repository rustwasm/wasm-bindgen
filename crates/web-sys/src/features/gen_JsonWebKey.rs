#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = JsonWebKey)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `JsonWebKey` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub type JsonWebKey;
    #[wasm_bindgen(method, getter = "alg")]
    fn alg_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "alg")]
    fn set_alg_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "crv")]
    fn crv_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "crv")]
    fn set_crv_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "d")]
    fn d_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "d")]
    fn set_d_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "dp")]
    fn dp_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "dp")]
    fn set_dp_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "dq")]
    fn dq_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "dq")]
    fn set_dq_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "e")]
    fn e_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "e")]
    fn set_e_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "ext")]
    fn ext_shim(this: &JsonWebKey) -> bool;
    #[wasm_bindgen(method, setter = "ext")]
    fn set_ext_shim(this: &JsonWebKey, val: bool);
    #[wasm_bindgen(method, getter = "k")]
    fn k_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "k")]
    fn set_k_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "key_ops")]
    fn key_ops_shim(this: &JsonWebKey) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "key_ops")]
    fn set_key_ops_shim(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "kty")]
    fn kty_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "kty")]
    fn set_kty_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "n")]
    fn n_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "n")]
    fn set_n_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "oth")]
    fn oth_shim(this: &JsonWebKey) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "oth")]
    fn set_oth_shim(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "p")]
    fn p_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "p")]
    fn set_p_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "q")]
    fn q_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "q")]
    fn set_q_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "qi")]
    fn qi_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "qi")]
    fn set_qi_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "use")]
    fn use__shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "use")]
    fn set_use__shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "x")]
    fn x_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "x")]
    fn set_x_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, getter = "y")]
    fn y_shim(this: &JsonWebKey) -> String;
    #[wasm_bindgen(method, setter = "y")]
    fn set_y_shim(this: &JsonWebKey, val: &str);
}
#[doc = "The trait to access properties on the `JsonWebKey` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
pub trait JsonWebKeyGetters {
    #[doc = "Get the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn alg(&self) -> String;
    #[doc = "Get the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn crv(&self) -> String;
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn d(&self) -> String;
    #[doc = "Get the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn dp(&self) -> String;
    #[doc = "Get the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn dq(&self) -> String;
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn e(&self) -> String;
    #[doc = "Get the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn ext(&self) -> bool;
    #[doc = "Get the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn k(&self) -> String;
    #[doc = "Get the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn key_ops(&self) -> ::js_sys::Array;
    #[doc = "Get the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn kty(&self) -> String;
    #[doc = "Get the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn n(&self) -> String;
    #[doc = "Get the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn oth(&self) -> ::js_sys::Array;
    #[doc = "Get the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn p(&self) -> String;
    #[doc = "Get the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn q(&self) -> String;
    #[doc = "Get the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn qi(&self) -> String;
    #[doc = "Get the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn use_(&self) -> String;
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn x(&self) -> String;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn y(&self) -> String;
}
impl JsonWebKeyGetters for JsonWebKey {
    fn alg(&self) -> String {
        self.alg_shim()
    }
    fn crv(&self) -> String {
        self.crv_shim()
    }
    fn d(&self) -> String {
        self.d_shim()
    }
    fn dp(&self) -> String {
        self.dp_shim()
    }
    fn dq(&self) -> String {
        self.dq_shim()
    }
    fn e(&self) -> String {
        self.e_shim()
    }
    fn ext(&self) -> bool {
        self.ext_shim()
    }
    fn k(&self) -> String {
        self.k_shim()
    }
    fn key_ops(&self) -> ::js_sys::Array {
        self.key_ops_shim()
    }
    fn kty(&self) -> String {
        self.kty_shim()
    }
    fn n(&self) -> String {
        self.n_shim()
    }
    fn oth(&self) -> ::js_sys::Array {
        self.oth_shim()
    }
    fn p(&self) -> String {
        self.p_shim()
    }
    fn q(&self) -> String {
        self.q_shim()
    }
    fn qi(&self) -> String {
        self.qi_shim()
    }
    fn use_(&self) -> String {
        self.use__shim()
    }
    fn x(&self) -> String {
        self.x_shim()
    }
    fn y(&self) -> String {
        self.y_shim()
    }
}
impl JsonWebKey {
    #[doc = "Construct a new `JsonWebKey`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn new(kty: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.kty(kty);
        ret
    }
    #[doc = "Change the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn alg(&mut self, val: &str) -> &mut Self {
        self.set_alg_shim(val);
        self
    }
    #[doc = "Change the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn crv(&mut self, val: &str) -> &mut Self {
        self.set_crv_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn d(&mut self, val: &str) -> &mut Self {
        self.set_d_shim(val);
        self
    }
    #[doc = "Change the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dp(&mut self, val: &str) -> &mut Self {
        self.set_dp_shim(val);
        self
    }
    #[doc = "Change the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dq(&mut self, val: &str) -> &mut Self {
        self.set_dq_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn e(&mut self, val: &str) -> &mut Self {
        self.set_e_shim(val);
        self
    }
    #[doc = "Change the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn ext(&mut self, val: bool) -> &mut Self {
        self.set_ext_shim(val);
        self
    }
    #[doc = "Change the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn k(&mut self, val: &str) -> &mut Self {
        self.set_k_shim(val);
        self
    }
    #[doc = "Change the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn key_ops(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_key_ops_shim(val);
        self
    }
    #[doc = "Change the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn kty(&mut self, val: &str) -> &mut Self {
        self.set_kty_shim(val);
        self
    }
    #[doc = "Change the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn n(&mut self, val: &str) -> &mut Self {
        self.set_n_shim(val);
        self
    }
    #[doc = "Change the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn oth(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_oth_shim(val);
        self
    }
    #[doc = "Change the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn p(&mut self, val: &str) -> &mut Self {
        self.set_p_shim(val);
        self
    }
    #[doc = "Change the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn q(&mut self, val: &str) -> &mut Self {
        self.set_q_shim(val);
        self
    }
    #[doc = "Change the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn qi(&mut self, val: &str) -> &mut Self {
        self.set_qi_shim(val);
        self
    }
    #[doc = "Change the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn use_(&mut self, val: &str) -> &mut Self {
        self.set_use__shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn x(&mut self, val: &str) -> &mut Self {
        self.set_x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn y(&mut self, val: &str) -> &mut Self {
        self.set_y_shim(val);
        self
    }
}

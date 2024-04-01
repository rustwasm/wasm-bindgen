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
    #[wasm_bindgen(method, setter = "alg")]
    fn alg_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "crv")]
    fn crv_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "d")]
    fn d_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "dp")]
    fn dp_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "dq")]
    fn dq_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "e")]
    fn e_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "ext")]
    fn ext_shim(this: &JsonWebKey, val: bool);
    #[wasm_bindgen(method, setter = "k")]
    fn k_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "key_ops")]
    fn key_ops_shim(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "kty")]
    fn kty_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "n")]
    fn n_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "oth")]
    fn oth_shim(this: &JsonWebKey, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "p")]
    fn p_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "q")]
    fn q_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "qi")]
    fn qi_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "use")]
    fn use__shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "x")]
    fn x_shim(this: &JsonWebKey, val: &str);
    #[wasm_bindgen(method, setter = "y")]
    fn y_shim(this: &JsonWebKey, val: &str);
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
        self.alg_shim(val);
        self
    }
    #[doc = "Change the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn crv(&mut self, val: &str) -> &mut Self {
        self.crv_shim(val);
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn d(&mut self, val: &str) -> &mut Self {
        self.d_shim(val);
        self
    }
    #[doc = "Change the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dp(&mut self, val: &str) -> &mut Self {
        self.dp_shim(val);
        self
    }
    #[doc = "Change the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dq(&mut self, val: &str) -> &mut Self {
        self.dq_shim(val);
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn e(&mut self, val: &str) -> &mut Self {
        self.e_shim(val);
        self
    }
    #[doc = "Change the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn ext(&mut self, val: bool) -> &mut Self {
        self.ext_shim(val);
        self
    }
    #[doc = "Change the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn k(&mut self, val: &str) -> &mut Self {
        self.k_shim(val);
        self
    }
    #[doc = "Change the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn key_ops(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.key_ops_shim(val);
        self
    }
    #[doc = "Change the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn kty(&mut self, val: &str) -> &mut Self {
        self.kty_shim(val);
        self
    }
    #[doc = "Change the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn n(&mut self, val: &str) -> &mut Self {
        self.n_shim(val);
        self
    }
    #[doc = "Change the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn oth(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.oth_shim(val);
        self
    }
    #[doc = "Change the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn p(&mut self, val: &str) -> &mut Self {
        self.p_shim(val);
        self
    }
    #[doc = "Change the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn q(&mut self, val: &str) -> &mut Self {
        self.q_shim(val);
        self
    }
    #[doc = "Change the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn qi(&mut self, val: &str) -> &mut Self {
        self.qi_shim(val);
        self
    }
    #[doc = "Change the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn use_(&mut self, val: &str) -> &mut Self {
        self.use__shim(val);
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn x(&mut self, val: &str) -> &mut Self {
        self.x_shim(val);
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn y(&mut self, val: &str) -> &mut Self {
        self.y_shim(val);
        self
    }
}

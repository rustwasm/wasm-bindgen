use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = WorkletGlobalScope , extends = :: js_sys :: Object , js_name = PaintWorkletGlobalScope , typescript_type = "PaintWorkletGlobalScope" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `PaintWorkletGlobalScope` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope)
    ///
    ///*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*
    pub type PaintWorkletGlobalScope;

    # [ wasm_bindgen ( method , structural , js_class = "PaintWorkletGlobalScope" , js_name = registerPaint ) ]
    ///The `registerPaint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/registerPaint)
    ///
    ///*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*
    pub fn register_paint(
        this: &PaintWorkletGlobalScope,
        name: &str,
        paint_ctor: &::js_sys::Function,
    );

}

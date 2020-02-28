use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = AnimationTimeline , extends = :: js_sys :: Object , js_name = DocumentTimeline , typescript_name = DocumentTimeline ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentTimeline` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`*"]
    pub type DocumentTimeline;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`*"]
    pub fn new(this: &DocumentTimeline) -> Result<DocumentTimeline, JsValue>;
    #[cfg(feature = "DocumentTimelineOptions")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)\n\n*This API requires the following crate features to be activated: `DocumentTimeline`, `DocumentTimelineOptions`*"]
    pub fn new_with_options(
        this: &DocumentTimeline,
        options: &DocumentTimelineOptions,
    ) -> Result<DocumentTimeline, JsValue>;
}

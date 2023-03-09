use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
interface ITextStyle {
    bold: boolean;
    italic: boolean;
    size: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ITextStyle")]
    pub type ITextStyle;
}

#[wasm_bindgen]
#[derive(Default)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub size: i32,
}

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(_i: ITextStyle) -> TextStyle {
        // parse JsValue
        TextStyle::default()
    }

    pub fn optional_new(_i: Option<ITextStyle>) -> TextStyle {
        // parse JsValue
        TextStyle::default()
    }
}

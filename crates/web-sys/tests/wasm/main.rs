#![feature(use_extern_macros)]
#![cfg(target_arch = "wasm32")]

extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate web_sys;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod anchor_element;
pub mod body_element;
pub mod br_element;
pub mod button_element;
pub mod div_element;
pub mod element;
pub mod head_element;
pub mod headers;
pub mod history;
pub mod html_element;
pub mod html_html_element;
pub mod response;
pub mod script_element;
pub mod span_element;
pub mod style_element;
/*TODO tests for:
    web_sys::HtmlFontElement,
    web_sys::HtmlMenuItemElement,
    web_sys::HtmlSourceElement,
    web_sys::HtmlAreaElement,
    web_sys::HtmlFormElement,
    web_sys::HtmlMetaElement,
    web_sys::HtmlAudioElement,
    web_sys::HtmlFrameElement,
    web_sys::HtmlMeterElement,
    web_sys::HtmlBaseElement,
    web_sys::HtmlFrameSetElement,
    web_sys::HtmlModElement,
    web_sys::HtmlTableCaptionElement,
    web_sys::HtmlObjectElement,
    web_sys::HtmlTableCellElement,
    web_sys::HtmlHeadingElement,
    web_sys::HtmlOListElement,
    web_sys::HtmlTableColElement,
    web_sys::HtmlHRElement,
    web_sys::HtmlOptGroupElement,
    web_sys::HtmlTableElement,
    web_sys::HtmlCanvasElement,
    web_sys::HtmlOptionElement,
    web_sys::HtmlTableRowElement,
    web_sys::HtmlDataElement,
    web_sys::HtmlIFrameElement,
    web_sys::HtmlOutputElement,
    web_sys::HtmlTableSectionElement,
    web_sys::HtmlDataListElement,
    web_sys::HtmlImageElement,
    web_sys::HtmlParagraphElement,
    web_sys::HtmlTemplateElement,
    web_sys::HtmlDetailsElement,
    web_sys::HtmlInputElement,
    web_sys::HtmlParamElement,
    web_sys::HtmlTextAreaElement,
    web_sys::HtmlDialogElement,
    web_sys::HtmlLabelElement,
    web_sys::HtmlPictureElement,
    web_sys::HtmlTimeElement,
    web_sys::HtmlDirectoryElement,
    web_sys::HtmlLegendElement,
    web_sys::HtmlPreElement,
    web_sys::HtmlTitleElement,
    web_sys::HtmlLIElement,
    web_sys::HtmlProgressElement,
    web_sys::HtmlTrackElement,
    web_sys::HtmlDListElement,
    web_sys::HtmlLinkElement,
    web_sys::HtmlQuoteElement,
    web_sys::HtmlUListElement,
    web_sys::HtmlMapElement,
    web_sys::HtmlVideoElement,
    web_sys::HtmlEmbedElement,
    web_sys::HtmlMediaElement,
    web_sys::HtmlSelectElement,
    web_sys::HtmlFieldSetElement,
    web_sys::HtmlMenuElement,
    web_sys::HtmlSlotElement,
*/
pub mod xpath_result;

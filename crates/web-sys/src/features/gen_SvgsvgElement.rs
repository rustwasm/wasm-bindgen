use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSVGElement , typescript_name = SVGSVGElement ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgsvgElement` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub type SvgsvgElement;
    # [ wasm_bindgen ( structural , method , getter , js_name = x ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/x)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    pub fn x(this: &SvgsvgElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = y ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/y)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    pub fn y(this: &SvgsvgElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = width ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/width)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    pub fn width(this: &SvgsvgElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = height ) ]
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/height)\n\n*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
    pub fn height(this: &SvgsvgElement) -> SvgAnimatedLength;
    # [ wasm_bindgen ( structural , method , getter , js_name = useCurrentView ) ]
    #[doc = "Getter for the `useCurrentView` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/useCurrentView)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn use_current_view(this: &SvgsvgElement) -> bool;
    # [ wasm_bindgen ( structural , method , getter , js_name = currentScale ) ]
    #[doc = "Getter for the `currentScale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn current_scale(this: &SvgsvgElement) -> f32;
    # [ wasm_bindgen ( structural , method , setter , js_name = currentScale ) ]
    #[doc = "Setter for the `currentScale` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn set_current_scale(this: &SvgsvgElement, value: f32);
    # [ wasm_bindgen ( structural , method , getter , js_name = currentTranslate ) ]
    #[cfg(feature = "SvgPoint")]
    #[doc = "Getter for the `currentTranslate` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentTranslate)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
    pub fn current_translate(this: &SvgsvgElement) -> SvgPoint;
    # [ wasm_bindgen ( structural , method , getter , js_name = viewBox ) ]
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/viewBox)\n\n*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgsvgElement`*"]
    pub fn view_box(this: &SvgsvgElement) -> SvgAnimatedRect;
    # [ wasm_bindgen ( structural , method , getter , js_name = preserveAspectRatio ) ]
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/preserveAspectRatio)\n\n*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgsvgElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgsvgElement) -> SvgAnimatedPreserveAspectRatio;
    # [ wasm_bindgen ( structural , method , getter , js_name = zoomAndPan ) ]
    #[doc = "Getter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn zoom_and_pan(this: &SvgsvgElement) -> u16;
    # [ wasm_bindgen ( structural , method , setter , js_name = zoomAndPan ) ]
    #[doc = "Setter for the `zoomAndPan` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn set_zoom_and_pan(this: &SvgsvgElement, value: u16);
    # [ wasm_bindgen ( method , structural , js_name = animationsPaused ) ]
    #[doc = "The `animationsPaused()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/animationsPaused)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn animations_paused(this: &SvgsvgElement) -> bool;
    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGAngle ) ]
    #[doc = "The `createSVGAngle()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGAngle)\n\n*This API requires the following crate features to be activated: `SvgAngle`, `SvgsvgElement`*"]
    pub fn create_svg_angle(this: &SvgsvgElement) -> SvgAngle;
    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGLength ) ]
    #[doc = "The `createSVGLength()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGLength)\n\n*This API requires the following crate features to be activated: `SvgLength`, `SvgsvgElement`*"]
    pub fn create_svg_length(this: &SvgsvgElement) -> SvgLength;
    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGMatrix ) ]
    #[doc = "The `createSVGMatrix()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgsvgElement`*"]
    pub fn create_svg_matrix(this: &SvgsvgElement) -> SvgMatrix;
    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGNumber ) ]
    #[doc = "The `createSVGNumber()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGNumber)\n\n*This API requires the following crate features to be activated: `SvgNumber`, `SvgsvgElement`*"]
    pub fn create_svg_number(this: &SvgsvgElement) -> SvgNumber;
    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGPoint ) ]
    #[doc = "The `createSVGPoint()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGPoint)\n\n*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
    pub fn create_svg_point(this: &SvgsvgElement) -> SvgPoint;
    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGRect ) ]
    #[doc = "The `createSVGRect()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGRect)\n\n*This API requires the following crate features to be activated: `SvgRect`, `SvgsvgElement`*"]
    pub fn create_svg_rect(this: &SvgsvgElement) -> SvgRect;
    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( method , structural , js_name = createSVGTransform ) ]
    #[doc = "The `createSVGTransform()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransform)\n\n*This API requires the following crate features to be activated: `SvgTransform`, `SvgsvgElement`*"]
    pub fn create_svg_transform(this: &SvgsvgElement) -> SvgTransform;
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    # [ wasm_bindgen ( method , structural , js_name = createSVGTransformFromMatrix ) ]
    #[doc = "The `createSVGTransformFromMatrix()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)\n\n*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgsvgElement`*"]
    pub fn create_svg_transform_from_matrix(
        this: &SvgsvgElement,
        matrix: &SvgMatrix,
    ) -> SvgTransform;
    # [ wasm_bindgen ( method , structural , js_name = deselectAll ) ]
    #[doc = "The `deselectAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/deselectAll)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn deselect_all(this: &SvgsvgElement);
    # [ wasm_bindgen ( method , structural , js_name = forceRedraw ) ]
    #[doc = "The `forceRedraw()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/forceRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn force_redraw(this: &SvgsvgElement);
    # [ wasm_bindgen ( method , structural , js_name = getCurrentTime ) ]
    #[doc = "The `getCurrentTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn get_current_time(this: &SvgsvgElement) -> f32;
    #[cfg(feature = "Element")]
    # [ wasm_bindgen ( method , structural , js_name = getElementById ) ]
    #[doc = "The `getElementById()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getElementById)\n\n*This API requires the following crate features to be activated: `Element`, `SvgsvgElement`*"]
    pub fn get_element_by_id(this: &SvgsvgElement, element_id: &str) -> Option<Element>;
    # [ wasm_bindgen ( method , structural , js_name = pauseAnimations ) ]
    #[doc = "The `pauseAnimations()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/pauseAnimations)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn pause_animations(this: &SvgsvgElement);
    # [ wasm_bindgen ( method , structural , js_name = setCurrentTime ) ]
    #[doc = "The `setCurrentTime()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/setCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn set_current_time(this: &SvgsvgElement, seconds: f32);
    # [ wasm_bindgen ( method , structural , js_name = suspendRedraw ) ]
    #[doc = "The `suspendRedraw()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/suspendRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn suspend_redraw(this: &SvgsvgElement, max_wait_milliseconds: u32) -> u32;
    # [ wasm_bindgen ( method , structural , js_name = unpauseAnimations ) ]
    #[doc = "The `unpauseAnimations()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unpauseAnimations)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn unpause_animations(this: &SvgsvgElement);
    # [ wasm_bindgen ( method , structural , js_name = unsuspendRedraw ) ]
    #[doc = "The `unsuspendRedraw()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedraw)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn unsuspend_redraw(this: &SvgsvgElement, suspend_handle_id: u32);
    # [ wasm_bindgen ( method , structural , js_name = unsuspendRedrawAll ) ]
    #[doc = "The `unsuspendRedrawAll()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedrawAll)\n\n*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub fn unsuspend_redraw_all(this: &SvgsvgElement);
}
impl SvgsvgElement {
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}

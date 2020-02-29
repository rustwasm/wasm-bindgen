use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = SvgGraphicsElement , extends = SvgElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = SVGSVGElement , typescript_type = "SVGSVGElement" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `SvgsvgElement` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub type SvgsvgElement;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = x ) ]
    ///Getter for the `x` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/x)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*
    pub fn x(this: &SvgsvgElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = y ) ]
    ///Getter for the `y` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/y)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*
    pub fn y(this: &SvgsvgElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = width ) ]
    ///Getter for the `width` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/width)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*
    pub fn width(this: &SvgsvgElement) -> SvgAnimatedLength;

    #[cfg(feature = "SvgAnimatedLength")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = height ) ]
    ///Getter for the `height` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/height)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*
    pub fn height(this: &SvgsvgElement) -> SvgAnimatedLength;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = useCurrentView ) ]
    ///Getter for the `useCurrentView` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/useCurrentView)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn use_current_view(this: &SvgsvgElement) -> bool;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = currentScale ) ]
    ///Getter for the `currentScale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn current_scale(this: &SvgsvgElement) -> f32;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGSVGElement" , js_name = currentScale ) ]
    ///Setter for the `currentScale` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn set_current_scale(this: &SvgsvgElement, value: f32);

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = currentTranslate ) ]
    ///Getter for the `currentTranslate` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentTranslate)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*
    pub fn current_translate(this: &SvgsvgElement) -> SvgPoint;

    #[cfg(feature = "SvgAnimatedRect")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = viewBox ) ]
    ///Getter for the `viewBox` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/viewBox)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgsvgElement`*
    pub fn view_box(this: &SvgsvgElement) -> SvgAnimatedRect;

    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = preserveAspectRatio ) ]
    ///Getter for the `preserveAspectRatio` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/preserveAspectRatio)
    ///
    ///*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgsvgElement`*
    pub fn preserve_aspect_ratio(this: &SvgsvgElement) -> SvgAnimatedPreserveAspectRatio;

    # [ wasm_bindgen ( structural , method , getter , js_class = "SVGSVGElement" , js_name = zoomAndPan ) ]
    ///Getter for the `zoomAndPan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn zoom_and_pan(this: &SvgsvgElement) -> u16;

    # [ wasm_bindgen ( structural , method , setter , js_class = "SVGSVGElement" , js_name = zoomAndPan ) ]
    ///Setter for the `zoomAndPan` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn set_zoom_and_pan(this: &SvgsvgElement, value: u16);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = animationsPaused ) ]
    ///The `animationsPaused()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/animationsPaused)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn animations_paused(this: &SvgsvgElement) -> bool;

    #[cfg(feature = "SvgAngle")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGAngle ) ]
    ///The `createSVGAngle()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGAngle)
    ///
    ///*This API requires the following crate features to be activated: `SvgAngle`, `SvgsvgElement`*
    pub fn create_svg_angle(this: &SvgsvgElement) -> SvgAngle;

    #[cfg(feature = "SvgLength")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGLength ) ]
    ///The `createSVGLength()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGLength)
    ///
    ///*This API requires the following crate features to be activated: `SvgLength`, `SvgsvgElement`*
    pub fn create_svg_length(this: &SvgsvgElement) -> SvgLength;

    #[cfg(feature = "SvgMatrix")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGMatrix ) ]
    ///The `createSVGMatrix()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgsvgElement`*
    pub fn create_svg_matrix(this: &SvgsvgElement) -> SvgMatrix;

    #[cfg(feature = "SvgNumber")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGNumber ) ]
    ///The `createSVGNumber()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGNumber)
    ///
    ///*This API requires the following crate features to be activated: `SvgNumber`, `SvgsvgElement`*
    pub fn create_svg_number(this: &SvgsvgElement) -> SvgNumber;

    #[cfg(feature = "SvgPoint")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGPoint ) ]
    ///The `createSVGPoint()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGPoint)
    ///
    ///*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*
    pub fn create_svg_point(this: &SvgsvgElement) -> SvgPoint;

    #[cfg(feature = "SvgRect")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGRect ) ]
    ///The `createSVGRect()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGRect)
    ///
    ///*This API requires the following crate features to be activated: `SvgRect`, `SvgsvgElement`*
    pub fn create_svg_rect(this: &SvgsvgElement) -> SvgRect;

    #[cfg(feature = "SvgTransform")]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGTransform ) ]
    ///The `createSVGTransform()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransform)
    ///
    ///*This API requires the following crate features to be activated: `SvgTransform`, `SvgsvgElement`*
    pub fn create_svg_transform(this: &SvgsvgElement) -> SvgTransform;

    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = createSVGTransformFromMatrix ) ]
    ///The `createSVGTransformFromMatrix()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)
    ///
    ///*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgsvgElement`*
    pub fn create_svg_transform_from_matrix(
        this: &SvgsvgElement,
        matrix: &SvgMatrix,
    ) -> SvgTransform;

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = deselectAll ) ]
    ///The `deselectAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/deselectAll)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn deselect_all(this: &SvgsvgElement);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = forceRedraw ) ]
    ///The `forceRedraw()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/forceRedraw)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn force_redraw(this: &SvgsvgElement);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = getCurrentTime ) ]
    ///The `getCurrentTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getCurrentTime)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn get_current_time(this: &SvgsvgElement) -> f32;

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = getElementById ) ]
    ///The `getElementById()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getElementById)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn get_element_by_id(this: &SvgsvgElement, element_id: &str) -> Option<Element>;

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = pauseAnimations ) ]
    ///The `pauseAnimations()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/pauseAnimations)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn pause_animations(this: &SvgsvgElement);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = setCurrentTime ) ]
    ///The `setCurrentTime()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/setCurrentTime)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn set_current_time(this: &SvgsvgElement, seconds: f32);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = suspendRedraw ) ]
    ///The `suspendRedraw()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/suspendRedraw)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn suspend_redraw(this: &SvgsvgElement, max_wait_milliseconds: u32) -> u32;

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = unpauseAnimations ) ]
    ///The `unpauseAnimations()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unpauseAnimations)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn unpause_animations(this: &SvgsvgElement);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = unsuspendRedraw ) ]
    ///The `unsuspendRedraw()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedraw)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn unsuspend_redraw(this: &SvgsvgElement, suspend_handle_id: u32);

    # [ wasm_bindgen ( method , structural , js_class = "SVGSVGElement" , js_name = unsuspendRedrawAll ) ]
    ///The `unsuspendRedrawAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedrawAll)
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*
    pub fn unsuspend_redraw_all(this: &SvgsvgElement);

}

impl SvgsvgElement {
    ///The `SVGSVGElement.SVG_ZOOMANDPAN_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*

    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;

    ///The `SVGSVGElement.SVG_ZOOMANDPAN_DISABLE` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*

    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;

    ///The `SVGSVGElement.SVG_ZOOMANDPAN_MAGNIFY` const.
    ///
    ///*This API requires the following crate features to be activated: `SvgsvgElement`*

    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}

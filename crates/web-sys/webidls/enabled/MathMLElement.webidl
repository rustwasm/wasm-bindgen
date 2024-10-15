// https://www.w3.org/TR/mathml-core

[Exposed=Window]
interface MathMLElement : Element { };
// https://www.w3.org/TR/mathml-core/#dom-and-javascript
MathMLElement includes GlobalEventHandlers;
MathMLElement includes HTMLOrSVGElement;
// https://drafts.csswg.org/cssom/#the-elementcssinlinestyle-mixin
MathMLElement includes ElementCSSInlineStyle;
// TODO: Deprecated, add to `GlobalEventHandlers` via mixin:
// https://w3c.github.io/touch-events/#extensions-to-the-globaleventhandlers-mixin
MathMLElement includes TouchEventHandlers;
// TODO: See `OnErrorEventHandlerForNodes` definition in `EventHandler.webidl`.
MathMLElement includes OnErrorEventHandlerForNodes;

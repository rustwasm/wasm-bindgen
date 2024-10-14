/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://mathml-refresh.github.io/mathml-core/
 *
 * Copyright © 2019 W3C® (MIT, ERCIM, Keio, Beihang). W3C liability, trademark
 * and permissive document license rules apply.
 */

[Exposed=Window]
interface MathMLElement : Element { };
// https://www.w3.org/TR/mathml-core/#dom-and-javascript
MathMLElement includes GlobalEventHandlers;
MathMLElement includes HTMLOrSVGElement;
// https://drafts.csswg.org/cssom/#the-elementcssinlinestyle-mixin
MathMLElement includes ElementCSSInlineStyle;
// https://w3c.github.io/touch-events/#extensions-to-the-globaleventhandlers-mixin
MathMLElement includes TouchEventHandlers;
MathMLElement includes OnErrorEventHandlerForNodes;

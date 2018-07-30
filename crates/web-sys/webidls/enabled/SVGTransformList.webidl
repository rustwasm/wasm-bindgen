/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.w3.org/TR/SVG11/
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

interface SVGTransformList {
  readonly attribute unsigned long numberOfItems;
  [Throws]
  void clear();
  [Throws]
  SVGTransform initialize(SVGTransform newItem);
  [Throws]
  getter SVGTransform getItem(unsigned long index);
  [Throws]
  SVGTransform insertItemBefore(SVGTransform newItem, unsigned long index);
  [Throws]
  SVGTransform replaceItem(SVGTransform newItem, unsigned long index);
  [Throws]
  SVGTransform removeItem(unsigned long index);
  [Throws]
  SVGTransform appendItem(SVGTransform newItem);
  SVGTransform createSVGTransformFromMatrix(SVGMatrix matrix);
  [Throws]
  SVGTransform? consolidate();

  // Mozilla-specific stuff
  readonly attribute unsigned long length; // synonym for numberOfItems
};

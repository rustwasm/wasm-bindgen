/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dev.w3.org/fxtf/web-animations/#the-animatable-interface
 *
 * Copyright © 2014 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

dictionary KeyframeAnimationOptions : KeyframeEffectOptions {
  DOMString id = "";
};

dictionary GetAnimationsOptions {
  boolean subtree = false;
};

interface mixin Animatable {
  [Throws]
  Animation animate(object? keyframes,
                    optional UnrestrictedDoubleOrKeyframeAnimationOptions options = {});
  [Func="Document::IsWebAnimationsGetAnimationsEnabled"]
  sequence<Animation> getAnimations(optional GetAnimationsOptions options = {});
};

/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://w3c.github.io/mediacapture-main/getusermedia.html
 */

dictionary MediaTrackSettings {
    long      width;
    long      height;
    double    frameRate;
    DOMString facingMode;
    DOMString deviceId;
    boolean echoCancellation;
    boolean noiseSuppression;
    boolean autoGainControl;
    long      channelCount;

    // Mozilla-specific extensions:

    // http://fluffy.github.io/w3c-screen-share/#screen-based-video-constraints
    // OBE by http://w3c.github.io/mediacapture-screen-share

    DOMString mediaSource;

    // Experimental https://bugzilla.mozilla.org/show_bug.cgi?id=1131568#c3
    //              https://bugzilla.mozilla.org/show_bug.cgi?id=1193075

    long long browserWindow;
    boolean scrollWithPage;
    long viewportOffsetX;
    long viewportOffsetY;
    long viewportWidth;
    long viewportHeight;
};

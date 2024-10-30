/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://w3c.github.io/mediacapture-main/#dom-mediatrackcapabilities
 */

partial interface MediaStreamTrack {
  MediaTrackCapabilities getCapabilities ();
};

dictionary ULongRange {
  [Clamp] unsigned long max;
  [Clamp] unsigned long min;
};

dictionary DoubleRange {
  double max;
  double min;
};

dictionary MediaTrackCapabilities {
  ULongRange width;
  ULongRange height;
  DoubleRange aspectRatio;
  DoubleRange frameRate;
  sequence<DOMString> facingMode;
  sequence<DOMString> resizeMode;
  ULongRange sampleRate;
  ULongRange sampleSize;
  sequence<boolean> echoCancellation;
  sequence<boolean> autoGainControl;
  sequence<boolean> noiseSuppression;
  DoubleRange latency;
  ULongRange channelCount;
  DOMString deviceId;
  DOMString groupId;
  sequence<boolean> backgroundBlur;
};

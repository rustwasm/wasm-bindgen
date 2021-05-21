/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://w3c.github.io/mediacapture-image/
 *
 * Copyright © 2012-2014 W3C® (MIT, ERCIM, Keio, Beihang), All Rights Reserved.
 * W3C liability, trademark and document use rules apply.
 */

[Exposed=Window]
interface ImageCapture {
   constructor(MediaStreamTrack videoTrack);
   Promise<Blob>              takePhoto(optional PhotoSettings photoSettings = {});
   Promise<PhotoCapabilities> getPhotoCapabilities();
   Promise<PhotoSettings>     getPhotoSettings();

   Promise<ImageBitmap>       grabFrame();

   readonly attribute MediaStreamTrack track;
};

dictionary PhotoCapabilities {
  RedEyeReduction         redEyeReduction;
  MediaSettingsRange      imageHeight;
  MediaSettingsRange      imageWidth;
  sequence<FillLightMode> fillLightMode;
};

dictionary PhotoSettings {
  FillLightMode   fillLightMode;
  double          imageHeight;
  double          imageWidth;
  boolean         redEyeReduction;
};

dictionary MediaSettingsRange {
    double max;
    double min;
    double step;
};

enum RedEyeReduction {
  "never",
  "always",
  "controllable"
};

enum FillLightMode {
  "auto",
  "off",
  "flash"
};

partial dictionary MediaTrackSupportedConstraints {
  boolean whiteBalanceMode = true;
  boolean exposureMode = true;
  boolean focusMode = true;
  boolean pointsOfInterest = true;

  boolean exposureCompensation = true;
  boolean exposureTime = true;
  boolean colorTemperature = true;
  boolean iso = true;

  boolean brightness = true;
  boolean contrast = true;
  boolean pan = true;
  boolean saturation = true;
  boolean sharpness = true;
  boolean focusDistance = true;
  boolean tilt = true;
  boolean zoom = true;
  boolean torch = true;
};

partial dictionary MediaTrackCapabilities {
  sequence<DOMString>  whiteBalanceMode;
  sequence<DOMString>  exposureMode;
  sequence<DOMString>  focusMode;

  MediaSettingsRange   exposureCompensation;
  MediaSettingsRange   exposureTime;
  MediaSettingsRange   colorTemperature;
  MediaSettingsRange   iso;

  MediaSettingsRange   brightness;
  MediaSettingsRange   contrast;
  MediaSettingsRange   saturation;
  MediaSettingsRange   sharpness;

  MediaSettingsRange   focusDistance;
  MediaSettingsRange   pan;
  MediaSettingsRange   tilt;
  MediaSettingsRange   zoom;

  boolean              torch;
};

partial dictionary MediaTrackConstraintSet {
  ConstrainDOMString           whiteBalanceMode;
  ConstrainDOMString           exposureMode;
  ConstrainDOMString           focusMode;
  ConstrainPoint2D             pointsOfInterest;

  ConstrainDouble              exposureCompensation;
  ConstrainDouble              exposureTime;
  ConstrainDouble              colorTemperature;
  ConstrainDouble              iso;

  ConstrainDouble              brightness;
  ConstrainDouble              contrast;
  ConstrainDouble              saturation;
  ConstrainDouble              sharpness;

  ConstrainDouble              focusDistance;
  (boolean or ConstrainDouble) pan;
  (boolean or ConstrainDouble) tilt;
  (boolean or ConstrainDouble) zoom;

  ConstrainBoolean             torch;
};

partial dictionary MediaTrackSettings {
  DOMString         whiteBalanceMode;
  DOMString         exposureMode;
  DOMString         focusMode;
  sequence<Point2D> pointsOfInterest;

  double            exposureCompensation;
  double            exposureTime;
  double            colorTemperature;
  double            iso;

  double            brightness;
  double            contrast;
  double            saturation;
  double            sharpness;

  double            focusDistance;
  double            pan;
  double            tilt;
  double            zoom;

  boolean           torch;
};

dictionary ConstrainPoint2DParameters {
  sequence<Point2D> exact;
  sequence<Point2D> ideal;
};

typedef (sequence<Point2D> or ConstrainPoint2DParameters) ConstrainPoint2D;

enum MeteringMode {
  "none",
  "manual",
  "single-shot",
  "continuous"
};

dictionary Point2D {
  double x = 0.0;
  double y = 0.0;
};

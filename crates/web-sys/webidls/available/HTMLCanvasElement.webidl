/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://www.whatwg.org/specs/web-apps/current-work/#the-canvas-element
 * © Copyright 2004-2011 Apple Computer, Inc., Mozilla Foundation, and
 * Opera Software ASA. You are granted a license to use, reproduce
 * and create derivative works of this document.
 */

interface nsISupports;
interface Variant;

[HTMLConstructor]
interface HTMLCanvasElement : HTMLElement {
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long width;
  [CEReactions, Pure, SetterThrows]
           attribute unsigned long height;

  [Throws]
  nsISupports? getContext(DOMString contextId, optional any contextOptions = null);

  [Throws, NeedsSubjectPrincipal]
  DOMString toDataURL(optional DOMString type = "",
                      optional any encoderOptions);
  [Throws, NeedsSubjectPrincipal]
  void toBlob(BlobCallback _callback,
              optional DOMString type = "",
              optional any encoderOptions);
};

// Mozilla specific bits
partial interface HTMLCanvasElement {
  [Pure, SetterThrows]
           attribute boolean mozOpaque;
  [Throws, NeedsSubjectPrincipal]
  File mozGetAsFile(DOMString name, optional DOMString? type = null);
  // A Mozilla-only extension to get a canvas context backed by double-buffered
  // shared memory. Only privileged callers can call this.
  [ChromeOnly, Throws]
  nsISupports? MozGetIPCContext(DOMString contextId);

           attribute PrintCallback? mozPrintCallback;

  [Throws, Pref="canvas.capturestream.enabled", NeedsSubjectPrincipal]
  CanvasCaptureMediaStream captureStream(optional double frameRate);
};

// For OffscreenCanvas
// Reference: https://wiki.whatwg.org/wiki/OffscreenCanvas
partial interface HTMLCanvasElement {
  [Pref="gfx.offscreencanvas.enabled", Throws]
  OffscreenCanvas transferControlToOffscreen();
};

[ChromeOnly]
interface MozCanvasPrintState
{
  // A canvas rendering context.
  readonly attribute nsISupports context;

  // To be called when rendering to the context is done.
  void done();
};

callback PrintCallback = void(MozCanvasPrintState ctx);

callback BlobCallback = void(Blob? blob);

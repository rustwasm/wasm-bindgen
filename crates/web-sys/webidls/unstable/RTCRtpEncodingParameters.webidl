/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* Sources:
 * - https://w3c.github.io/webrtc-extensions/#rtcrtpencodingparameters
 * - https://w3.org/TR/webrtc-svc/#rtcrtpencodingparameters
 */

partial dictionary RTCRtpEncodingParameters {
  unsigned long ptime;
  boolean       adaptivePtime = false;
  RTCRtpCodec   codec;
};

partial dictionary RTCRtpEncodingParameters {
  DOMString scalabilityMode;
};
/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* Source https://w3c.github.io/webrtc-extensions/#rtcrtpencodingparameters
 */

partial dictionary RTCRtpEncodingParameters {
  unsigned long ptime;
  boolean       adaptivePtime = false;
  RTCRtpCodec   codec;
};
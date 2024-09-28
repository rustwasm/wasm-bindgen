/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://www.w3.org/TR/webrtc-encoded-transform/#idl-index
 */

 typedef (SFrameTransform or RTCRtpScriptTransform) RTCRtpTransform;

// New methods for RTCRtpSender and RTCRtpReceiver
partial interface RTCRtpSender {
    attribute RTCRtpTransform? transform;
};

partial interface RTCRtpReceiver {
    attribute RTCRtpTransform? transform;
};

enum SFrameTransformRole {
    "encrypt",
    "decrypt"
};

dictionary SFrameTransformOptions {
    SFrameTransformRole role = "encrypt";
};

typedef [EnforceRange] unsigned long long SmallCryptoKeyID;
typedef (SmallCryptoKeyID or bigint) CryptoKeyID;

[Exposed=(Window,DedicatedWorker)]
interface SFrameTransform : EventTarget {
    constructor(optional SFrameTransformOptions options = {});
    Promise<undefined> setEncryptionKey(CryptoKey key, optional CryptoKeyID keyID);
    attribute EventHandler onerror;
};
SFrameTransform includes GenericTransformStream;

enum SFrameTransformErrorEventType {
    "authentication",
    "keyID",
    "syntax"
};

[Exposed=(Window,DedicatedWorker)]
interface SFrameTransformErrorEvent : Event {
    constructor(DOMString type, SFrameTransformErrorEventInit eventInitDict);

    readonly attribute SFrameTransformErrorEventType errorType;
    readonly attribute CryptoKeyID? keyID;
    readonly attribute any frame;
};

dictionary SFrameTransformErrorEventInit : EventInit {
    required SFrameTransformErrorEventType errorType;
    required any frame;
    CryptoKeyID? keyID;
};

// New enum for video frame types. Will eventually re-use the equivalent defined
// by WebCodecs.
enum RTCEncodedVideoFrameType {
    "empty",
    "key",
    "delta",
};

dictionary RTCEncodedVideoFrameMetadata {
    unsigned long long frameId;
    sequence<unsigned long long> dependencies;
    unsigned short width;
    unsigned short height;
    unsigned long spatialIndex;
    unsigned long temporalIndex;
    unsigned long synchronizationSource;
    octet payloadType;
    sequence<unsigned long> contributingSources;
    long long timestamp;    // microseconds
    unsigned long rtpTimestamp;
    DOMString mimeType;
};

dictionary RTCEncodedVideoFrameOptions {
    RTCEncodedVideoFrameMetadata metadata;
};

// New interfaces to define encoded video and audio frames. Will eventually
// re-use or extend the equivalent defined in WebCodecs.
[Exposed=(Window,DedicatedWorker), Serializable]
interface RTCEncodedVideoFrame {
    constructor(RTCEncodedVideoFrame originalFrame, optional RTCEncodedVideoFrameOptions options = {});
    readonly attribute RTCEncodedVideoFrameType type;
    attribute ArrayBuffer data;
    RTCEncodedVideoFrameMetadata getMetadata();
};

dictionary RTCEncodedAudioFrameMetadata {
    unsigned long synchronizationSource;
    octet payloadType;
    sequence<unsigned long> contributingSources;
    short sequenceNumber;
    unsigned long rtpTimestamp;
    DOMString mimeType;
};

dictionary RTCEncodedAudioFrameOptions {
    RTCEncodedAudioFrameMetadata metadata;
};

[Exposed=(Window,DedicatedWorker), Serializable]
interface RTCEncodedAudioFrame {
    constructor(RTCEncodedAudioFrame originalFrame, optional RTCEncodedAudioFrameOptions options = {});
    attribute ArrayBuffer data;
    RTCEncodedAudioFrameMetadata getMetadata();
};

[Exposed=DedicatedWorker]
interface RTCTransformEvent : Event {
    readonly attribute RTCRtpScriptTransformer transformer;
};

partial interface DedicatedWorkerGlobalScope {
    attribute EventHandler onrtctransform;
};

[Exposed=DedicatedWorker]
interface RTCRtpScriptTransformer : EventTarget {
    // Attributes and methods related to the transformer source
    readonly attribute ReadableStream readable;
    Promise<unsigned long long> generateKeyFrame(optional DOMString rid);
    Promise<undefined> sendKeyFrameRequest();
    // Attributes and methods related to the transformer sink
    readonly attribute WritableStream writable;
    attribute EventHandler onkeyframerequest;
    // Attributes for configuring the Javascript code
    readonly attribute any options;
};

[Exposed=Window]
interface RTCRtpScriptTransform {
    constructor(Worker worker, optional any options, optional sequence<object> transfer);
};

[Exposed=DedicatedWorker]
interface KeyFrameRequestEvent : Event {
  constructor(DOMString type, optional DOMString rid);
  readonly attribute DOMString? rid;
};

partial interface RTCRtpSender {
    Promise<undefined> generateKeyFrame(optional sequence <DOMString> rids);
};

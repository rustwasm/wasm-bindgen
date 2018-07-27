/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * http://dvcs.w3.org/hg/html-media/raw-file/default/media-source/media-source.html
 *
 * Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C
 * liability, trademark and document use rules apply.
 */

enum SourceBufferAppendMode {
    "segments",
    "sequence"
};

[Func="mozilla::dom::MediaSource::Enabled"]
interface SourceBuffer : EventTarget {
  [SetterThrows]
  attribute SourceBufferAppendMode mode;
  readonly attribute boolean updating;
  [Throws]
  readonly attribute TimeRanges buffered;
  [SetterThrows]
  attribute double timestampOffset;
  //readonly attribute AudioTrackList audioTracks;
  //readonly attribute VideoTrackList videoTracks;
  //readonly attribute TextTrackList textTracks;
  [SetterThrows]
  attribute double appendWindowStart;
  [SetterThrows]
  attribute unrestricted double appendWindowEnd;
  attribute EventHandler onupdatestart;
  attribute EventHandler onupdate;
  attribute EventHandler onupdateend;
  attribute EventHandler onerror;
  attribute EventHandler onabort;
  [Throws]
  void appendBuffer(ArrayBuffer data);
  [Throws]
  void appendBuffer(ArrayBufferView data);
  // Experimental function as proposed in:
  // https://github.com/w3c/media-source/issues/100 for promise proposal.
  [Throws, Func="mozilla::dom::MediaSource::ExperimentalEnabled"]
  Promise<void> appendBufferAsync(ArrayBuffer data);
  [Throws, Func="mozilla::dom::MediaSource::ExperimentalEnabled"]
  Promise<void> appendBufferAsync(ArrayBufferView data);
  //[Throws]
  //void appendStream(Stream stream, [EnforceRange] optional unsigned long long maxSize);
  [Throws]
  void abort();
  [Throws]
  void remove(double start, unrestricted double end);
  // Experimental function as proposed in:
  // https://github.com/w3c/media-source/issues/100 for promise proposal.
  [Throws, Func="mozilla::dom::MediaSource::ExperimentalEnabled"]
  Promise<void> removeAsync(double start, unrestricted double end);
  // Experimental function as proposed in:
  // https://github.com/w3c/media-source/issues/155
  [Throws, Func="mozilla::dom::MediaSource::ExperimentalEnabled"]
  void changeType(DOMString type);
};

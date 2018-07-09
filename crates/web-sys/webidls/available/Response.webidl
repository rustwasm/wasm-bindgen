/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is
 * https://fetch.spec.whatwg.org/#response-class
 */

// This should be Constructor(optional BodyInit... but BodyInit doesn't include
// ReadableStream yet because we don't want to expose Streams API to Request.
[Constructor(optional (Blob or BufferSource or FormData or URLSearchParams or ReadableStream or USVString)? body, optional ResponseInit init),
 Exposed=(Window,Worker)]
interface Response {
  [NewObject] static Response error();
  [Throws,
   NewObject] static Response redirect(USVString url, optional unsigned short status = 302);

  readonly attribute ResponseType type;

  readonly attribute USVString url;
  readonly attribute boolean redirected;
  readonly attribute unsigned short status;
  readonly attribute boolean ok;
  readonly attribute ByteString statusText;
  [SameObject] readonly attribute Headers headers;

  [Throws,
   NewObject] Response clone();

  [ChromeOnly, NewObject, Throws] Response cloneUnfiltered();
};
Response implements Body;

// This should be part of Body but we don't want to expose body to request yet.
// See bug 1387483.
partial interface Response {
  [GetterThrows, Func="mozilla::dom::DOMPrefs::StreamsEnabled"]
  readonly attribute ReadableStream? body;
};

dictionary ResponseInit {
  unsigned short status = 200;
  ByteString statusText = "OK";
  HeadersInit headers;
};

enum ResponseType { "basic", "cors", "default", "error", "opaque", "opaqueredirect" };

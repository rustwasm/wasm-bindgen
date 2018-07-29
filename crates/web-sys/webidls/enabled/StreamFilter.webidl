/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/**
 * This is a Mozilla-specific WebExtension API, which is not available to web
 * content. It allows monitoring and filtering of HTTP response stream data.
 *
 * This API should currently be considered experimental, and is not defined by
 * any standard.
 */

enum StreamFilterStatus {
  /**
   * The StreamFilter is not fully initialized. No methods may be called until
   * a "start" event has been received.
   */
  "uninitialized",
  /**
   * The underlying channel is currently transferring data, which will be
   * dispatched via "data" events.
   */
  "transferringdata",
  /**
   * The underlying channel has finished transferring data. Data may still be
   * written via write() calls at this point.
   */
  "finishedtransferringdata",
  /**
   * Data transfer is currently suspended. It may be resumed by a call to
   * resume(). Data may still be written via write() calls in this state.
   */
  "suspended",
  /**
   * The channel has been closed by a call to close(). No further data wlil be
   * delivered via "data" events, and no further data may be written via
   * write() calls.
   */
  "closed",
  /**
   * The channel has been disconnected by a call to disconnect(). All further
   * data will be delivered directly, without passing through the filter. No
   * further events will be dispatched, and no further data may be written by
   * write() calls.
   */
  "disconnected",
  /**
   * An error has occurred and the channel is disconnected. The `error`
   * property contains the details of the error.
   */
  "failed",
};

/**
 * An interface which allows an extension to intercept, and optionally modify,
 * response data from an HTTP request.
 */
[Exposed=(Window,System),
 Func="mozilla::extensions::StreamFilter::IsAllowedInContext"]
interface StreamFilter : EventTarget {
  /**
   * Creates a stream filter for the given add-on and the given extension ID.
   */
  [ChromeOnly]
  static StreamFilter create(unsigned long long requestId, DOMString addonId);

  /**
   * Suspends processing of the request. After this is called, no further data
   * will be delivered until the request is resumed.
   */
  [Throws]
  void suspend();

  /**
   * Resumes delivery of data for a suspended request.
   */
  [Throws]
  void resume();

  /**
   * Closes the request. After this is called, no more data may be written to
   * the stream, and no further data will be delivered.
   *
   * This *must* be called after the consumer is finished writing data, unless
   * disconnect() has already been called.
   */
  [Throws]
  void close();

  /**
   * Disconnects the stream filter from the request. After this is called, no
   * further data will be delivered to the filter, and any unprocessed data
   * will be written directly to the output stream.
   */
  [Throws]
  void disconnect();

  /**
   * Writes a chunk of data to the output stream. This may not be called
   * before the "start" event has been received.
   */
  [Throws]
  void write((ArrayBuffer or Uint8Array) data);

  /**
   * Returns the current status of the stream.
   */
  [Pure]
  readonly attribute StreamFilterStatus status;

  /**
   * After an "error" event has been dispatched, this contains a message
   * describing the error.
   */
  [Pure]
  readonly attribute DOMString error;

  /**
   * Dispatched with a StreamFilterDataEvent whenever incoming data is
   * available on the stream. This data will not be delivered to the output
   * stream unless it is explicitly written via a write() call.
   */
  attribute EventHandler ondata;

  /**
   * Dispatched when the stream is opened, and is about to begin delivering
   * data.
   */
  attribute EventHandler onstart;

  /**
   * Dispatched when the stream has closed, and has no more data to deliver.
   * The output stream remains open and writable until close() is called.
   */
  attribute EventHandler onstop;

  /**
   * Dispatched when an error has occurred. No further data may be read or
   * written after this point.
   */
  attribute EventHandler onerror;
};

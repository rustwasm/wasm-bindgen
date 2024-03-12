/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * API for sharing text, links and other content to an arbitrary destination of the user's choice.
 * W3C Recommendation, 30 May 2023
 * The origin of this IDL file is:
 * https://www.w3.org/TR/web-share
 */

partial interface Navigator {
  [SecureContext] Promise<undefined> share(optional ShareData data = {});
  [SecureContext] boolean canShare(optional ShareData data = {});
};

dictionary ShareData {
  sequence<File> files;
  USVString title;
  USVString text;
  USVString url;
};

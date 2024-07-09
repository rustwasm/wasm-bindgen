/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/*
 * Clipboard API and events
 * Editor’s Draft, 4 June 2024
 *
 * The origin of this IDL file is:
 * https://w3c.github.io/clipboard-apis/#idl-index
 */

// There is no consensus by browsers to implement this API.
// See https://github.com/w3c/clipboard-apis/pull/197.
dictionary ClipboardUnsanitizedFormats {
  sequence<DOMString> unsanitized;
};

// Also Chrome-only.
dictionary ClipboardPermissionDescriptor : PermissionDescriptor {
  boolean allowWithoutGesture = false;
};

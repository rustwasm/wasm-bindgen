/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

[Pref="browser.cache.offline.enable", Func="nsGlobalWindowInner::OfflineCacheAllowedForContext"]
interface OfflineResourceList : EventTarget {
  /**
   * State of the application cache this object is associated with.
   */

  /* This object is not associated with an application cache. */
  const unsigned short UNCACHED = 0;

  /* The application cache is not being updated. */
  const unsigned short IDLE = 1;

  /* The manifest is being fetched and checked for updates */
  const unsigned short CHECKING = 2;

  /* Resources are being downloaded to be added to the cache */
  const unsigned short DOWNLOADING = 3;

  /* There is a new version of the application cache available */
  const unsigned short UPDATEREADY = 4;

  /* The application cache group is now obsolete. */
  const unsigned short OBSOLETE = 5;

  [Throws, UseCounter]
  readonly attribute unsigned short status;

  /**
   * Begin the application update process on the associated application cache.
   */
  [Throws, UseCounter]
  void update();

  /**
   * Swap in the newest version of the application cache, or disassociate
   * from the cache if the cache group is obsolete.
   */
  [Throws, UseCounter]
  void swapCache();

  /* Events */
  [UseCounter]
  attribute EventHandler onchecking;
  [UseCounter]
  attribute EventHandler onerror;
  [UseCounter]
  attribute EventHandler onnoupdate;
  [UseCounter]
  attribute EventHandler ondownloading;
  [UseCounter]
  attribute EventHandler onprogress;
  [UseCounter]
  attribute EventHandler onupdateready;
  [UseCounter]
  attribute EventHandler oncached;
  [UseCounter]
  attribute EventHandler onobsolete;
};

// Mozilla extensions.
partial interface OfflineResourceList {
  /**
   * Get the list of dynamically-managed entries.
   */
  [Throws]
  readonly attribute DOMStringList mozItems;

  /**
   * Check that an entry exists in the list of dynamically-managed entries.
   *
   * @param uri
   *        The resource to check.
   */
  [Throws]
  boolean mozHasItem(DOMString uri);

  /**
   * Get the number of dynamically-managed entries.
   * @status DEPRECATED
   *         Clients should use the "items" attribute.
   */
  [Throws]
  readonly attribute unsigned long mozLength;

  /**
   * Get the URI of a dynamically-managed entry.
   * @status DEPRECATED
   *         Clients should use the "items" attribute.
   */
  [Throws]
  getter DOMString mozItem(unsigned long index);

  /**
   * We need a "length" to actually be valid Web IDL, given that we have an
   * indexed getter.
   */
  readonly attribute unsigned long length;

  /**
   * Add an item to the list of dynamically-managed entries.  The resource
   * will be fetched into the application cache.
   *
   * @param uri
   *        The resource to add.
   */
  [Throws]
  void mozAdd(DOMString uri);

  /**
   * Remove an item from the list of dynamically-managed entries.  If this
   * was the last reference to a URI in the application cache, the cache
   * entry will be removed.
   *
   * @param uri
   *        The resource to remove.
   */
  [Throws]
  void mozRemove(DOMString uri);
};

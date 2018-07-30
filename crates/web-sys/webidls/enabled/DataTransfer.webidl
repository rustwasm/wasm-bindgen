/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * The origin of this IDL file is:
 * http://www.whatwg.org/specs/web-apps/current-work/#the-datatransfer-interface
 */

[Constructor]
interface DataTransfer {
           attribute DOMString dropEffect;
           attribute DOMString effectAllowed;

  readonly attribute DataTransferItemList items;

  void setDragImage(Element image, long x, long y);

  // ReturnValueNeedsContainsHack on .types because lots of extension
  // code was expecting .contains() back when it was a DOMStringList.
  [Pure, Cached, Frozen, NeedsCallerType, ReturnValueNeedsContainsHack]
  readonly attribute sequence<DOMString> types;
  [Throws, NeedsSubjectPrincipal]
  DOMString getData(DOMString format);
  [Throws, NeedsSubjectPrincipal]
  void setData(DOMString format, DOMString data);
  [Throws, NeedsSubjectPrincipal]
  void clearData(optional DOMString format);
  [NeedsSubjectPrincipal]
  readonly attribute FileList? files;
};

partial interface DataTransfer {
  [Throws, Pref="dom.input.dirpicker", NeedsSubjectPrincipal]
  Promise<sequence<(File or Directory)>> getFilesAndDirectories();

  [Throws, Pref="dom.input.dirpicker", NeedsSubjectPrincipal]
  Promise<sequence<File>>                getFiles(optional boolean recursiveFlag = false);
};

// Mozilla specific stuff
partial interface DataTransfer {
  /*
   * Set the drag source. Usually you would not change this, but it will
   * affect which node the drag and dragend events are fired at. The
   * default target is the node that was dragged.
   *
   * @param element drag source to use
   * @throws NO_MODIFICATION_ALLOWED_ERR if the item cannot be modified
   */
  [Throws, UseCounter]
  void addElement(Element element);

  /**
   * The number of items being dragged.
   */
  [UseCounter]
  readonly attribute unsigned long mozItemCount;

  /**
   * Sets the drag cursor state. Primarily used to control the cursor during
   * tab drags, but could be expanded to other uses. XXX Currently implemented
   * on Win32 only.
   *
   * Possible values:
   *  auto - use default system behavior.
   *  default - set the cursor to an arrow during the drag operation.
   *
   * Values other than 'default' are indentical to setting mozCursor to
   * 'auto'.
   */
  [UseCounter]
  attribute DOMString mozCursor;

  /**
   * Holds a list of the format types of the data that is stored for an item
   * at the specified index. If the index is not in the range from 0 to
   * itemCount - 1, an empty string list is returned.
   */
  [Throws, NeedsCallerType, UseCounter]
  DOMStringList mozTypesAt(unsigned long index);

  /**
   * Remove the data associated with the given format for an item at the
   * specified index. The index is in the range from zero to itemCount - 1.
   *
   * If the last format for the item is removed, the entire item is removed,
   * reducing the itemCount by one.
   *
   * If format is empty, then the data associated with all formats is removed.
   * If the format is not found, then this method has no effect.
   *
   * @param format the format to remove
   * @throws NS_ERROR_DOM_INDEX_SIZE_ERR if index is greater or equal than itemCount
   * @throws NO_MODIFICATION_ALLOWED_ERR if the item cannot be modified
   */
  [Throws, NeedsSubjectPrincipal, UseCounter]
  void mozClearDataAt(DOMString format, unsigned long index);

  /*
   * A data transfer may store multiple items, each at a given zero-based
   * index. setDataAt may only be called with an index argument less than
   * itemCount in which case an existing item is modified, or equal to
   * itemCount in which case a new item is added, and the itemCount is
   * incremented by one.
   *
   * Data should be added in order of preference, with the most specific
   * format added first and the least specific format added last. If data of
   * the given format already exists, it is replaced in the same position as
   * the old data.
   *
   * The data should be either a string, a primitive boolean or number type
   * (which will be converted into a string) or an nsISupports.
   *
   * @param format the format to add
   * @param data the data to add
   * @throws NS_ERROR_NULL_POINTER if the data is null
   * @throws NS_ERROR_DOM_INDEX_SIZE_ERR if index is greater than itemCount
   * @throws NO_MODIFICATION_ALLOWED_ERR if the item cannot be modified
   */
  [Throws, NeedsSubjectPrincipal, UseCounter]
  void mozSetDataAt(DOMString format, any data, unsigned long index);

  /**
   * Retrieve the data associated with the given format for an item at the
   * specified index, or null if it does not exist. The index should be in the
   * range from zero to itemCount - 1.
   *
   * @param format the format of the data to look up
   * @returns the data of the given format, or null if it doesn't exist.
   * @throws NS_ERROR_DOM_INDEX_SIZE_ERR if index is greater or equal than itemCount
   */
  [Throws, NeedsSubjectPrincipal, UseCounter]
  any mozGetDataAt(DOMString format, unsigned long index);

  /**
   * Update the drag image. Arguments are the same as setDragImage. This is only
   * valid within the parent chrome process.
   */
  [ChromeOnly]
  void updateDragImage(Element image, long x, long y);

  /**
   * Will be true when the user has cancelled the drag (typically by pressing
   * Escape) and when the drag has been cancelled unexpectedly.  This will be
   * false otherwise, including when the drop has been rejected by its target.
   * This property is only relevant for the dragend event.
   */
  [UseCounter]
  readonly attribute boolean mozUserCancelled;

  /**
   * The node that the mouse was pressed over to begin the drag. For external
   * drags, or if the caller cannot access this node, this will be null.
   */
  [UseCounter]
  readonly attribute Node? mozSourceNode;

  /**
   * The URI spec of the triggering principal.  This may be different than
   * sourceNode's principal when sourceNode is xul:browser and the drag is
   * triggered in a browsing context inside it.
   */
  [ChromeOnly]
  readonly attribute DOMString mozTriggeringPrincipalURISpec;

  /**
   * Copy the given DataTransfer for the given event. Used by testing code for
   * creating emulated Drag and Drop events in the UI.
   *
   * NOTE: Don't expose a DataTransfer produced with this method to the web or
   * use this for non-testing purposes. It can easily be used to get the
   * DataTransfer into an invalid state, and is an unstable implementation
   * detail of EventUtils.synthesizeDrag.
   */
  [Throws, ChromeOnly]
  DataTransfer mozCloneForEvent(DOMString event);
};

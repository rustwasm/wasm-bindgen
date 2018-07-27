/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

[Func="IsChromeOrXBL"]
interface TreeColumns {
  /**
   * The tree widget for these columns.
   */
  readonly attribute TreeBoxObject? tree;

  /**
   * The number of columns.
   */
  readonly attribute unsigned long count;

  /**
   * An alias for count (for the benefit of scripts which treat this as an
   * array).
   */
  readonly attribute unsigned long length;

  /**
   * Get the first/last column.
   */
  TreeColumn? getFirstColumn();
  TreeColumn? getLastColumn();

  /**
   * Attribute based column getters.
   */
  TreeColumn? getPrimaryColumn();
  TreeColumn? getSortedColumn();
  TreeColumn? getKeyColumn();

  /**
   * Get the column for the given element.
   */
  TreeColumn? getColumnFor(Element? element);

  /**
   * Parametric column getters.
   */
  getter TreeColumn? getNamedColumn(DOMString name);
  getter TreeColumn? getColumnAt(unsigned long index);

  /**
   * This method is called whenever a treecol is added or removed and
   * the column cache needs to be rebuilt.
   */
  void invalidateColumns();

  void restoreNaturalOrder();
};

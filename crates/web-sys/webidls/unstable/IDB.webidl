// https://www.w3.org/TR/2023/WD-IndexedDB-3-20231212/

interface mixin IDBDatabase {
  [Throws, NewObject] IDBTransaction transaction((DOMString or sequence<DOMString>) storeNames,
                                         optional IDBTransactionMode mode = "readonly",
                                         optional IDBTransactionOptions options = {});
};

enum IDBTransactionDurability { "default", "strict", "relaxed" };

dictionary IDBTransactionOptions {
  IDBTransactionDurability durability = "default";
};

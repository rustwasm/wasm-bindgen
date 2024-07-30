// https://www.w3.org/TR/IndexedDB-2/

[Exposed=(Window,Worker)]
interface IDBRequest : EventTarget {
  [Throws]
  readonly attribute any result;
  [Throws]
  readonly attribute DOMException? error;
  readonly attribute (IDBObjectStore or IDBIndex or IDBCursor)? source;
  readonly attribute IDBTransaction? transaction;
  readonly attribute IDBRequestReadyState readyState;

  // Event handlers:
  attribute EventHandler onsuccess;
  attribute EventHandler onerror;
};

enum IDBRequestReadyState {
  "pending",
  "done"
};

[Exposed=(Window,Worker)]
interface IDBOpenDBRequest : IDBRequest {
  // Event handlers:
  attribute EventHandler onblocked;
  attribute EventHandler onupgradeneeded;
};

[Exposed=(Window,Worker),
 Constructor(DOMString type, optional IDBVersionChangeEventInit eventInitDict)]
interface IDBVersionChangeEvent : Event {
  readonly attribute unsigned long long oldVersion;
  readonly attribute unsigned long long? newVersion;
};

dictionary IDBVersionChangeEventInit : EventInit {
  unsigned long long oldVersion = 0;
  unsigned long long? newVersion = null;
};

partial interface mixin WindowOrWorkerGlobalScope {
  // Breaking change: Should this throw or be optional?
  [Throws, SameObject] readonly attribute IDBFactory? indexedDB;
};

[Exposed=(Window,Worker)]
interface IDBFactory {
  [Throws, NewObject] IDBOpenDBRequest open(DOMString name,
                                    optional [EnforceRange] unsigned long long version);
  [Throws, NewObject] IDBOpenDBRequest deleteDatabase(DOMString name);

  [Throws]
  short cmp(any first, any second);
};

[Exposed=(Window,Worker)]
interface IDBDatabase : EventTarget {
  readonly attribute DOMString name;
  readonly attribute unsigned long long version;
  readonly attribute DOMStringList objectStoreNames;

  [Throws, NewObject] IDBTransaction transaction((DOMString or sequence<DOMString>) storeNames,
                                         optional IDBTransactionMode mode = "readonly");
  undefined close();

  [Throws, NewObject] IDBObjectStore createObjectStore(DOMString name,
                                               optional IDBObjectStoreParameters options);
  [Throws] undefined deleteObjectStore(DOMString name);

  // Event handlers:
  attribute EventHandler onabort;
  attribute EventHandler onclose;
  attribute EventHandler onerror;
  attribute EventHandler onversionchange;
};

dictionary IDBObjectStoreParameters {
  (DOMString or sequence<DOMString>)? keyPath = null;
  boolean autoIncrement = false;
};

[Exposed=(Window,Worker)]
interface IDBObjectStore {
  [SetterThrows] attribute DOMString name;
  [Throws] readonly attribute any keyPath;
  readonly attribute DOMStringList indexNames;
  [SameObject] readonly attribute IDBTransaction transaction;
  readonly attribute boolean autoIncrement;

  [Throws, NewObject] IDBRequest put(any value, optional any key);
  [Throws, NewObject] IDBRequest add(any value, optional any key);
  [Throws, NewObject] IDBRequest delete(any query);
  [Throws, NewObject] IDBRequest clear();
  [Throws, NewObject] IDBRequest get(any query);
  [Throws, NewObject] IDBRequest getKey(any query);
  [Throws, NewObject] IDBRequest getAll(optional any query,
                                optional [EnforceRange] unsigned long count);
  [Throws, NewObject] IDBRequest getAllKeys(optional any query,
                                    optional [EnforceRange] unsigned long count);
  [Throws, NewObject] IDBRequest count(optional any query);

  [Throws, NewObject] IDBRequest openCursor(optional any query,
                                    optional IDBCursorDirection direction = "next");
  [Throws, NewObject] IDBRequest openKeyCursor(optional any query,
                                       optional IDBCursorDirection direction = "next");

  [Throws] IDBIndex index(DOMString name);

  [Throws, NewObject] IDBIndex createIndex(DOMString name,
                                   (DOMString or sequence<DOMString>) keyPath,
                                   optional IDBIndexParameters options);
  [Throws] undefined deleteIndex(DOMString name);
};

dictionary IDBIndexParameters {
  boolean unique = false;
  boolean multiEntry = false;
};

[Exposed=(Window,Worker)]
interface IDBIndex {
  [SetterThrows] attribute DOMString name;
  [SameObject] readonly attribute IDBObjectStore objectStore;
  [Throws] readonly attribute any keyPath;
  readonly attribute boolean multiEntry;
  readonly attribute boolean unique;

  [Throws, NewObject] IDBRequest get(any query);
  [Throws, NewObject] IDBRequest getKey(any query);
  [Throws, NewObject] IDBRequest getAll(optional any query,
                                optional [EnforceRange] unsigned long count);
  [Throws, NewObject] IDBRequest getAllKeys(optional any query,
                                    optional [EnforceRange] unsigned long count);
  [Throws, NewObject] IDBRequest count(optional any query);

  [Throws, NewObject] IDBRequest openCursor(optional any query,
                                    optional IDBCursorDirection direction = "next");
  [Throws, NewObject] IDBRequest openKeyCursor(optional any query,
                                       optional IDBCursorDirection direction = "next");
};

[Exposed=(Window,Worker)]
interface IDBKeyRange {
  [Throws] readonly attribute any lower;
  [Throws] readonly attribute any upper;
  readonly attribute boolean lowerOpen;
  readonly attribute boolean upperOpen;

  // Static construction methods:
  [Throws, NewObject] static IDBKeyRange only(any value);
  [Throws, NewObject] static IDBKeyRange lowerBound(any lower, optional boolean open = false);
  [Throws, NewObject] static IDBKeyRange upperBound(any upper, optional boolean open = false);
  [Throws, NewObject] static IDBKeyRange bound(any lower,
                                       any upper,
                                       optional boolean lowerOpen = false,
                                       optional boolean upperOpen = false);

  [Throws] boolean _includes(any key);
};

[Exposed=(Window,Worker)]
interface IDBCursor {
  readonly attribute (IDBObjectStore or IDBIndex) source;
  readonly attribute IDBCursorDirection direction;
  [Throws] readonly attribute any key;
  [Throws] readonly attribute any primaryKey;

  [Throws] undefined advance([EnforceRange] unsigned long count);
  [Throws] undefined continue(optional any key);
  [Throws] undefined continuePrimaryKey(any key, any primaryKey);

  [Throws, NewObject] IDBRequest update(any value);
  [Throws, NewObject] IDBRequest delete();
};

enum IDBCursorDirection {
  "next",
  "nextunique",
  "prev",
  "prevunique"
};

[Exposed=(Window,Worker)]
interface IDBCursorWithValue : IDBCursor {
  [Throws] readonly attribute any value;
};

[Exposed=(Window,Worker)]
interface IDBTransaction : EventTarget {
  readonly attribute DOMStringList objectStoreNames;
  [Throws] readonly attribute IDBTransactionMode mode;
  [SameObject] readonly attribute IDBDatabase db;
  // TODO: remove optional?
  readonly attribute DOMException? error;

  [Throws] IDBObjectStore objectStore(DOMString name);
  [Throws] undefined abort();

  // Event handlers:
  attribute EventHandler onabort;
  attribute EventHandler oncomplete;
  attribute EventHandler onerror;
};

enum IDBTransactionMode {
  "readonly",
  "readwrite",
  "versionchange",
  // TODO: deprecated
  "readwriteflush",
  // TODO: deprecated
  "cleanup"
};

/*
** DEPRECATED
*/

partial interface IDBFactory {
  [RustDeprecated, Throws, NewObject]
  IDBOpenDBRequest open(DOMString name, optional  IDBOpenDBOptions options = {});
  [RustDeprecated, Throws, NewObject]
  IDBOpenDBRequest deleteDatabase(DOMString name, optional IDBOpenDBOptions options = {});
};

partial dictionary IDBIndexParameters {
  [RustDeprecated] DOMString? locale = null;
};

partial interface IDBIndex {
  [RustDeprecated] readonly attribute DOMString? locale;
  [RustDeprecated] readonly attribute boolean isAutoLocale;
};

partial interface IDBCursor {
  [RustDeprecated] readonly attribute IDBRequest request;
};

partial interface IDBTransaction {
  [RustDeprecated, Throws] undefined commit();
};

partial interface IDBDatabase {
  [RustDeprecated] readonly attribute StorageType storage;
  [RustDeprecated, Throws] IDBRequest createMutableFile (DOMString name, optional DOMString type);
};

[RustDeprecated] dictionary IDBOpenDBOptions {
  [EnforceRange] unsigned long long version;
  StorageType storage;
};

[RustDeprecated] dictionary IDBFileMetadataParameters {
  boolean size = true;
  boolean lastModified = true;
};

[RustDeprecated] interface IDBFileHandle : EventTarget {
  readonly attribute IDBMutableFile? mutableFile;
  readonly attribute IDBMutableFile? fileHandle;
  readonly attribute FileMode mode;
  readonly attribute boolean active;
  attribute unsigned long long? location;

  [Throws]
  IDBFileRequest? getMetadata(optional IDBFileMetadataParameters parameters = {});
  [Throws]
  IDBFileRequest? readAsArrayBuffer(unsigned long long size);
  [Throws]
  IDBFileRequest? readAsText(unsigned long long size,
                             optional DOMString? encoding = null);

  [Throws]
  IDBFileRequest? write((DOMString or ArrayBuffer or ArrayBufferView or Blob) value);
  [Throws]
  IDBFileRequest? append((DOMString or ArrayBuffer or ArrayBufferView or Blob) value);
  [Throws]
  IDBFileRequest? truncate(optional unsigned long long size);
  [Throws]
  IDBFileRequest? flush();
  [Throws]
  undefined abort();

  attribute EventHandler oncomplete;
  attribute EventHandler onabort;
  attribute EventHandler onerror;
};

[RustDeprecated] interface IDBFileRequest : DOMRequest {
  readonly attribute IDBFileHandle? fileHandle;
  readonly attribute IDBFileHandle? lockedFile;

  attribute EventHandler onprogress;
};

[RustDeprecated] interface IDBLocaleAwareKeyRange : IDBKeyRange {
  [Throws]
  static IDBLocaleAwareKeyRange bound (any lower, any upper, optional boolean lowerOpen = false, optional boolean upperOpen = false);
};

[RustDeprecated] interface IDBMutableFile : EventTarget {
  readonly attribute DOMString name;
  readonly attribute DOMString type;

  readonly attribute IDBDatabase database;

  [Throws] IDBFileHandle open(optional FileMode mode = "readonly");

  [Throws] DOMRequest getFile();

  attribute EventHandler onabort;
  attribute EventHandler onerror;
};

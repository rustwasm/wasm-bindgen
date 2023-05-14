enum FileSystemHandleKind {
  "file",
  "directory",
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemHandle {
  readonly attribute FileSystemHandleKind kind;
  readonly attribute USVString name;

  Promise<boolean> isSameEntry(FileSystemHandle other);
};

dictionary FileSystemCreateWritableOptions {
  boolean keepExistingData = false;
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemFileHandle : FileSystemHandle {
  Promise<File> getFile();
  Promise<FileSystemWritableFileStream> createWritable(optional FileSystemCreateWritableOptions options = {});
  [Exposed=DedicatedWorker]
  Promise<FileSystemSyncAccessHandle> createSyncAccessHandle();
};

dictionary FileSystemGetFileOptions {
  boolean create = false;
};

dictionary FileSystemGetDirectoryOptions {
  boolean create = false;
};

dictionary FileSystemRemoveOptions {
  boolean recursive = false;
};

[Exposed=(Window,Worker), SecureContext, Serializable]
interface FileSystemDirectoryHandle : FileSystemHandle {
  async iterable<USVString, FileSystemHandle>;

  Promise<FileSystemFileHandle> getFileHandle(USVString name, optional FileSystemGetFileOptions options = {});
  Promise<FileSystemDirectoryHandle> getDirectoryHandle(USVString name, optional FileSystemGetDirectoryOptions options = {});

  Promise<undefined> removeEntry(USVString name, optional FileSystemRemoveOptions options = {});

  Promise<sequence<USVString>?> resolve(FileSystemHandle possibleDescendant);
};

enum WriteCommandType {
  "write",
  "seek",
  "truncate",
};

dictionary WriteParams {
  required WriteCommandType type;
  unsigned long long? size;
  unsigned long long? position;
  (BufferSource or Blob or USVString)? data;
};

typedef (BufferSource or Blob or USVString or WriteParams) FileSystemWriteChunkType;

[Exposed=(Window,Worker), SecureContext]
interface FileSystemWritableFileStream : WritableStream {
  [Throws]
  Promise<undefined> write(FileSystemWriteChunkType data);
  [Throws]
  Promise<undefined> seek(unsigned long long position);
  [Throws]
  Promise<undefined> truncate(unsigned long long size);
};

dictionary FileSystemReadWriteOptions {
  [EnforceRange] unsigned long long at;
};

[Exposed=DedicatedWorker, SecureContext]
interface FileSystemSyncAccessHandle {
  [Throws]
  unsigned long long read([AllowShared] BufferSource buffer,
                          optional FileSystemReadWriteOptions options = {});
  [Throws]
  unsigned long long write([AllowShared] BufferSource buffer,
                           optional FileSystemReadWriteOptions options = {});

  [Throws]
  undefined truncate([EnforceRange] unsigned long long newSize);
  [Throws]
  unsigned long long getSize();
  [Throws]
  undefined flush();
  undefined close();
};


[SecureContext]
partial interface StorageManager {
  Promise<FileSystemDirectoryHandle> getDirectory();
};

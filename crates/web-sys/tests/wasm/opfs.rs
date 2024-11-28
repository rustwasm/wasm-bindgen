use std::collections::BTreeMap;

use futures::stream::StreamExt;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{stream::JsStream, JsFuture};
use wasm_bindgen_test::*;
use web_sys::{
    window, File, FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemGetDirectoryOptions,
    FileSystemGetFileOptions, FileSystemWritableFileStream,
};

// aux functions

async fn get_storage() -> FileSystemDirectoryHandle {
    let storage_promise = window().unwrap().navigator().storage().get_directory();
    JsFuture::from(storage_promise)
        .await
        .unwrap()
        .dyn_into::<FileSystemDirectoryHandle>()
        .unwrap()
}

async fn create_directory(
    dir: &FileSystemDirectoryHandle,
    path: &str,
) -> FileSystemDirectoryHandle {
    let mut opts = FileSystemGetDirectoryOptions::new();
    #[allow(deprecated)]
    opts.create(true);
    JsFuture::from(dir.get_directory_handle_with_options(path, &opts))
        .await
        .expect("Couldn't create test directory")
        .dyn_into::<FileSystemDirectoryHandle>()
        .unwrap()
}

async fn create_test_directory(path: &str) -> FileSystemDirectoryHandle {
    let storage = get_storage().await;
    create_directory(&storage, path).await
}

async fn create_file(dir: &FileSystemDirectoryHandle, path: &str) {
    let mut opts = FileSystemGetFileOptions::new();
    #[allow(deprecated)]
    opts.create(true);
    let _ = JsFuture::from(dir.get_file_handle_with_options(path, &opts)).await;
}

// Tests

// Check we can access the root opfs directory
#[wasm_bindgen_test]
async fn access_storage() {
    let storage_promise = window().unwrap().navigator().storage().get_directory();
    let storage = JsFuture::from(storage_promise).await;
    assert!(storage.is_ok());
}

// Creating a directory in the root dir and test it's there
#[wasm_bindgen_test]
async fn test_create_directory() {
    let _test_dir = create_test_directory("create_directory").await;
    let storage = get_storage().await;
    let dir = JsFuture::from(storage.get_directory_handle("create_directory")).await;
    assert!(dir.is_ok());
}

#[wasm_bindgen_test]
async fn test_create_file() {
    let test_dir = create_test_directory("create_file").await;
    create_file(&test_dir, "test.txt").await;
    let stream = JsStream::from(test_dir.keys());
    let mut res = stream.collect::<Vec<Result<JsValue, JsValue>>>().await;
    assert_eq!(res.len(), 1, "Expected number of files");
    let path = res.pop().unwrap().unwrap().dyn_into::<JsString>().unwrap();
    assert_eq!(path, "test.txt", "Searched for file");
}

#[wasm_bindgen_test]
async fn test_write_to_file() {
    let test_dir = create_test_directory("write_to_file").await;

    // create file
    let mut opts = FileSystemGetFileOptions::new();
    #[allow(deprecated)]
    opts.create(true);
    let file = JsFuture::from(test_dir.get_file_handle_with_options("test.txt", &opts))
        .await
        .expect("Couldn't create file")
        .dyn_into::<FileSystemFileHandle>()
        .unwrap();

    // Write to file
    let test_txt = "testing testing";
    let write_stream = JsFuture::from(file.create_writable())
        .await
        .unwrap()
        .dyn_into::<FileSystemWritableFileStream>()
        .unwrap();
    let _ = JsFuture::from(write_stream.write_with_str(test_txt).unwrap()).await;
    JsFuture::from(write_stream.close()).await.unwrap();

    // Read and check contents
    let values = JsStream::from(test_dir.values())
        .map(|x| x.unwrap().dyn_into::<FileSystemFileHandle>().unwrap())
        .collect::<Vec<_>>()
        .await;
    assert_eq!(values.len(), 1);

    let file = JsFuture::from(values[0].get_file())
        .await
        .unwrap()
        .dyn_into::<File>()
        .unwrap();

    let text = JsFuture::from(file.text())
        .await
        .unwrap()
        .dyn_into::<JsString>()
        .unwrap();
    assert_eq!(&text, test_txt);
}

// Create a couple of entries in a directory and check entries returns them all
#[wasm_bindgen_test]
async fn test_entries() {
    let test_dir = create_test_directory("entries").await;
    create_directory(&test_dir, "dir").await;
    create_file(&test_dir, "file").await;
    create_file(&test_dir, "file2").await;

    let entries = JsStream::from(test_dir.entries())
        .map(|x| {
            let array: Vec<JsValue> = x.unwrap().dyn_into::<Array>().unwrap().to_vec();
            assert_eq!(array.len(), 2);
            let path: String = array[0].clone().dyn_into::<JsString>().unwrap().into();
            (path, array[1].clone())
        })
        .collect::<BTreeMap<String, JsValue>>()
        .await;

    assert_eq!(entries.len(), 3);

    match entries.get("dir") {
        Some(handle) => assert!(FileSystemDirectoryHandle::instanceof(handle)),
        _ => panic!("Didn't find directory"),
    }

    match entries.get("file") {
        Some(handle) => assert!(FileSystemFileHandle::instanceof(handle)),
        _ => panic!("Couldn't find file"),
    }

    match entries.get("file2") {
        Some(handle) => assert!(FileSystemFileHandle::instanceof(handle)),
        _ => panic!("Couldn't find file2"),
    }
}

use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub(crate) static BUILTIN_IDENTS: Lazy<BTreeSet<&'static str>> = Lazy::new(|| {
    BTreeSet::from_iter(vec![
        "str",
        "char",
        "bool",
        "JsValue",
        "u8",
        "i8",
        "u16",
        "i16",
        "u32",
        "i32",
        "u64",
        "i64",
        "usize",
        "isize",
        "f32",
        "f64",
        "Result",
        "String",
        "Vec",
        "Option",
        "Array",
        "ArrayBuffer",
        "Object",
        "Promise",
        "Function",
        "Clamped",
        "DataView",
    ])
});

// whitelist a few names that have known polyfills
pub(crate) static POLYFILL_INTERFACES: Lazy<BTreeSet<&'static str>> =
    Lazy::new(|| BTreeSet::from_iter(vec!["AudioContext", "OfflineAudioContext"]));

pub(crate) static IMMUTABLE_SLICE_WHITELIST: Lazy<BTreeSet<&'static str>> = Lazy::new(|| {
    BTreeSet::from_iter(vec![
        // ImageData
        "ImageData",
        // WebGlRenderingContext, WebGl2RenderingContext
        "uniform1fv",
        "uniform2fv",
        "uniform3fv",
        "uniform4fv",
        "uniform1iv",
        "uniform2iv",
        "uniform3iv",
        "uniform4iv",
        "uniformMatrix2fv",
        "uniformMatrix3fv",
        "uniformMatrix4fv",
        "uniformMatrix2x3fv",
        "uniformMatrix2x4fv",
        "uniformMatrix3x2fv",
        "uniformMatrix3x4fv",
        "uniformMatrix4x2fv",
        "uniformMatrix4x3fv",
        "vertexAttrib1fv",
        "vertexAttrib2fv",
        "vertexAttrib3fv",
        "vertexAttrib4fv",
        "bufferData",
        "bufferSubData",
        "texImage2D",
        "texSubImage2D",
        "compressedTexImage2D",
        // WebGl2RenderingContext
        "uniform1uiv",
        "uniform2uiv",
        "uniform3uiv",
        "uniform4uiv",
        "texImage3D",
        "texSubImage3D",
        "compressedTexImage3D",
        "clearBufferfv",
        "clearBufferiv",
        "clearBufferuiv",
        // WebSocket
        "send",
        // WebGPU
        "setBindGroup",
        "writeBuffer",
        "writeTexture",
        // AudioBuffer
        "copyToChannel",
        // FontFace
        "FontFace", // TODO: Add another type's functions here. Leave a comment header with the type name
    ])
});

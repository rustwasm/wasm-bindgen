use crate::descriptor::VectorKind;
use crate::wit::{AuxImport, WasmBindgenAux};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use walrus::{FunctionId, ImportId, TypedCustomSectionId};

#[derive(Default, Debug)]
pub struct NonstandardWitSection {
    /// A list of adapter functions, keyed by their id.
    pub adapters: HashMap<AdapterId, Adapter>,

    /// A list of pairs for adapter functions that implement core wasm imports.
    pub implements: Vec<(ImportId, FunctionId, AdapterId)>,

    /// A list of adapter functions and the names they're exported under.
    pub exports: Vec<(String, AdapterId)>,
}

pub type NonstandardWitSectionId = TypedCustomSectionId<NonstandardWitSection>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct AdapterId(pub usize);

#[derive(Debug, Clone)]
pub struct Adapter {
    pub id: AdapterId,
    pub params: Vec<AdapterType>,
    pub results: Vec<AdapterType>,
    pub inner_results: Vec<AdapterType>,
    pub kind: AdapterKind,
}

#[derive(Debug, Clone)]
pub enum AdapterKind {
    Local {
        instructions: Vec<InstructionData>,
    },
    Import {
        module: String,
        name: String,
        kind: AdapterJsImportKind,
    },
}

#[derive(Debug, Clone)]
pub struct InstructionData {
    pub instr: Instruction,
    pub stack_change: StackChange,
}

#[derive(Debug, Clone)]
pub enum StackChange {
    Modified { pushed: usize, popped: usize },
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AdapterJsImportKind {
    /// The first argument is an `externref` which is the `this` of the function
    /// call
    Method,
    /// The value imported should be invoked as `new`
    Constructor,
    /// A bland function import
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterType {
    S8,
    S16,
    S32,
    S64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Externref,
    Bool,
    I32,
    I64,
    Vector(VectorKind),
    Option(Box<AdapterType>),
    Struct(String),
    Enum(String),
    StringEnum(String),
    NamedExternref(String),
    Function,
    NonNull,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// Calls a function by its id.
    CallCore(walrus::FunctionId),
    /// Call the deallocation function.
    DeferFree {
        free: walrus::FunctionId,
        align: usize,
    },
    /// A call to one of our own defined adapters, similar to the standard
    /// call-adapter instruction
    CallAdapter(AdapterId),
    /// Call an exported function in the core module
    CallExport(walrus::ExportId),
    /// Call an element in the function table of the core module
    CallTableElement(u32),

    /// Gets an argument by its index.
    ArgGet(u32),

    /// An instruction to store `ty` at the `offset` index in the return pointer
    StoreRetptr {
        ty: AdapterType,
        offset: usize,
        mem: walrus::MemoryId,
    },
    /// An instruction to load `ty` at the `offset` index from the return pointer
    LoadRetptr {
        ty: AdapterType,
        offset: usize,
        mem: walrus::MemoryId,
    },
    /// An instruction which pushes the return pointer onto the stack, reserving
    /// `size` bytes of space.
    Retptr {
        size: u32,
    },

    /// Pops a typed integer (`u8`, `s16`, etc.) and pushes a plain wasm `i32` or `i64` equivalent.
    IntToWasm {
        input: AdapterType,
        output: walrus::ValType,
    },
    /// Pops a wasm `i32` or `i64` and pushes a typed integer (`u8`, `s16`, etc.) equivalent.
    WasmToInt {
        input: walrus::ValType,
        output: AdapterType,
    },

    /// Pops a wasm `i32` and pushes the enum variant as a string
    WasmToStringEnum {
        variant_values: Vec<String>,
    },

    OptionWasmToStringEnum {
        variant_values: Vec<String>,
        hole: u32,
    },

    /// pops a string and pushes the enum variant as an `i32`
    StringEnumToWasm {
        variant_values: Vec<String>,
        invalid: u32,
    },

    OptionStringEnumToWasm {
        variant_values: Vec<String>,
        invalid: u32,
        hole: u32,
    },

    /// Pops a `bool` from the stack and pushes an `i32` equivalent
    I32FromBool,
    /// Pops a `string` from the stack and pushes the first character as `i32`
    I32FromStringFirstChar,
    /// Pops an `externref` from the stack, allocates space in the externref table,
    /// returns the index it was stored at.
    I32FromExternrefOwned,
    /// Pops an `externref` from the stack, pushes it onto the externref wasm table
    /// stack, and returns the index it was stored at.
    I32FromExternrefBorrow,
    /// Pops an `externref` from the stack, assumes it's a Rust class given, and
    /// deallocates the JS object and returns the i32 Rust pointer.
    I32FromExternrefRustOwned {
        class: String,
    },
    /// Pops an `externref` from the stack, assumes it's a Rust class given, and
    /// passes the pointer to Rust which will be borrowed for the duration of a
    /// call
    I32FromExternrefRustBorrow {
        class: String,
    },
    /// Pops an `externref` from the stack, pushes 0 if it's "none" or the
    /// consumed pointer value if it's "some".
    I32FromOptionRust {
        class: String,
    },
    /// Pops an `externref` from the stack, pushes either 0 if it's "none" or and
    /// index into the owned wasm table it was stored at if it's "some"
    I32FromOptionExternref {
        /// Set to `Some` by the externref pass of where to put it in the wasm
        /// module, otherwise it's shoved into the JS shim.
        table_and_alloc: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    /// Pops an `externref` from the stack, pushes either a sentinel value if it's
    /// "none" or the integer value of it if it's "some"
    I32FromOptionU32Sentinel,
    /// Pops an `externref` from the stack, pushes 0 for "none", 1 for
    /// "some(false)', and 2 for "some(true)"
    I32FromOptionBool,
    /// Pops an `externref` from the stack, pushes a sentinel for "none" or the
    /// value if it's "some"
    I32FromOptionChar,
    /// Pops an `externref` from the stack, pushes `hole` for "none" or the
    /// value if it's "some"
    I32FromOptionEnum {
        hole: u32,
    },
    /// Pops any externref from the stack and then pushes two values. First is a
    /// 0/1 if it's none/some and second is `ty` value if it was there or 0 if
    /// it wasn't there.
    FromOptionNative {
        ty: walrus::ValType,
    },

    /// Pops a vector value of `kind` from the stack, allocates memory with
    /// `malloc`, and then copies all the data into `mem`. Pushes the pointer
    /// and length as i32.
    VectorToMemory {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
    },
    MutableSliceToMemory {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
    },

    /// Pops a string, pushes pointer/length or all zeros
    OptionString {
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
        realloc: Option<walrus::FunctionId>,
    },
    /// Pops a string, pushes pointer/length
    StringToMemory {
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
        realloc: Option<walrus::FunctionId>,
    },
    /// Pops a pointer + length, pushes a string
    MemoryToString(walrus::MemoryId),

    /// Pops an externref, pushes pointer/length or all zeros
    OptionVector {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
    },

    /// Pops a nullable externref; if it is non-zero, throws it.
    UnwrapResult {
        /// Similar to `I32FromOptionExternref`,
        /// Set to `Some` by the externref pass, and we then take from the externref table. If
        /// None, we use takeObject.
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    UnwrapResultString {
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },

    /// pops a `i32`, pushes `bool`
    BoolFromI32,
    /// pops `i32`, loads externref at that slot, dealloates externref, pushes `externref`
    ExternrefLoadOwned {
        /// This is needed solely for `Result`, since it can contain externrefs,
        /// but has to pass them through a retptr.
        table_and_drop: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    /// pops `i32`, pushes string from that `char`
    StringFromChar,
    /// pops `i32`, pushes an externref for the wrapped rust class
    RustFromI32 {
        class: String,
    },
    OptionRustFromI32 {
        class: String,
    },
    /// pops ptr/length i32, loads string from cache
    CachedStringLoad {
        owned: bool,
        optional: bool,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
        /// If we're in reference-types mode, the externref table ID to get the cached string from.
        table: Option<walrus::TableId>,
    },
    /// pops ptr/length, pushes a vector, frees the original data
    VectorLoad {
        kind: VectorKind,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
    },
    /// pops ptr/length, pushes a vector, frees the original data
    OptionVectorLoad {
        kind: VectorKind,
        mem: walrus::MemoryId,
        free: walrus::FunctionId,
    },
    /// pops i32, loads externref from externref table
    TableGet,
    /// pops two i32 data pointers, pushes an externref closure
    StackClosure {
        adapter: AdapterId,
        nargs: usize,
        mutable: bool,
    },
    /// pops two i32 data pointers, pushes a vector view
    View {
        kind: VectorKind,
        mem: walrus::MemoryId,
    },
    /// pops two i32 data pointers, pushes a vector view
    OptionView {
        kind: VectorKind,
        mem: walrus::MemoryId,
    },
    /// pops i32, pushes it viewed as an optional value with a known sentinel
    OptionU32Sentinel,
    /// pops an i32, then `ty`, then pushes externref
    ToOptionNative {
        ty: walrus::ValType,
        signed: bool,
    },
    OptionBoolFromI32,
    OptionCharFromI32,
    OptionEnumFromI32 {
        hole: u32,
    },
    I32FromOptionNonNull,
    OptionNonNullFromI32,
    I32FromNonNull,
}

impl AdapterType {
    pub fn from_wasm(wasm: walrus::ValType) -> Option<AdapterType> {
        Some(match wasm {
            walrus::ValType::I32 => AdapterType::I32,
            walrus::ValType::I64 => AdapterType::I64,
            walrus::ValType::F32 => AdapterType::F32,
            walrus::ValType::F64 => AdapterType::F64,
            walrus::ValType::Externref => AdapterType::Externref,
            walrus::ValType::Funcref | walrus::ValType::V128 => return None,
        })
    }

    pub fn to_wasm(&self) -> Option<walrus::ValType> {
        Some(match self {
            AdapterType::I32 => walrus::ValType::I32,
            AdapterType::I64 => walrus::ValType::I64,
            AdapterType::F32 => walrus::ValType::F32,
            AdapterType::F64 => walrus::ValType::F64,
            AdapterType::Enum(_) => walrus::ValType::I32,
            AdapterType::Externref | AdapterType::NamedExternref(_) => walrus::ValType::Externref,
            _ => return None,
        })
    }

    pub fn option(self) -> AdapterType {
        AdapterType::Option(Box::new(self))
    }
}

impl NonstandardWitSection {
    pub fn append(
        &mut self,
        params: Vec<AdapterType>,
        results: Vec<AdapterType>,
        inner_results: Vec<AdapterType>,
        kind: AdapterKind,
    ) -> AdapterId {
        let id = AdapterId(self.adapters.len());
        self.adapters.insert(
            id,
            Adapter {
                id,
                params,
                results,
                inner_results,
                kind,
            },
        );
        id
    }

    /// Removes any dead entries in `adapters` that are no longer necessary
    /// and/or no longer referenced.
    ///
    /// Returns `true` if any adapters were deleted, or `false` if the adapters
    /// did not change.
    pub fn gc(&mut self, aux: &WasmBindgenAux) -> bool {
        // Populate the live set with the exports, implements directives, and
        // anything transitively referenced by those adapters.
        let mut live = HashSet::new();
        for (_, id) in self.exports.iter() {
            self.add_live(*id, &mut live);
        }
        for (_, _, id) in self.implements.iter() {
            self.add_live(*id, &mut live);
        }
        for import in aux.import_map.values() {
            if let AuxImport::Closure { adapter, .. } = import {
                self.add_live(*adapter, &mut live);
            }
        }

        // And now that we have the live set we can filter out our list of
        // adapter definitions.
        let before = self.adapters.len();
        self.adapters.retain(|id, _| live.contains(id));
        before != self.adapters.len()
    }

    fn add_live(&self, id: AdapterId, live: &mut HashSet<AdapterId>) {
        if !live.insert(id) {
            return;
        }
        let instructions = match &self.adapters[&id].kind {
            AdapterKind::Local { instructions } => instructions,
            AdapterKind::Import { .. } => return,
        };
        for instr in instructions {
            match instr.instr {
                Instruction::StackClosure { adapter, .. } | Instruction::CallAdapter(adapter) => {
                    self.add_live(adapter, live);
                }
                _ => {}
            }
        }
    }
}

impl walrus::CustomSection for NonstandardWitSection {
    fn name(&self) -> &str {
        "nonstandard wit section"
    }

    fn data(&self, _: &walrus::IdsToIndices) -> Cow<[u8]> {
        panic!("shouldn't emit custom sections just yet");
    }

    fn add_gc_roots(&self, roots: &mut walrus::passes::Roots) {
        use Instruction::*;

        for (_, adapter) in self.adapters.iter() {
            let instrs = match &adapter.kind {
                AdapterKind::Local { instructions } => instructions,
                AdapterKind::Import { .. } => continue,
            };
            for instr in instrs {
                match instr.instr {
                    DeferFree { free: f, .. } | CallCore(f) => {
                        roots.push_func(f);
                    }
                    StoreRetptr { mem, .. }
                    | LoadRetptr { mem, .. }
                    | View { mem, .. }
                    | OptionView { mem, .. }
                    | MemoryToString(mem) => {
                        roots.push_memory(mem);
                    }
                    VectorToMemory { malloc, mem, .. } | OptionVector { malloc, mem, .. } => {
                        roots.push_memory(mem);
                        roots.push_func(malloc);
                    }
                    MutableSliceToMemory { malloc, mem, .. } => {
                        roots.push_memory(mem);
                        roots.push_func(malloc);
                    }
                    VectorLoad { free, mem, .. }
                    | OptionVectorLoad { free, mem, .. }
                    | CachedStringLoad { free, mem, .. } => {
                        roots.push_memory(mem);
                        roots.push_func(free);
                    }
                    OptionString {
                        mem,
                        malloc,
                        realloc,
                    }
                    | StringToMemory {
                        mem,
                        malloc,
                        realloc,
                    } => {
                        roots.push_memory(mem);
                        roots.push_func(malloc);
                        if let Some(id) = realloc {
                            roots.push_func(id);
                        }
                    }
                    I32FromOptionExternref {
                        table_and_alloc: Some((table, alloc)),
                    } => {
                        roots.push_table(table);
                        roots.push_func(alloc);
                    }
                    UnwrapResult { table_and_drop } | UnwrapResultString { table_and_drop } => {
                        if let Some((table, drop)) = table_and_drop {
                            roots.push_table(table);
                            roots.push_func(drop);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

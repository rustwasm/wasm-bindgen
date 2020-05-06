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
    /// The first argument is an `anyref` which is the `this` of the function
    /// call
    Method,
    /// The value imported should be invoked as `new`
    Constructor,
    /// A bland function import
    Normal,
}

#[derive(Debug, Clone)]
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
    Anyref,
    Bool,
    I32,
    I64,
    Vector(VectorKind),
    Option(Box<AdapterType>),
    Struct(String),
    NamedAnyref(String),
    Function,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// A known instruction in the "standard"
    Standard(wit_walrus::Instruction),

    /// A call to one of our own defined adapters, similar to the standard
    /// call-adapter instruction
    CallAdapter(AdapterId),
    /// Call an exported function in the core module
    CallExport(walrus::ExportId),
    /// Call an element in the function table of the core module
    CallTableElement(u32),

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
    /// An instruction which pushes the return pointer onto the stack.
    Retptr,

    /// Pops a `bool` from the stack and pushes an `i32` equivalent
    I32FromBool,
    /// Pops a `string` from the stack and pushes the first character as `i32`
    I32FromStringFirstChar,
    /// Pops an `anyref` from the stack, allocates space in the anyref table,
    /// returns the index it was stored at.
    I32FromAnyrefOwned,
    /// Pops an `anyref` from the stack, pushes it onto the anyref wasm table
    /// stack, and returns the index it was stored at.
    I32FromAnyrefBorrow,
    /// Pops an `anyref` from the stack, assumes it's a Rust class given, and
    /// deallocates the JS object and returns the i32 Rust pointer.
    I32FromAnyrefRustOwned {
        class: String,
    },
    /// Pops an `anyref` from the stack, assumes it's a Rust class given, and
    /// passes the pointer to Rust which will be borrowed for the duration of a
    /// call
    I32FromAnyrefRustBorrow {
        class: String,
    },
    /// Pops an `anyref` from the stack, pushes 0 if it's "none" or the
    /// consumed pointer value if it's "some".
    I32FromOptionRust {
        class: String,
    },
    /// Pops an `s64` or `u64` from the stack, pushing two `i32` values.
    I32Split64 {
        signed: bool,
    },
    /// Pops an `s64` or `u64` from the stack, pushing three `i32` values.
    /// First is the "some/none" bit, and the next is the low bits, and the
    /// next is the high bits.
    I32SplitOption64 {
        signed: bool,
    },
    /// Pops an `anyref` from the stack, pushes either 0 if it's "none" or and
    /// index into the owned wasm table it was stored at if it's "some"
    I32FromOptionAnyref {
        /// Set to `Some` by the anyref pass of where to put it in the wasm
        /// module, otherwise it's shoved into the JS shim.
        table_and_alloc: Option<(walrus::TableId, walrus::FunctionId)>,
    },
    /// Pops an `anyref` from the stack, pushes either a sentinel value if it's
    /// "none" or the integer value of it if it's "some"
    I32FromOptionU32Sentinel,
    /// Pops an `anyref` from the stack, pushes 0 for "none", 1 for
    /// "some(false)', and 2 for "some(true)"
    I32FromOptionBool,
    /// Pops an `anyref` from the stack, pushes a sentinel for "none" or the
    /// value if it's "some"
    I32FromOptionChar,
    /// Pops an `anyref` from the stack, pushes `hole` for "none" or the
    /// value if it's "some"
    I32FromOptionEnum {
        hole: u32,
    },
    /// Pops any anyref from the stack and then pushes two values. First is a
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
        free: walrus::FunctionId,
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

    /// Pops an anyref, pushes pointer/length or all zeros
    OptionVector {
        kind: VectorKind,
        malloc: walrus::FunctionId,
        mem: walrus::MemoryId,
    },

    /// pops a `i32`, pushes `bool`
    BoolFromI32,
    /// pops `i32`, loads anyref at that slot, dealloates anyref, pushes `anyref`
    AnyrefLoadOwned,
    /// pops `i32`, pushes string from that `char`
    StringFromChar,
    /// pops two `i32`, pushes a 64-bit number
    I64FromLoHi {
        signed: bool,
    },
    /// pops `i32`, pushes an anyref for the wrapped rust class
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
    /// pops i32, loads anyref from anyref table
    TableGet,
    /// pops two i32 data pointers, pushes an anyref closure
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
    /// pops an i32, then `ty`, then pushes anyref
    ToOptionNative {
        ty: walrus::ValType,
        signed: bool,
    },
    OptionBoolFromI32,
    OptionCharFromI32,
    OptionEnumFromI32 {
        hole: u32,
    },
    Option64FromI32 {
        signed: bool,
    },
}

impl AdapterType {
    pub fn from_wit(wit: wit_walrus::ValType) -> AdapterType {
        match wit {
            wit_walrus::ValType::S8 => AdapterType::S8,
            wit_walrus::ValType::S16 => AdapterType::S16,
            wit_walrus::ValType::S32 => AdapterType::S32,
            wit_walrus::ValType::S64 => AdapterType::S64,
            wit_walrus::ValType::U8 => AdapterType::U8,
            wit_walrus::ValType::U16 => AdapterType::U16,
            wit_walrus::ValType::U32 => AdapterType::U32,
            wit_walrus::ValType::U64 => AdapterType::U64,
            wit_walrus::ValType::F32 => AdapterType::F32,
            wit_walrus::ValType::F64 => AdapterType::F64,
            wit_walrus::ValType::String => AdapterType::String,
            wit_walrus::ValType::Anyref => AdapterType::Anyref,
            wit_walrus::ValType::I32 => AdapterType::I32,
            wit_walrus::ValType::I64 => AdapterType::I64,
        }
    }

    pub fn from_wasm(wasm: walrus::ValType) -> Option<AdapterType> {
        Some(match wasm {
            walrus::ValType::I32 => AdapterType::I32,
            walrus::ValType::I64 => AdapterType::I64,
            walrus::ValType::F32 => AdapterType::F32,
            walrus::ValType::F64 => AdapterType::F64,
            walrus::ValType::Anyref => AdapterType::Anyref,
            walrus::ValType::Funcref | walrus::ValType::Nullref | walrus::ValType::V128 => {
                return None
            }
        })
    }

    pub fn to_wasm(&self) -> Option<walrus::ValType> {
        Some(match self {
            AdapterType::I32 => walrus::ValType::I32,
            AdapterType::I64 => walrus::ValType::I64,
            AdapterType::F32 => walrus::ValType::F32,
            AdapterType::F64 => walrus::ValType::F64,
            AdapterType::Anyref | AdapterType::NamedAnyref(_) => walrus::ValType::Anyref,
            _ => return None,
        })
    }

    pub fn to_wit(&self) -> Option<wit_walrus::ValType> {
        Some(match self {
            AdapterType::S8 => wit_walrus::ValType::S8,
            AdapterType::S16 => wit_walrus::ValType::S16,
            AdapterType::S32 => wit_walrus::ValType::S32,
            AdapterType::S64 => wit_walrus::ValType::S64,
            AdapterType::U8 => wit_walrus::ValType::U8,
            AdapterType::U16 => wit_walrus::ValType::U16,
            AdapterType::U32 => wit_walrus::ValType::U32,
            AdapterType::U64 => wit_walrus::ValType::U64,
            AdapterType::F32 => wit_walrus::ValType::F32,
            AdapterType::F64 => wit_walrus::ValType::F64,
            AdapterType::String => wit_walrus::ValType::String,
            AdapterType::Anyref | AdapterType::NamedAnyref(_) => wit_walrus::ValType::Anyref,

            AdapterType::I32 => wit_walrus::ValType::I32,
            AdapterType::I64 => wit_walrus::ValType::I64,
            AdapterType::Option(_)
            | AdapterType::Function
            | AdapterType::Struct(_)
            | AdapterType::Bool
            | AdapterType::Vector(_) => return None,
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
        kind: AdapterKind,
    ) -> AdapterId {
        let id = AdapterId(self.adapters.len());
        self.adapters.insert(
            id,
            Adapter {
                id,
                params,
                results,
                kind,
            },
        );
        return id;
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
                    Standard(wit_walrus::Instruction::DeferCallCore(f))
                    | Standard(wit_walrus::Instruction::CallCore(f)) => {
                        roots.push_func(f);
                    }
                    StoreRetptr { mem, .. }
                    | LoadRetptr { mem, .. }
                    | View { mem, .. }
                    | OptionView { mem, .. }
                    | Standard(wit_walrus::Instruction::MemoryToString(mem)) => {
                        roots.push_memory(mem);
                    }
                    VectorToMemory { malloc, mem, .. }
                    | OptionVector { malloc, mem, .. }
                    | Standard(wit_walrus::Instruction::StringToMemory { mem, malloc }) => {
                        roots.push_memory(mem);
                        roots.push_func(malloc);
                    }
                    MutableSliceToMemory {
                        free, malloc, mem, ..
                    } => {
                        roots.push_memory(mem);
                        roots.push_func(malloc);
                        roots.push_func(free);
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
                    I32FromOptionAnyref { table_and_alloc } => {
                        if let Some((table, alloc)) = table_and_alloc {
                            roots.push_table(table);
                            roots.push_func(alloc);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

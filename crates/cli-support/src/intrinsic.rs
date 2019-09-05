//! Definition of all wasm-bindgen intrinsics.
//!
//! This contains a definition of all intrinsics used by `src/lib.rs` in the
//! wasm-bindgen crate. Each intrinsic listed here is part of an `enum
//! Intrinsic` and is generated through a macro to reduce repetition.
//!
//! Intrinsics in this module currently largely contain their expected symbol
//! name as well as the signature of the function that it expects.

use crate::descriptor::{self, Descriptor, Function};

macro_rules! intrinsics {
    (pub enum Intrinsic {
        $(
            #[symbol = $sym:tt]
            #[signature = fn($($arg:expr),*) -> $ret:ident]
            $name:ident,
        )*
    }) => {
        /// All wasm-bindgen intrinsics that could be depended on by a wasm
        /// module.
        #[derive(Debug)]
        pub enum Intrinsic {
            $($name,)*
        }

        impl Intrinsic {
            /// Returns the corresponding intrinsic for a symbol name, if one
            /// matches.
            pub fn from_symbol(symbol: &str) -> Option<Intrinsic> {
                match symbol {
                    $($sym => Some(Intrinsic::$name),)*
                    _ => None,
                }
            }

            /// Returns the expected signature of this intrinsic, used for
            /// generating a JS shim.
            pub fn binding(&self) -> Function {
                use crate::descriptor::Descriptor::*;
                match self {
                    $(
                        Intrinsic::$name => {
                            descriptor::Function {
                                shim_idx: 0,
                                arguments: vec![$($arg),*],
                                ret: $ret,
                            }
                        }
                    )*
                }
            }

            /// Returns the symbol name of this intrinsic
            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Intrinsic::$name => $sym,
                    )*
                }
            }
        }
    };
}

fn ref_anyref() -> Descriptor {
    Descriptor::Ref(Box::new(Descriptor::Anyref))
}

fn ref_string() -> Descriptor {
    Descriptor::Ref(Box::new(Descriptor::String))
}

intrinsics! {
    pub enum Intrinsic {
        #[symbol = "__wbindgen_jsval_eq"]
        #[signature = fn(ref_anyref(), ref_anyref()) -> Boolean]
        JsvalEq,
        #[symbol = "__wbindgen_is_function"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsFunction,
        #[symbol = "__wbindgen_is_undefined"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsUndefined,
        #[symbol = "__wbindgen_is_null"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsNull,
        #[symbol = "__wbindgen_is_object"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsObject,
        #[symbol = "__wbindgen_is_symbol"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsSymbol,
        #[symbol = "__wbindgen_is_string"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsString,
        #[symbol = "__wbindgen_is_falsy"]
        #[signature = fn(ref_anyref()) -> Boolean]
        IsFalsy,
        #[symbol = "__wbindgen_object_clone_ref"]
        #[signature = fn(ref_anyref()) -> Anyref]
        ObjectCloneRef,
        #[symbol = "__wbindgen_object_drop_ref"]
        #[signature = fn(Anyref) -> Unit]
        ObjectDropRef,
        #[symbol = "__wbindgen_cb_drop"]
        #[signature = fn(Anyref) -> Boolean]
        CallbackDrop,
        #[symbol = "__wbindgen_cb_forget"]
        #[signature = fn(Anyref) -> Unit]
        CallbackForget,
        #[symbol = "__wbindgen_number_new"]
        #[signature = fn(F64) -> Anyref]
        NumberNew,
        #[symbol = "__wbindgen_string_new"]
        #[signature = fn(ref_string()) -> Anyref]
        StringNew,
        #[symbol = "__wbindgen_symbol_anonymous_new"]
        #[signature = fn() -> Anyref]
        SymbolAnonymousNew,
        #[symbol = "__wbindgen_symbol_named_new"]
        #[signature = fn(ref_string()) -> Anyref]
        SymbolNamedNew,
        #[symbol = "__wbindgen_number_get"]
        #[signature = fn(ref_anyref(), I32) -> F64]
        NumberGet,
        #[symbol = "__wbindgen_string_get"]
        #[signature = fn(ref_anyref(), I32) -> I32]
        StringGet,
        #[symbol = "__wbindgen_boolean_get"]
        #[signature = fn(ref_anyref()) -> I32]
        BooleanGet,
        #[symbol = "__wbindgen_throw"]
        #[signature = fn(ref_string()) -> Unit]
        Throw,
        #[symbol = "__wbindgen_rethrow"]
        #[signature = fn(Anyref) -> Unit]
        Rethrow,
        #[symbol = "__wbindgen_memory"]
        #[signature = fn() -> Anyref]
        Memory,
        #[symbol = "__wbindgen_module"]
        #[signature = fn() -> Anyref]
        Module,
        #[symbol = "__wbindgen_function_table"]
        #[signature = fn() -> Anyref]
        FunctionTable,
        #[symbol = "__wbindgen_debug_string"]
        #[signature = fn(ref_anyref()) -> String]
        DebugString,
        #[symbol = "__wbindgen_json_parse"]
        #[signature = fn(ref_string()) -> Anyref]
        JsonParse,
        #[symbol = "__wbindgen_json_serialize"]
        #[signature = fn(ref_anyref()) -> String]
        JsonSerialize,
        #[symbol = "__wbindgen_anyref_heap_live_count"]
        #[signature = fn() -> I32]
        AnyrefHeapLiveCount,
        #[symbol = "__wbindgen_init_anyref_table"]
        #[signature = fn() -> Unit]
        InitAnyrefTable,
    }
}

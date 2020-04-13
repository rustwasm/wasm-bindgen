use crate::Diagnostic;
use proc_macro2::{Ident, Span};
use std::hash::{Hash, Hasher};
use syn;
use wasm_bindgen_shared as shared;

/// An abstract syntax tree representing a rust program. Contains
/// extra information for joining up this rust code with javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Default, Clone)]
pub struct Program {
    /// rust -> js interfaces
    pub exports: Vec<Export>,
    /// js -> rust interfaces
    pub imports: Vec<Import>,
    /// rust enums
    pub enums: Vec<Enum>,
    /// rust structs
    pub structs: Vec<Struct>,
    /// custom typescript sections to be included in the definition file
    pub typescript_custom_sections: Vec<String>,
    /// Inline JS snippets
    pub inline_js: Vec<String>,
}

impl Program {
    /// Returns true if the Program is empty
    pub fn is_empty(&self) -> bool {
        self.exports.is_empty()
            && self.imports.is_empty()
            && self.enums.is_empty()
            && self.structs.is_empty()
            && self.typescript_custom_sections.is_empty()
            && self.inline_js.is_empty()
    }
}

/// A rust to js interface. Allows interaction with rust objects/functions
/// from javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct Export {
    /// Comments extracted from the rust source.
    pub comments: Vec<String>,
    /// The rust function
    pub function: Function,
    /// The class name in JS this is attached to
    pub js_class: Option<String>,
    /// The kind (static, named, regular)
    pub method_kind: MethodKind,
    /// The type of `self` (either `self`, `&self`, or `&mut self`)
    pub method_self: Option<MethodSelf>,
    /// The struct name, in Rust, this is attached to
    pub rust_class: Option<Ident>,
    /// The name of the rust function/method on the rust side.
    pub rust_name: Ident,
    /// Whether or not this function should be flagged as the wasm start
    /// function.
    pub start: bool,
}

/// The 3 types variations of `self`.
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum MethodSelf {
    /// `self`
    ByValue,
    /// `&mut self`
    RefMutable,
    /// `&self`
    RefShared,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct Import {
    pub module: ImportModule,
    pub js_namespace: Option<Ident>,
    pub kind: ImportKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub enum ImportModule {
    None,
    Named(String, Span),
    RawNamed(String, Span),
    Inline(usize, Span),
}

impl Hash for ImportModule {
    fn hash<H: Hasher>(&self, h: &mut H) {
        match self {
            ImportModule::None => {
                0u8.hash(h);
            }
            ImportModule::Named(name, _) => {
                1u8.hash(h);
                name.hash(h);
            }
            ImportModule::Inline(idx, _) => {
                2u8.hash(h);
                idx.hash(h);
            }
            ImportModule::RawNamed(name, _) => {
                3u8.hash(h);
                name.hash(h);
            }
        }
    }
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
    Enum(ImportEnum),
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct ImportFunction {
    pub function: Function,
    pub rust_name: Ident,
    pub js_ret: Option<syn::Type>,
    pub catch: bool,
    pub variadic: bool,
    pub structural: bool,
    pub assert_no_shim: bool,
    pub kind: ImportFunctionKind,
    pub shim: Ident,
    pub doc_comment: Option<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum ImportFunctionKind {
    Method {
        class: String,
        ty: syn::Type,
        kind: MethodKind,
    },
    Normal,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum MethodKind {
    Constructor,
    Operation(Operation),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Operation {
    pub is_static: bool,
    pub kind: OperationKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum OperationKind {
    Regular,
    Getter(Option<Ident>),
    Setter(Option<Ident>),
    IndexingGetter,
    IndexingSetter,
    IndexingDeleter,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct ImportStatic {
    pub vis: syn::Visibility,
    pub ty: syn::Type,
    pub shim: Ident,
    pub rust_name: Ident,
    pub js_name: String,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct ImportType {
    pub vis: syn::Visibility,
    pub rust_name: Ident,
    pub js_name: String,
    pub attrs: Vec<syn::Attribute>,
    pub typescript_type: Option<String>,
    pub doc_comment: Option<String>,
    pub instanceof_shim: String,
    pub is_type_of: Option<syn::Expr>,
    pub extends: Vec<syn::Path>,
    pub vendor_prefixes: Vec<Ident>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct ImportEnum {
    /// The Rust enum's visibility
    pub vis: syn::Visibility,
    /// The Rust enum's identifiers
    pub name: Ident,
    /// The Rust identifiers for the variants
    pub variants: Vec<Ident>,
    /// The JS string values of the variants
    pub variant_values: Vec<String>,
    /// Attributes to apply to the Rust enum
    pub rust_attrs: Vec<syn::Attribute>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub name_span: Span,
    pub renamed_via_js_name: bool,
    pub arguments: Vec<syn::PatType>,
    pub ret: Option<syn::Type>,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_vis: syn::Visibility,
    pub r#async: bool,
    pub generate_typescript: bool,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Struct {
    pub rust_name: Ident,
    pub js_name: String,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
    pub is_inspectable: bool,
    pub generate_typescript: bool,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct StructField {
    pub name: syn::Member,
    pub struct_name: Ident,
    pub readonly: bool,
    pub ty: syn::Type,
    pub getter: Ident,
    pub setter: Ident,
    pub comments: Vec<String>,
    pub generate_typescript: bool,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Enum {
    pub rust_name: Ident,
    pub js_name: String,
    pub variants: Vec<Variant>,
    pub comments: Vec<String>,
    pub hole: u32,
    pub generate_typescript: bool,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Variant {
    pub name: Ident,
    pub value: u32,
    pub comments: Vec<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    ByRef,
    ByMutRef,
    ByValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TypeLocation {
    ImportArgument,
    ImportRet,
    ExportArgument,
    ExportRet,
}

impl Export {
    /// Mangles a rust -> javascript export, so that the created Ident will be unique over function
    /// name and class name, if the function belongs to a javascript class.
    pub(crate) fn rust_symbol(&self) -> Ident {
        let mut generated_name = String::from("__wasm_bindgen_generated");
        if let Some(class) = &self.js_class {
            generated_name.push_str("_");
            generated_name.push_str(class);
        }
        generated_name.push_str("_");
        generated_name.push_str(&self.function.name.to_string());
        Ident::new(&generated_name, Span::call_site())
    }

    /// This is the name of the shim function that gets exported and takes the raw
    /// ABI form of its arguments and converts them back into their normal,
    /// "high level" form before calling the actual function.
    pub(crate) fn export_name(&self) -> String {
        let fn_name = self.function.name.to_string();
        match &self.js_class {
            Some(class) => shared::struct_function_export_name(class, &fn_name),
            None => shared::free_function_export_name(&fn_name),
        }
    }
}

impl ImportKind {
    /// Whether this type can be inside an `impl` block.
    pub fn fits_on_impl(&self) -> bool {
        match *self {
            ImportKind::Function(_) => true,
            ImportKind::Static(_) => false,
            ImportKind::Type(_) => false,
            ImportKind::Enum(_) => false,
        }
    }
}

impl Function {
    /// If the rust object has a `fn xxx(&self) -> MyType` method, get the name for a getter in
    /// javascript (in this case `xxx`, so you can write `val = obj.xxx`)
    pub fn infer_getter_property(&self) -> &str {
        &self.name
    }

    /// If the rust object has a `fn set_xxx(&mut self, MyType)` style method, get the name
    /// for a setter in javascript (in this case `xxx`, so you can write `obj.xxx = val`)
    pub fn infer_setter_property(&self) -> Result<String, Diagnostic> {
        let name = self.name.to_string();

        // Otherwise we infer names based on the Rust function name.
        if !name.starts_with("set_") {
            bail_span!(
                syn::token::Pub(self.name_span),
                "setters must start with `set_`, found: {}",
                name,
            );
        }
        Ok(name[4..].to_string())
    }
}

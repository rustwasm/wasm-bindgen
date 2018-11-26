use Diagnostic;
use proc_macro2::{Ident, Span};
use shared;
use syn;

/// An abstract syntax tree representing a rust program. Contains
/// extra information for joining up this rust code with javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
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
    /// rust consts
    pub consts: Vec<Const>,
    /// "dictionaries", generated for WebIDL, which are basically just "typed
    /// objects" in the sense that they represent a JS object with a particular
    /// shape in JIT parlance.
    pub dictionaries: Vec<Dictionary>,
    /// custom typescript sections to be included in the definition file
    pub typescript_custom_sections: Vec<String>,
}

/// A rust to js interface. Allows interaction with rust objects/functions
/// from javascript.
#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Export {
    /// The struct name, in Rust, this is attached to
    pub rust_class: Option<Ident>,
    /// The class name in JS this is attached to
    pub js_class: Option<String>,
    /// The type of `self` (either `self`, `&self`, or `&mut self`)
    pub method_self: Option<MethodSelf>,
    /// Whether or not this export is flagged as a constructor, returning an
    /// instance of the `impl` type
    pub is_constructor: bool,
    /// The rust function
    pub function: Function,
    /// Comments extracted from the rust source.
    pub comments: Vec<String>,
    /// The name of the rust function/method on the rust side.
    pub rust_name: Ident,
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

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Import {
    pub module: Option<String>,
    pub js_namespace: Option<Ident>,
    pub kind: ImportKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
    Enum(ImportEnum),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct ImportFunction {
    pub function: Function,
    pub rust_name: Ident,
    pub js_ret: Option<syn::Type>,
    pub catch: bool,
    pub variadic: bool,
    pub structural: bool,
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
    pub doc_comment: Option<String>,
    pub instanceof_shim: String,
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

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub name_span: Span,
    pub renamed_via_js_name: bool,
    pub arguments: Vec<syn::ArgCaptured>,
    pub ret: Option<syn::Type>,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_vis: syn::Visibility,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Struct {
    pub rust_name: Ident,
    pub js_name: String,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct StructField {
    pub name: Ident,
    pub struct_name: Ident,
    pub readonly: bool,
    pub ty: syn::Type,
    pub getter: Ident,
    pub setter: Ident,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<Variant>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Variant {
    pub name: Ident,
    pub value: u32,
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

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct Const {
    pub vis: syn::Visibility,
    pub name: Ident,
    pub class: Option<Ident>,
    pub ty: syn::Type,
    pub value: ConstValue,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
#[derive(Clone)]
/// same as webidl::ast::ConstValue
pub enum ConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    SignedIntegerLiteral(i64),
    UnsignedIntegerLiteral(u64),
    Null,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct Dictionary {
    pub name: Ident,
    pub fields: Vec<DictionaryField>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
#[derive(Clone)]
pub struct DictionaryField {
    pub name: Ident,
    pub required: bool,
    pub ty: syn::Type,
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

impl ImportFunction {
    /// If the rust object has a `fn xxx(&self) -> MyType` method, get the name for a getter in
    /// javascript (in this case `xxx`, so you can write `val = obj.xxx`)
    pub fn infer_getter_property(&self) -> &str {
        &self.function.name
    }

    /// If the rust object has a `fn set_xxx(&mut self, MyType)` style method, get the name
    /// for a setter in javascript (in this case `xxx`, so you can write `obj.xxx = val`)
    pub fn infer_setter_property(&self) -> Result<String, Diagnostic> {
        let name = self.function.name.to_string();

        // if `#[wasm_bindgen(js_name = "...")]` is used then that explicitly
        // because it was hand-written anyway.
        if self.function.renamed_via_js_name {
            return Ok(name);
        }

        // Otherwise we infer names based on the Rust function name.
        if !name.starts_with("set_") {
            bail_span!(
                syn::token::Pub(self.function.name_span),
                "setters must start with `set_`, found: {}",
                name,
            );
        }
        Ok(name[4..].to_string())
    }
}

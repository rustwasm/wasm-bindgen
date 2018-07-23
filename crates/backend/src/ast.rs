use proc_macro2::{Ident, Span};
use shared;
use syn;

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
#[derive(Default)]
pub struct Program {
    pub exports: Vec<Export>,
    pub imports: Vec<Import>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub type_aliases: Vec<TypeAlias>,
    pub consts: Vec<Const>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Export {
    pub class: Option<Ident>,
    pub method_self: Option<MethodSelf>,
    pub constructor: Option<String>,
    pub function: Function,
    pub comments: Vec<String>,
    pub rust_name: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum MethodSelf {
    ByValue,
    RefMutable,
    RefShared,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Import {
    pub module: Option<String>,
    pub version: Option<String>,
    pub js_namespace: Option<Ident>,
    pub kind: ImportKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum ImportKind {
    Function(ImportFunction),
    Static(ImportStatic),
    Type(ImportType),
    Enum(ImportEnum),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportFunction {
    pub function: Function,
    pub rust_name: Ident,
    pub js_ret: Option<syn::Type>,
    pub catch: bool,
    pub structural: bool,
    pub kind: ImportFunctionKind,
    pub shim: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum ImportFunctionKind {
    Method {
        class: String,
        ty: syn::Type,
        kind: MethodKind,
    },
    Normal,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum MethodKind {
    Constructor,
    Operation(Operation),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Operation {
    pub is_static: bool,
    pub kind: OperationKind,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub enum OperationKind {
    Regular,
    Setter(Option<Ident>),
    Getter(Option<Ident>),
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportStatic {
    pub vis: syn::Visibility,
    pub ty: syn::Type,
    pub shim: Ident,
    pub rust_name: Ident,
    pub js_name: Ident,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct ImportType {
    pub vis: syn::Visibility,
    pub name: Ident,
    pub attrs: Vec<syn::Attribute>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
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
pub struct Function {
    pub name: Ident,
    pub arguments: Vec<syn::ArgCaptured>,
    pub ret: Option<syn::Type>,
    pub rust_attrs: Vec<syn::Attribute>,
    pub rust_vis: syn::Visibility,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct Struct {
    pub name: Ident,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
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
pub struct Enum {
    pub name: Ident,
    pub variants: Vec<Variant>,
    pub comments: Vec<String>,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
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

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq, Eq))]
pub struct TypeAlias {
    pub vis: syn::Visibility,
    pub dest: Ident,
    pub src: syn::Type,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
pub struct Const {
    pub vis: syn::Visibility,
    pub name: Ident,
    pub class: Option<Ident>,
    pub ty: syn::Type,
    pub value: ConstValue,
}

#[cfg_attr(feature = "extra-traits", derive(Debug, PartialEq))]
/// same as webidl::ast::ConstValue
pub enum ConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    SignedIntegerLiteral(i64),
    UnsignedIntegerLiteral(u64),
    Null,
}

impl Program {
    pub(crate) fn shared(&self) -> shared::Program {
        shared::Program {
            exports: self.exports.iter().map(|a| a.shared()).collect(),
            structs: self.structs.iter().map(|a| a.shared()).collect(),
            enums: self.enums.iter().map(|a| a.shared()).collect(),
            imports: self.imports.iter().map(|a| a.shared()).collect(),
            version: shared::version(),
            schema_version: shared::SCHEMA_VERSION.to_string(),
        }
    }
}

impl Function {
    fn shared(&self) -> shared::Function {
        shared::Function {
            name: self.name.to_string(),
        }
    }
}

impl Export {
    pub(crate) fn rust_symbol(&self) -> Ident {
        let mut generated_name = String::from("__wasm_bindgen_generated");
        if let Some(class) = &self.class {
            generated_name.push_str("_");
            generated_name.push_str(&class.to_string());
        }
        generated_name.push_str("_");
        generated_name.push_str(&self.function.name.to_string());
        Ident::new(&generated_name, Span::call_site())
    }

    pub(crate) fn export_name(&self) -> String {
        let fn_name = self.function.name.to_string();
        match &self.class {
            Some(class) => shared::struct_function_export_name(&class.to_string(), &fn_name),
            None => shared::free_function_export_name(&fn_name),
        }
    }

    fn shared(&self) -> shared::Export {
        let (method, consumed) = match self.method_self {
            Some(MethodSelf::ByValue) => (true, true),
            Some(_) => (true, false),
            None => (false, false),
        };
        shared::Export {
            class: self.class.as_ref().map(|s| s.to_string()),
            method,
            consumed,
            constructor: self.constructor.clone(),
            function: self.function.shared(),
            comments: self.comments.clone(),
        }
    }
}

impl Enum {
    fn shared(&self) -> shared::Enum {
        shared::Enum {
            name: self.name.to_string(),
            variants: self.variants.iter().map(|v| v.shared()).collect(),
            comments: self.comments.clone(),
        }
    }
}

impl Variant {
    fn shared(&self) -> shared::EnumVariant {
        shared::EnumVariant {
            name: self.name.to_string(),
            value: self.value,
        }
    }
}

impl Import {
    fn shared(&self) -> shared::Import {
        match (&self.module, &self.version) {
            (&Some(ref m), None) if m.starts_with("./") => {}
            (&Some(ref m), &Some(_)) if m.starts_with("./") => {
                panic!(
                    "when a module path starts with `./` that indicates \
                     that a local file is being imported so the `version` \
                     key cannot also be specified"
                );
            }
            (&Some(_), &Some(_)) => {}
            (&Some(_), &None) => panic!(
                "when the `module` directive doesn't start with `./` \
                 then it's interpreted as an NPM package which requires \
                 a `version` to be specified as well, try using \
                 #[wasm_bindgen(module = \"...\", version = \"...\")]"
            ),
            (&None, &Some(_)) => {
                panic!(
                    "the #[wasm_bindgen(version = \"...\")] attribute can only \
                     be used when `module = \"...\"` is also specified"
                );
            }
            (&None, &None) => {}
        }
        shared::Import {
            module: self.module.clone(),
            version: self.version.clone(),
            js_namespace: self.js_namespace.as_ref().map(|s| s.to_string()),
            kind: self.kind.shared(),
        }
    }
}

impl ImportKind {
    pub fn fits_on_impl(&self) -> bool {
        match *self {
            ImportKind::Function(_) => true,
            ImportKind::Static(_) => false,
            ImportKind::Type(_) => false,
            ImportKind::Enum(_) => false,
        }
    }

    fn shared(&self) -> shared::ImportKind {
        match *self {
            ImportKind::Function(ref f) => shared::ImportKind::Function(f.shared()),
            ImportKind::Static(ref f) => shared::ImportKind::Static(f.shared()),
            ImportKind::Type(ref f) => shared::ImportKind::Type(f.shared()),
            ImportKind::Enum(ref f) => shared::ImportKind::Enum(f.shared()),
        }
    }
}

impl ImportFunction {
    fn infer_getter_property(&self) -> String {
        self.function.name.to_string()
    }

    fn infer_setter_property(&self) -> String {
        let name = self.function.name.to_string();
        assert!(name.starts_with("set_"), "setters must start with `set_`");
        name[4..].to_string()
    }

    fn shared(&self) -> shared::ImportFunction {
        let method = match self.kind {
            ImportFunctionKind::Method {
                ref class,
                ref kind,
                ..
            } => {
                let kind = match kind {
                    MethodKind::Constructor => shared::MethodKind::Constructor,
                    MethodKind::Operation(Operation { is_static, kind }) => {
                        let is_static = *is_static;
                        let kind = match kind {
                            OperationKind::Regular => shared::OperationKind::Regular,
                            OperationKind::Getter(g) => {
                                let g = g.as_ref().map(|g| g.to_string());
                                shared::OperationKind::Getter(
                                    g.unwrap_or_else(|| self.infer_getter_property()),
                                )
                            }
                            OperationKind::Setter(s) => {
                                let s = s.as_ref().map(|s| s.to_string());
                                shared::OperationKind::Setter(
                                    s.unwrap_or_else(|| self.infer_setter_property()),
                                )
                            }
                        };
                        shared::MethodKind::Operation(shared::Operation { is_static, kind })
                    }
                };
                Some(shared::MethodData {
                    class: class.clone(),
                    kind,
                })
            }
            ImportFunctionKind::Normal => None,
        };

        shared::ImportFunction {
            shim: self.shim.to_string(),
            catch: self.catch,
            method,
            structural: self.structural,
            function: self.function.shared(),
        }
    }
}

impl ImportStatic {
    fn shared(&self) -> shared::ImportStatic {
        shared::ImportStatic {
            name: self.js_name.to_string(),
            shim: self.shim.to_string(),
        }
    }
}

impl ImportType {
    fn shared(&self) -> shared::ImportType {
        shared::ImportType {}
    }
}

impl ImportEnum {
    fn shared(&self) -> shared::ImportEnum {
        shared::ImportEnum {}
    }
}

impl Struct {
    fn shared(&self) -> shared::Struct {
        shared::Struct {
            name: self.name.to_string(),
            fields: self.fields.iter().map(|s| s.shared()).collect(),
            comments: self.comments.clone(),
        }
    }
}

impl StructField {
    fn shared(&self) -> shared::StructField {
        shared::StructField {
            name: self.name.to_string(),
            readonly: self.readonly,
            comments: self.comments.clone(),
        }
    }
}

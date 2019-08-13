use crate::ast;
use proc_macro2::Ident;
use syn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportedTypeKind {
    /// The definition of an imported type.
    Definition,
    /// A reference to an imported type.
    Reference,
}

impl<T> ImportedTypes for Option<T>
where
    T: ImportedTypes,
{
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        if let Some(inner) = self {
            inner.imported_types(f);
        }
    }
}

/// Iterate over definitions of and references to imported types in the AST.
pub trait ImportedTypes {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind);
}

/// Iterate over definitions of imported types in the AST.
pub trait ImportedTypeDefinitions {
    fn imported_type_definitions<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident);
}

impl<T> ImportedTypeDefinitions for T
where
    T: ImportedTypes,
{
    fn imported_type_definitions<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.imported_types(&mut |id, kind| {
            if let ImportedTypeKind::Definition = kind {
                f(id);
            }
        });
    }
}

/// Iterate over references to imported types in the AST.
pub trait ImportedTypeReferences {
    fn imported_type_references<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident);
}

impl<T> ImportedTypeReferences for T
where
    T: ImportedTypes,
{
    fn imported_type_references<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident),
    {
        self.imported_types(&mut |id, kind| {
            if let ImportedTypeKind::Reference = kind {
                f(id);
            }
        });
    }
}

impl<'a, T: ImportedTypes> ImportedTypes for &'a T {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        (*self).imported_types(f)
    }
}

impl ImportedTypes for ast::Program {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.imports.imported_types(f);
        self.consts.imported_types(f);
        self.dictionaries.imported_types(f);
    }
}

impl<T> ImportedTypes for Vec<T>
where
    T: ImportedTypes,
{
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        for x in self {
            x.imported_types(f);
        }
    }
}

impl ImportedTypes for ast::Import {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.kind.imported_types(f)
    }
}

impl ImportedTypes for ast::ImportKind {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            ast::ImportKind::Static(s) => s.imported_types(f),
            ast::ImportKind::Function(fun) => fun.imported_types(f),
            ast::ImportKind::Type(ty) => ty.imported_types(f),
            ast::ImportKind::Enum(enm) => enm.imported_types(f),
        }
    }
}

impl ImportedTypes for ast::ImportStatic {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.ty.imported_types(f);
    }
}

impl ImportedTypes for syn::Type {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            syn::Type::Reference(ref r) => r.imported_types(f),
            syn::Type::Path(ref p) => p.imported_types(f),
            _ => {}
        }
    }
}

impl ImportedTypes for syn::TypeReference {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.elem.imported_types(f);
    }
}

impl ImportedTypes for syn::TypePath {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.qself.imported_types(f);
        self.path.imported_types(f);
    }
}

impl ImportedTypes for syn::QSelf {
    fn imported_types<F>(&self, _: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        // TODO
    }
}

impl ImportedTypes for syn::Path {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        for seg in self.segments.iter() {
            seg.arguments.imported_types(f);
        }
        f(
            &self.segments.last().unwrap().ident,
            ImportedTypeKind::Reference,
        );
    }
}

impl ImportedTypes for syn::PathArguments {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            syn::PathArguments::AngleBracketed(data) => {
                for arg in data.args.iter() {
                    arg.imported_types(f);
                }
            }
            //TOCHECK
            syn::PathArguments::Parenthesized(data) => {
                for input in data.inputs.iter() {
                    input.imported_types(f);
                }
                // TODO do we need to handle output here?
                // https://docs.rs/syn/0.14.0/syn/struct.ParenthesizedGenericArguments.html
            }
            syn::PathArguments::None => {}
        }
    }
}

impl ImportedTypes for syn::GenericArgument {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            syn::GenericArgument::Lifetime(_) => {}
            syn::GenericArgument::Type(ty) => ty.imported_types(f),
            syn::GenericArgument::Binding(_) => {}    // TODO
            syn::GenericArgument::Const(_) => {}      // TODO
            syn::GenericArgument::Constraint(_) => {} // TODO
        }
    }
}

impl ImportedTypes for ast::ImportFunction {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.function.imported_types(f);
        self.kind.imported_types(f);
    }
}

impl ImportedTypes for ast::ImportFunctionKind {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            ast::ImportFunctionKind::Method { ty, .. } => ty.imported_types(f),
            ast::ImportFunctionKind::Normal => {}
        }
    }
}

impl ImportedTypes for ast::Function {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.arguments.imported_types(f);
        if let Some(ref r) = self.ret {
            r.imported_types(f);
        }
    }
}

impl ImportedTypes for syn::FnArg {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        match self {
            syn::FnArg::Receiver(_) => {}
            syn::FnArg::Typed(p) => p.imported_types(f),
        }
    }
}

impl ImportedTypes for syn::PatType {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.ty.imported_types(f)
    }
}

impl ImportedTypes for ast::ImportType {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        f(&self.rust_name, ImportedTypeKind::Definition);
        for class in self.extends.iter() {
            class.imported_types(f);
        }
    }
}

impl ImportedTypes for ast::ImportEnum {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        f(&self.name, ImportedTypeKind::Definition);
    }
}

impl ImportedTypes for ast::Const {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.ty.imported_types(f);
    }
}

impl ImportedTypes for ast::Dictionary {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        f(&self.name, ImportedTypeKind::Definition);
        for field in self.fields.iter() {
            field.imported_types(f);
        }
    }
}

impl ImportedTypes for ast::DictionaryField {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&Ident, ImportedTypeKind),
    {
        self.ty.imported_types(f);
    }
}

/// Remove any methods, statics, &c, that reference types that are *not*
/// defined.
pub trait RemoveUndefinedImports {
    fn remove_undefined_imports<F>(&mut self, is_defined: &F) -> bool
    where
        F: Fn(&Ident) -> bool;
}

impl RemoveUndefinedImports for ast::Program {
    fn remove_undefined_imports<F>(&mut self, is_defined: &F) -> bool
    where
        F: Fn(&Ident) -> bool,
    {
        let mut changed = self.imports.remove_undefined_imports(is_defined);
        changed = self.consts.remove_undefined_imports(is_defined) || changed;

        for dictionary in self.dictionaries.iter_mut() {
            let num_required =
                |dict: &ast::Dictionary| dict.fields.iter().filter(|f| f.required).count();
            let before = num_required(dictionary);
            changed = dictionary.fields.remove_undefined_imports(is_defined) || changed;

            // If a required field was removed we can no longer construct this
            // dictionary so disable the constructor.
            if before != num_required(dictionary) {
                dictionary.ctor = false;
            }
        }

        changed
    }
}

impl<T> RemoveUndefinedImports for Vec<T>
where
    T: ImportedTypeReferences,
{
    fn remove_undefined_imports<F>(&mut self, is_defined: &F) -> bool
    where
        F: Fn(&Ident) -> bool,
    {
        let before = self.len();
        self.retain(|x| {
            let mut all_defined = true;
            x.imported_type_references(&mut |id| {
                if all_defined {
                    if !is_defined(id) {
                        log::info!("removing due to {} not being defined", id);
                        all_defined = false;
                    }
                }
            });
            all_defined
        });
        before != self.len()
    }
}

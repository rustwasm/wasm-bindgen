use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Ident, Path,
};

/// Convert `self` into a valid `syn::Lit`, preserving spans.
pub trait IntoLit {
    /// Convert `self` into a valid literal.
    ///
    /// Implementers should set the span of the returned literal to match that of `self`.
    fn into_lit(self) -> syn::Lit;
}

impl IntoLit for syn::Lit {
    fn into_lit(self) -> syn::Lit {
        self
    }
}

/// Create a `syn::Lit` by calling `quote!` and wrapping the contents in quotation marks.
pub fn quote_lit<T: Spanned + ToTokens>(v: &T) -> syn::Lit {
    syn::LitStr::new(&quote!(#v).to_string(), v.span()).into()
}

/// A `syn::Meta` that allows values other than literals.
#[derive(Debug, Clone)]
pub enum Meta<T> {
    Path(Path),
    NameValue(MetaNameValue<T>),
    List(MetaList<T>),
}

impl<T: Parse> Meta<T> {
    pub fn with_body(path: Path, body: TokenStream) -> syn::Result<Self> {
        if body.is_empty() {
            Ok(Self::Path(path))
        } else {
            syn::parse2(quote!(#path(#body)))
        }
    }
}

impl<T: IntoLit> From<Meta<T>> for syn::Meta {
    fn from(v: Meta<T>) -> Self {
        match v {
            Meta::Path(path) => Self::Path(path),
            Meta::List(list) => Self::List(list.into()),
            Meta::NameValue(nv) => Self::NameValue(nv.into()),
        }
    }
}

impl<T: Parse> Parse for Meta<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Look for paths, allowing for the possibility of keywords as idents
        let path = if input.fork().parse::<Path>().is_ok() {
            input.parse::<Path>()
        } else {
            input.call(Ident::parse_any).map(Path::from)
        }?;

        // Decide which variant is being looked at.
        if input.peek(Token![=]) {
            let eq_token = input.parse::<Token![=]>()?;
            let lit = input.parse::<T>()?;
            Ok(Self::NameValue(MetaNameValue {
                path,
                lit,
                eq_token,
            }))
        } else if input.peek(syn::token::Paren) {
            let content;
            Ok(Self::List(MetaList {
                path,
                paren_token: parenthesized!(content in input),
                nested: content.parse_terminated(NestedMeta::<T>::parse)?,
            }))
        } else {
            Ok(Self::Path(path))
        }
    }
}

/// Try to parse the body of an attribute as `Self`.
impl<T: Parse> TryFrom<syn::Attribute> for Meta<T> {
    type Error = syn::Error;

    fn try_from(value: syn::Attribute) -> Result<Self, Self::Error> {
        let syn::Attribute { path, tokens, .. } = value;
        syn::parse2(quote::quote!(#path #tokens))
    }
}

#[derive(Debug, Clone)]
pub enum NestedMeta<T> {
    Meta(Meta<T>),
    Lit(T),
}

impl<T: IntoLit> From<NestedMeta<T>> for syn::NestedMeta {
    fn from(v: NestedMeta<T>) -> Self {
        match v {
            NestedMeta::Meta(m) => Self::Meta(m.into()),
            NestedMeta::Lit(l) => Self::Lit(l.into_lit()),
        }
    }
}

/// This will only attempt to parse `Meta<T>`.
impl<T: Parse> Parse for NestedMeta<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self::Meta)
    }
}

#[derive(Debug, Clone)]
pub struct MetaList<T> {
    pub path: Path,
    pub paren_token: syn::token::Paren,
    pub nested: Punctuated<NestedMeta<T>, syn::token::Comma>,
}

impl<T: IntoLit> From<MetaList<T>> for syn::MetaList {
    fn from(v: MetaList<T>) -> Self {
        syn::MetaList {
            paren_token: v.paren_token,
            path: v.path,
            nested: v.nested.into_iter().map(syn::NestedMeta::from).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetaNameValue<T> {
    pub path: Path,
    pub eq_token: syn::token::Eq,
    pub lit: T,
}

impl<T: IntoLit> From<MetaNameValue<T>> for syn::MetaNameValue {
    fn from(
        MetaNameValue {
            path,
            lit,
            eq_token,
        }: MetaNameValue<T>,
    ) -> Self {
        Self {
            eq_token,
            path,
            lit: lit.into_lit(),
        }
    }
}

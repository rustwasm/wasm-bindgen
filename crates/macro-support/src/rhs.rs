use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Expr, Ident, LitStr, Path,
};

use crate::meta::{quote_lit, IntoLit};

/// Valid syntax to appear to the right of `=` in `wasm_bindgen` attributes.
#[derive(Debug, Clone)]
pub enum Rhs {
    Path(Path),
    LitStr(LitStr),
    Expr(Expr),
}

impl Parse for Rhs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.fork().parse::<LitStr>().is_ok() {
            input.parse().map(Self::LitStr)
        } else if input.fork().parse::<Path>().is_ok() {
            input.parse().map(Self::Path)
        } else if input.fork().call(Ident::parse_any).is_ok() {
            input
                .call(Ident::parse_any)
                .map(syn::Path::from)
                .map(Self::Path)
        } else {
            input.parse().map(Self::Expr)
        }
    }
}

impl IntoLit for Rhs {
    fn into_lit(self) -> syn::Lit {
        match self {
            Self::Path(p) => quote_lit(&p),
            Self::LitStr(s) => s.into(),
            Self::Expr(e) => quote_lit(&e),
        }
    }
}

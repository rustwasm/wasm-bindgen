use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    Expr, Ident, LitStr, Path,
};

use crate::{meta::IntoLit, quote_lit};

/// Valid syntax to appear to the right of `=` in `wasm_bindgen` attributes.
#[derive(Debug, Clone)]
pub enum Rhs {
    Path(Path),
    LitStr(LitStr),
    Expr(Expr),
}

impl From<Path> for Rhs {
    fn from(v: Path) -> Self {
        Self::Path(v)
    }
}

impl From<LitStr> for Rhs {
    fn from(v: LitStr) -> Self {
        Self::LitStr(v)
    }
}

impl From<Expr> for Rhs {
    fn from(v: Expr) -> Self {
        Self::Expr(v)
    }
}

impl Parse for Rhs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.fork().parse::<LitStr>().is_ok() {
            input.parse::<LitStr>().map(Self::from)
        } else if input.fork().parse::<Path>().is_ok() {
            input.parse::<Path>().map(Self::from)
        } else if input.fork().call(Ident::parse_any).is_ok() {
            input
                .call(Ident::parse_any)
                .map(syn::Path::from)
                .map(Self::from)
        } else {
            input.parse::<Expr>().map(Self::from)
        }
    }
}

impl IntoLit for Rhs {
    fn into_lit(self) -> syn::Lit {
        syn::Lit::Str(match self {
            Self::Path(p) => quote_lit!(p),
            Self::LitStr(s) => s,
            Self::Expr(e) => quote_lit!(e),
        })
    }
}

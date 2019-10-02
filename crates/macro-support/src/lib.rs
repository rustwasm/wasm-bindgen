//! This crate contains the part of the implementation of the `#[wasm_bindgen]` optsibute that is
//! not in the shared backend crate.

#![doc(html_root_url = "https://docs.rs/wasm-bindgen-macro-support/0.2")]

extern crate proc_macro2;
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate wasm_bindgen_backend as backend;
extern crate wasm_bindgen_shared as shared;

pub use crate::parser::BindgenAttrs;
use crate::parser::MacroParse;
use backend::{Diagnostic, TryToTokens};
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result as SynResult};

mod parser;

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();
    let opts = syn::parse2::<BindgenAttrs>(attr.clone())?;

    // If `__wasm_target` attribute is missing, inject it and re-run.
    // This way, the parser can determine whether we are compiling for a wasm target
    // in order to inject the necessary `__proto__` field whilst tracking such field
    // member in the AST.
    if !opts.has_wasm_target() {
        let modified = quote::quote! {
            #[cfg_attr(
                all(target_arch = "wasm32", not(target_os = "emscripten")),
                ::wasm_bindgen::prelude::wasm_bindgen(__wasm_target = true, #attr)
            )]
            #[cfg_attr(
                not(all(target_arch = "wasm32", not(target_os = "emscripten"))),
                ::wasm_bindgen::prelude::wasm_bindgen(__wasm_target = false, #attr)
            )]
            #input
        };
        return Ok(modified);
    }

    let item = syn::parse2::<syn::Item>(input)?;

    let mut tokens = proc_macro2::TokenStream::new();
    let mut program = backend::ast::Program::default();
    item.macro_parse(&mut program, (Some(opts), &mut tokens))?;
    program.try_to_tokens(&mut tokens)?;

    // If we successfully got here then we should have used up all attributes
    // and considered all of them to see if they were used. If one was forgotten
    // that's a bug on our end, so sanity check here.
    parser::assert_all_attrs_checked();

    Ok(tokens)
}

/// Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings
pub fn expand_class_marker(
    attr: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, Diagnostic> {
    parser::reset_attrs_used();
    let mut item = syn::parse2::<syn::ImplItemMethod>(input)?;
    let opts: ClassMarker = syn::parse2(attr)?;

    let mut program = backend::ast::Program::default();
    item.macro_parse(&mut program, (&opts.class, &opts.js_class, opts.wasm_target))?;
    parser::assert_all_attrs_checked(); // same as above

    // This is where things are slightly different, we are being expanded in the
    // context of an impl so we can't inject arbitrary item-like tokens into the
    // output stream. If we were to do that then it wouldn't parse!
    //
    // Instead what we want to do is to generate the tokens for `program` into
    // the header of the function. This'll inject some no_mangle functions and
    // statics and such, and they should all be valid in the context of the
    // start of a function.
    //
    // We manually implement `ToTokens for ImplItemMethod` here, injecting our
    // program's tokens before the actual method's inner body tokens.
    let mut tokens = proc_macro2::TokenStream::new();
    tokens.append_all(item.attrs.iter().filter(|attr| match attr.style {
        syn::AttrStyle::Outer => true,
        _ => false,
    }));
    item.vis.to_tokens(&mut tokens);
    item.sig.to_tokens(&mut tokens);
    let mut err = None;
    item.block.brace_token.surround(&mut tokens, |tokens| {
        if let Err(e) = program.try_to_tokens(tokens) {
            err = Some(e);
        }
        tokens.append_all(item.attrs.iter().filter(|attr| match attr.style {
            syn::AttrStyle::Inner(_) => true,
            _ => false,
        }));
        tokens.append_all(&item.block.stmts);
    });

    if let Some(err) = err {
        return Err(err);
    }

    Ok(tokens)
}

struct ClassMarker {
    class: syn::Ident,
    js_class: String,
    wasm_target: bool,
}

impl Parse for ClassMarker {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let class = input.parse::<syn::Ident>()?;
        input.parse::<Token![=]>()?;
        let js_class = input.parse::<syn::LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let wasm_target = input.parse::<syn::LitBool>()?.value;
        Ok(ClassMarker { class, js_class, wasm_target })
    }
}

pub fn expand_instantiate(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    Ok(syn::parse2::<Instantiation>(input)?.into_token_stream())
}

struct Instantiation {
    ty: syn::Path,
    expr: syn::Expr,
}

impl Parse for Instantiation {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let prototype: syn::Expr = if let Ok(_) = input.parse::<Token![super]>() {
            let args;
            parenthesized!(args in input);
            input.parse::<Token![;]>()?;

            let mut super_args = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated(&args)?;
            super_args.iter_mut().for_each(|expr| *expr = parse_quote! { #expr.into() });

            parse_quote! { __wbg_callback.invoke(Box::new([ #super_args ])) }
        } else {
            parse_quote! { __wbg_callback.get() }
        };

        let without_prototype = input.parse::<syn::Expr>()?;
        let (ty, with_prototype): (syn::Path, syn::Expr) = match &without_prototype {
            syn::Expr::Call(expr_call) => {
                let ty = match &*expr_call.func {
                    syn::Expr::Path(expr_path) => expr_path.path.clone(),
                    err => return Err(syn::Error::new_spanned(err, "expression is not a struct literal")),
                };
                let mut with_prototype = expr_call.clone();
                with_prototype.args.push(parse_quote!{ #prototype });
                (ty, with_prototype.into())
            }
            
            syn::Expr::Struct(expr_struct) => {
                let mut with_prototype = expr_struct.clone();
                with_prototype.fields.push(parse_quote! { __proto__: #prototype });
                (expr_struct.path.clone(), with_prototype.into())
            }
            
            syn::Expr::Path(expr_path) => {
                let with_prototype = parse_quote! { #without_prototype { __proto__: #prototype } };
                (expr_path.path.clone(), with_prototype)
            }
            
            err => return Err(syn::Error::new_spanned(err, "expression not a struct literal")),
        };

        Ok(Instantiation {
            ty,
            expr: parse_quote!{{
                #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                let expr = { #with_prototype };

                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                let expr = { #without_prototype };

                expr
            }}
        })
    }
}

impl ToTokens for Instantiation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = &self.expr;
        let ty = &self.ty;

        (quote::quote! {{
            use ::wasm_bindgen::{JsValue, WasmType, SuperconstructorCallback};
            use ::wasm_bindgen::__rt::{WasmRefCell, core::convert::AsRef};

            let wasm = WasmType::new(WasmRefCell::new(#expr));

            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            <#ty as AsRef<JsValue>>::as_ref(&*wasm.borrow())
              .set_wasm_pointer( Box::into_raw(Box::new(WasmType::clone(&wasm))) );
            
            wasm
        }})
        .to_tokens(tokens);
    }
}

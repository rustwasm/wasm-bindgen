//! See the README for `wasm-bindgen-test` for a bit more info about what's
//! going on here.

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

use std::sync::atomic::*;
use proc_macro2::*;

static CNT: AtomicUsize = ATOMIC_USIZE_INIT;

#[proc_macro_attribute]
pub fn wasm_bindgen_test(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if attr.into_iter().next().is_some() {
        panic!("this attribute currently takes no arguments");
    }

    let mut body = TokenStream::from(body).into_iter();

    // Assume the input item is of the form `fn #ident ...`, and extract
    // `#ident`
    let fn_tok = body.next();
    let ident = match body.next() {
        Some(TokenTree::Ident(token)) => token,
        _ => panic!("expected a function name"),
    };

    let mut tokens = Vec::<TokenTree>::new();

    // We generate a `#[no_mangle]` with a known prefix so the test harness can
    // later slurp up all of these functions and pass them as arguments to the
    // main test harness. This is the entry point for all tests.
    let name = format!("__wbg_test_{}_{}", ident, CNT.fetch_add(1, Ordering::SeqCst));
    let name = Ident::new(&name, Span::call_site());
    tokens.extend((quote! {
        #[no_mangle]
        pub extern fn #name(cx: *const ::wasm_bindgen_test::__rt::Context) {
            unsafe {
                (*cx).execute(concat!(module_path!(), "::", stringify!(#ident)), #ident);
            }
        }
    }).into_iter());

    tokens.extend(fn_tok);
    tokens.push(ident.into());
    tokens.extend(body);

    tokens.into_iter().collect::<TokenStream>().into()
}

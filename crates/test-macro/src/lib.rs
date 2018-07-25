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
    let mut attr = attr.into_iter();
    let mut async = false;
    while let Some(token) = attr.next() {
        match &token {
            proc_macro::TokenTree::Ident(i) if i.to_string() == "async" => async = true,
            _ => panic!("malformed `#[wasm_bindgen_test]` attribute"),
        }
        match &attr.next() {
            Some(proc_macro::TokenTree::Punct(op)) if op.as_char() == ',' => {}
            Some(_) => panic!("malformed `#[wasm_bindgen_test]` attribute"),
            None => break,
        }
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

    let test_body = if async {
        quote! { cx.execute_async(test_name, #ident); }
    } else {
        quote! { cx.execute_sync(test_name, #ident); }
    };

    // We generate a `#[no_mangle]` with a known prefix so the test harness can
    // later slurp up all of these functions and pass them as arguments to the
    // main test harness. This is the entry point for all tests.
    let name = format!("__wbg_test_{}_{}", ident, CNT.fetch_add(1, Ordering::SeqCst));
    let name = Ident::new(&name, Span::call_site());
    tokens.extend((quote! {
        #[no_mangle]
        pub extern fn #name(cx: *const ::wasm_bindgen_test::__rt::Context) {
            unsafe {
                let cx = &*cx;
                let test_name = concat!(module_path!(), "::", stringify!(#ident));
                #test_body
            }
        }
    }).into_iter());

    tokens.extend(fn_tok);
    tokens.push(ident.into());
    tokens.extend(body);

    tokens.into_iter().collect::<TokenStream>().into()
}

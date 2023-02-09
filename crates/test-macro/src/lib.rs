//! See the README for `wasm-bindgen-test` for a bit more info about what's
//! going on here.

extern crate proc_macro;

use proc_macro2::*;
use quote::quote;
use std::sync::atomic::*;

static CNT: AtomicUsize = AtomicUsize::new(0);

#[proc_macro_attribute]
pub fn wasm_bindgen_test(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut attr = attr.into_iter();
    let mut r#async = false;
    let mut should_panic = false;
    while let Some(token) = attr.next() {
        match &token {
            proc_macro::TokenTree::Ident(i) if i.to_string() == "async" => r#async = true,
            _ => panic!("malformed `#[wasm_bindgen_test]` attribute"),
        }
        match &attr.next() {
            Some(proc_macro::TokenTree::Punct(op)) if op.as_char() == ',' => {}
            Some(_) => panic!("malformed `#[wasm_bindgen_test]` attribute"),
            None => break,
        }
    }

    let mut body = TokenStream::from(body).into_iter();

    // Skip over other attributes to `fn #ident ...`, and extract `#ident`
    let mut leading_tokens = Vec::new();
    while let Some(token) = body.next() {
        leading_tokens.push(token.clone());

        match token {
            TokenTree::Punct(op) if op.as_char() == '#' => {
                if let Some(token) = body.next() {
                    leading_tokens.push(token.clone());

                    match token {
                        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                            let mut stream = group.stream().into_iter();

                            if let Some(TokenTree::Ident(token)) = stream.next() {
                                if token == "should_panic" {
                                    should_panic = true;
                                }
                            }
                        }
                        _ => panic!("expected an attribute"),
                    }
                }
            }
            TokenTree::Ident(token) => {
                if token == "async" {
                    r#async = true;
                }
                if token == "fn" {
                    break;
                }
            }
            _ => (),
        }
    }
    let ident = find_ident(&mut body).expect("expected a function name");

    let mut tokens = Vec::<TokenTree>::new();

    let test_body = if r#async {
        quote! { cx.execute_async(test_name, #ident, #should_panic); }
    } else {
        quote! { cx.execute_sync(test_name, #ident, #should_panic); }
    };

    // We generate a `#[no_mangle]` with a known prefix so the test harness can
    // later slurp up all of these functions and pass them as arguments to the
    // main test harness. This is the entry point for all tests.
    let name = format!("__wbgt_{}_{}", ident, CNT.fetch_add(1, Ordering::SeqCst));
    let name = Ident::new(&name, Span::call_site());
    tokens.extend(
        (quote! {
            #[no_mangle]
            pub extern "C" fn #name(cx: &::wasm_bindgen_test::__rt::Context) {
                let test_name = ::core::concat!(::core::module_path!(), "::", ::core::stringify!(#ident));
                #test_body
            }
        })
        .into_iter(),
    );

    tokens.extend(leading_tokens);
    tokens.push(ident.into());
    tokens.extend(body);

    tokens.into_iter().collect::<TokenStream>().into()
}

fn find_ident(iter: &mut token_stream::IntoIter) -> Option<Ident> {
    match iter.next()? {
        TokenTree::Ident(i) => Some(i),
        TokenTree::Group(g) if g.delimiter() == Delimiter::None => {
            find_ident(&mut g.stream().into_iter())
        }
        _ => None,
    }
}

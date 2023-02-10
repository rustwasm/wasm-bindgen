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
    let mut should_panic = None;
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
        match token {
            // Parse a potential `should_panic` attribute
            // Start by parsing the `#`
            TokenTree::Punct(op) if op.as_char() == '#' => {
                let token = if let Some(token) = body.next() {
                    token
                } else {
                    leading_tokens.push(op.into());
                    continue;
                };

                // Parse `[...]`
                let group = match token {
                    TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => group,
                    token => {
                        leading_tokens.push(token);
                        continue;
                    }
                };

                let mut stream = group.stream().into_iter();

                // Parse `should_panic`
                match stream.next() {
                    Some(TokenTree::Ident(token)) if token == "should_panic" => (),
                    Some(token) => {
                        leading_tokens.push(token);
                        continue;
                    }
                    None => continue,
                }

                // We are interested in the `expected` attribute or string if there is any
                match stream.next() {
                    // Parse the `(...)` in `#[should_panic(...)]`
                    Some(TokenTree::Group(group))
                        if group.delimiter() == Delimiter::Parenthesis =>
                    {
                        stream = group.stream().into_iter();

                        // Parse `expected`
                        match stream.next() {
                            Some(TokenTree::Ident(token)) if token == "expected" => (),
                            _ => panic!("malformed `#[should_panic(...)]` attribute"),
                        }

                        // Parse `=`
                        match stream.next() {
                            Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
                            _ => panic!("malformed `#[should_panic(...)]` attribute"),
                        }
                    }
                    // Parse `=`
                    Some(TokenTree::Punct(op)) if op.as_char() == '=' => (),
                    Some(_) => panic!("malformed `#[should_panic = \"...\"]` attribute"),
                    None => {
                        should_panic = Some(None);
                        continue;
                    }
                }

                // Parse string in `#[should_panic(expected = "string")]` or `#[should_panic = "string"]`
                if let Some(TokenTree::Literal(lit)) = stream.next() {
                    let string = lit.to_string();

                    // Verify it's a string.
                    if string.starts_with('"') && string.ends_with('"') {
                        should_panic = Some(Some(lit));
                        continue;
                    }
                }

                panic!("malformed `#[should_panic]` attribute");
            }
            TokenTree::Ident(token) => {
                leading_tokens.push(token.clone().into());

                if token == "async" {
                    r#async = true;
                }
                if token == "fn" {
                    break;
                }
            }
            token => leading_tokens.push(token),
        }
    }
    let ident = find_ident(&mut body).expect("expected a function name");

    let mut tokens = Vec::<TokenTree>::new();

    let should_panic = match should_panic {
        Some(Some(lit)) => quote! { Some(Some(#lit)) },
        Some(None) => quote! { Some(None) },
        None => quote! { None },
    };

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

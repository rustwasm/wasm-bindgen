#![feature(proc_macro)]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
extern crate wasm_bindgen_shared;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{Tokens, ToTokens};

mod ast;

#[proc_macro]
pub fn wasm_bindgen(input: TokenStream) -> TokenStream {
    let file = syn::parse::<syn::File>(input)
        .expect("expected a set of valid Rust items");


    let mut ret = Tokens::new();

    for item in file.items.iter() {
        item.to_tokens(&mut ret);
        match *item {
            syn::Item::Fn(ref f) => bindgen_fn(f, &mut ret),
            _ => panic!("unexpected item in bindgen macro"),
        }
    }

    ret.into()
}

fn bindgen_fn(input: &syn::ItemFn, into: &mut Tokens) {
    let function = ast::Function::from(input);

    let export_name = function.export_name();
    let generated_name = function.rust_symbol();
    let mut args = vec![];
    let arg_conversions = vec![quote!{}];
    let real_name = &input.ident;
    let mut converted_arguments = vec![];
    let ret = syn::Ident::from("_ret");

    for (i, ty) in function.arguments.iter().enumerate() {
        let ident = syn::Ident::from(format!("arg{}", i));
        match *ty {
            ast::Type::Integer(i) => {
                args.push(quote! { #ident: #i });
                converted_arguments.push(quote! { #ident });
            }
        }
    }
    let ret_ty;
    let convert_ret;
    match function.ret {
        Some(ast::Type::Integer(i)) => {
            ret_ty = quote! { -> #i };
            convert_ret = quote! { #ret };
        }
        None => {
            ret_ty = quote! {};
            convert_ret = quote! {};
        }
    }

    let generated_static_name = function.generated_static_name();
    let generated_static = function.generate_static();
    let generated_static_value = syn::Lit {
        value: syn::LitKind::Other(Literal::byte_string(&generated_static)),
        span: Default::default(),
    };
    let generated_static_length = generated_static.len();


    let tokens = quote! {
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static #generated_static_name: [u8; #generated_static_length] =
            *#generated_static_value;

        #[no_mangle]
        #[export_name = #export_name]
        pub extern fn #generated_name(#(#args),*) #ret_ty {
            #(#arg_conversions);*
            let #ret = #real_name(#(#converted_arguments),*);
            #convert_ret
        }
    };
    tokens.to_tokens(into);
}

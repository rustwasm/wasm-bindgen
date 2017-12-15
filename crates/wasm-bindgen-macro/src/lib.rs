#![feature(proc_macro)]

extern crate syn;
#[macro_use]
extern crate synom;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
extern crate wasm_bindgen_shared;

use std::sync::atomic::*;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{Tokens, ToTokens};

mod ast;

static MALLOC_GENERATED: AtomicBool = ATOMIC_BOOL_INIT;
static BOXED_STR_GENERATED: AtomicBool = ATOMIC_BOOL_INIT;

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
    let mut arg_conversions = vec![];
    let real_name = &input.ident;
    let mut converted_arguments = vec![];
    let ret = syn::Ident::from("_ret");

    let mut malloc = false;
    let mut boxed_str = false;

    for (i, ty) in function.arguments.iter().enumerate() {
        let ident = syn::Ident::from(format!("arg{}", i));
        match *ty {
            ast::Type::Integer(i) => {
                args.push(quote! { #ident: #i });
            }
            ast::Type::BorrowedStr => {
                malloc = !MALLOC_GENERATED.swap(true, Ordering::SeqCst);
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(quote! { #ptr: *const u8 });
                args.push(quote! { #len: usize });
                arg_conversions.push(quote! {
                    let #ident = unsafe {
                        let slice = ::std::slice::from_raw_parts(#ptr, #len);
                        ::std::str::from_utf8_unchecked(slice)
                    };
                });
            }
            ast::Type::String => {
                malloc = !MALLOC_GENERATED.swap(true, Ordering::SeqCst);
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(quote! { #ptr: *mut u8 });
                args.push(quote! { #len: usize });
                arg_conversions.push(quote! {
                    let #ident = unsafe {
                        let vec = ::std::vec::Vec::from_raw_parts(#ptr, #len, #len);
                        ::std::string::String::from_utf8_unchecked(vec)
                    };
                });
            }
        }
        converted_arguments.push(quote! { #ident });
    }
    let ret_ty;
    let convert_ret;
    match function.ret {
        Some(ast::Type::Integer(i)) => {
            ret_ty = quote! { -> #i };
            convert_ret = quote! { #ret };
        }
        Some(ast::Type::BorrowedStr) => panic!("can't return a borrowed string"),
        Some(ast::Type::String) => {
            boxed_str = !BOXED_STR_GENERATED.swap(true, Ordering::SeqCst);
            ret_ty = quote! { -> *mut String };
            convert_ret = quote! { Box::into_raw(Box::new(#ret)) };
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

    let malloc = if malloc {
        quote! {
            #[no_mangle]
            pub extern fn __wbindgen_malloc(size: usize) -> *mut u8 {
                let mut ret = Vec::with_capacity(size);
                let ptr = ret.as_mut_ptr();
                ::std::mem::forget(ret);
                return ptr
            }

            #[no_mangle]
            pub unsafe extern fn __wbindgen_free(ptr: *mut u8, size: usize) {
                drop(Vec::<u8>::from_raw_parts(ptr, 0, size));
            }
        }
    } else {
        quote! {
        }
    };

    let boxed_str = if boxed_str {
        quote! {
            #[no_mangle]
            pub unsafe extern fn __wbindgen_boxed_str_len(ptr: *mut String) -> usize {
                (*ptr).len()
            }

            #[no_mangle]
            pub unsafe extern fn __wbindgen_boxed_str_ptr(ptr: *mut String) -> *const u8 {
                (*ptr).as_ptr()
            }

            #[no_mangle]
            pub unsafe extern fn __wbindgen_boxed_str_free(ptr: *mut String) {
                drop(Box::from_raw(ptr));
            }
        }
    } else {
        quote! {
        }
    };

    let tokens = quote! {
        #malloc
        #boxed_str

        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static #generated_static_name: [u8; #generated_static_length] =
            *#generated_static_value;

        #[no_mangle]
        #[export_name = #export_name]
        pub extern fn #generated_name(#(#args),*) #ret_ty {
            #(#arg_conversions)*
            let #ret = #real_name(#(#converted_arguments),*);
            #convert_ret
        }
    };
    // println!("{}", tokens);
    tokens.to_tokens(into);
}

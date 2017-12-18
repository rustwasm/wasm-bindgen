#![feature(proc_macro)]

extern crate syn;
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

    let mut program = ast::Program {
        structs: Vec::new(),
        free_functions: Vec::new(),
    };

    for item in file.items.iter() {
        item.to_tokens(&mut ret);
        match *item {
            syn::Item::Fn(ref f) => {
                program.free_functions.push(ast::Function::from(f));
            }
            syn::Item::Struct(ref s) => {
                let s = ast::Struct::from(s);
                if program.structs.iter().any(|a| a.name == s.name) {
                    panic!("redefinition of struct: {}", s.name);
                }
                program.structs.push(s);
            }
            syn::Item::Impl(ref s) => program.push_impl(s),
            _ => panic!("unexpected item in bindgen macro"),
        }
    }

    for function in program.free_functions.iter() {
        bindgen_fn(function, &mut ret);
    }
    for s in program.structs.iter() {
        bindgen_struct(s, &mut ret);
    }

    static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
    let generated_static_name = format!("__WASM_BINDGEN_GENERATED{}",
                                        CNT.fetch_add(1, Ordering::SeqCst));
    let mut generated_static = String::from("wbg:");
    generated_static.push_str(&serde_json::to_string(&program.shared()).unwrap());
    let generated_static_value = syn::Lit {
        value: syn::LitKind::Other(Literal::byte_string(generated_static.as_bytes())),
        span: Default::default(),
    };
    let generated_static_length = generated_static.len();

    (quote! {
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static #generated_static_name: [u8; #generated_static_length] =
            *#generated_static_value;
    }).to_tokens(&mut ret);

    ret.into()
}

fn bindgen_fn(function: &ast::Function, into: &mut Tokens) {
    let export_name = function.export_name();
    let generated_name = function.rust_symbol();
    let mut args = vec![];
    let mut arg_conversions = vec![];
    let real_name = &function.name;
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
            ast::Type::ByValue(name) => {
                args.push(quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(quote! {
                    assert!(!#ident.is_null());
                    let #ident = unsafe {
                        (*#ident).borrow_mut();
                        Box::from_raw(#ident).into_inner()
                    };
                });
            }
            ast::Type::ByRef(name) => {
                args.push(quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(quote! {
                    assert!(!#ident.is_null());
                    let #ident = unsafe { (*#ident).borrow() };
                    let #ident = &*#ident;
                });
            }
            ast::Type::ByMutRef(name) => {
                args.push(quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(quote! {
                    assert!(!#ident.is_null());
                    let mut #ident = unsafe { (*#ident).borrow_mut() };
                    let #ident = &mut *#ident;
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
        Some(ast::Type::ByRef(_)) => panic!("can't return a borrowed ref"),
        Some(ast::Type::ByMutRef(_)) => panic!("can't return a borrowed ref"),
        Some(ast::Type::String) => {
            boxed_str = !BOXED_STR_GENERATED.swap(true, Ordering::SeqCst);
            ret_ty = quote! { -> *mut String };
            convert_ret = quote! { Box::into_raw(Box::new(#ret)) };
        }
        Some(ast::Type::ByValue(name)) => {
            ret_ty = quote! { -> *mut ::std::cell::RefCell<#name> };
            convert_ret = quote! {
                Box::into_raw(Box::new(::std::cell::RefCell<#ret>))
            };
        }
        None => {
            ret_ty = quote! {};
            convert_ret = quote! {};
        }
    }

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

fn bindgen_struct(s: &ast::Struct, into: &mut Tokens) {
    if s.ctor.is_none() {
        panic!("struct `{}` needs a `new` function to construct it", s.name);
    }
}

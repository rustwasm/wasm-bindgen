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
use proc_macro2::{Literal, Span};
use quote::{Tokens, ToTokens};

mod ast;

static MALLOC_GENERATED: AtomicBool = ATOMIC_BOOL_INIT;
static BOXED_STR_GENERATED: AtomicBool = ATOMIC_BOOL_INIT;

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(Span::call_site(), $($t)*))
}

#[proc_macro]
pub fn wasm_bindgen(input: TokenStream) -> TokenStream {
    // Parse the input as a list of Rust items, reusing the `syn::File` parser.
    let file = syn::parse::<syn::File>(input)
        .expect("expected a set of valid Rust items");

    let mut ret = Tokens::new();

    let mut program = ast::Program {
        structs: Vec::new(),
        free_functions: Vec::new(),
        imports: Vec::new(),
    };

    // Translate all input items into our own internal representation (the `ast`
    // module). We'll be panicking here on anything that we can't process

    for item in file.items.iter() {
        match *item {
            syn::Item::Fn(ref f) => {
                item.to_tokens(&mut ret);
                program.free_functions.push(ast::Function::from(f));
            }
            syn::Item::Struct(ref s) => {
                item.to_tokens(&mut ret);
                let s = ast::Struct::from(s);
                if program.structs.iter().any(|a| a.name == s.name) {
                    panic!("redefinition of struct: {}", s.name);
                }
                program.structs.push(s);
            }
            syn::Item::Impl(ref i) => {
                item.to_tokens(&mut ret);
                program.push_impl(i);
            }
            syn::Item::ForeignMod(ref f) => {
                program.push_foreign_mod(f);
            }
            _ => panic!("unexpected item in bindgen macro"),
        }
    }

    // Generate wrappers for all the items that we've found

    for function in program.free_functions.iter() {
        bindgen_fn(function, &mut ret);
    }
    for s in program.structs.iter() {
        bindgen_struct(s, &mut ret);
    }
    for i in program.imports.iter() {
        bindgen_import(i, &mut ret);
    }

    // Finally generate a static which will eventually be what lives in a custom
    // section of the wasm executable. For now it's just a plain old static, but
    // we'll eventually have it actually in its own section.

    static CNT: AtomicUsize = ATOMIC_USIZE_INIT;
    let generated_static_name = format!("__WASM_BINDGEN_GENERATED{}",
                                        CNT.fetch_add(1, Ordering::SeqCst));
    let generated_static_name = syn::Ident::from(generated_static_name);
    let mut generated_static = String::from("wbg:");
    generated_static.push_str(&serde_json::to_string(&program.shared()).unwrap());
    let generated_static_value = syn::Lit {
        value: syn::LitKind::Other(Literal::byte_string(generated_static.as_bytes())),
        span: Default::default(),
    };
    let generated_static_length = generated_static.len();

    (my_quote! {
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static #generated_static_name: [u8; #generated_static_length] =
            *#generated_static_value;
    }).to_tokens(&mut ret);

    // println!("{}", ret);

    ret.into()
}

fn bindgen_fn(function: &ast::Function, into: &mut Tokens) {
    bindgen(&function.free_function_export_name(),
            function.rust_symbol(None),
            Receiver::FreeFunction(function.name),
            &function.arguments,
            function.ret.as_ref(),
            into)
}

fn bindgen_struct(s: &ast::Struct, into: &mut Tokens) {
    for f in s.functions.iter() {
        bindgen_struct_fn(s, f, into);
    }
    for f in s.methods.iter() {
        bindgen_struct_method(s, f, into);
    }

    let name = &s.name;
    let free_fn = s.free_function();
    (my_quote! {
        #[no_mangle]
        pub unsafe extern fn #free_fn(ptr: *mut ::std::cell::RefCell<#name>) {
            assert!(!ptr.is_null());
            drop(Box::from_raw(ptr));
        }
    }).to_tokens(into);
}

fn bindgen_struct_fn(s: &ast::Struct, f: &ast::Function, into: &mut Tokens) {
    bindgen(&f.struct_function_export_name(s.name),
            f.rust_symbol(Some(s.name)),
            Receiver::StructFunction(s.name, f.name),
            &f.arguments,
            f.ret.as_ref(),
            into)
}

fn bindgen_struct_method(s: &ast::Struct, m: &ast::Method, into: &mut Tokens) {
    bindgen(&m.function.struct_function_export_name(s.name),
            m.function.rust_symbol(Some(s.name)),
            Receiver::StructMethod(s.name, m.mutable, m.function.name),
            &m.function.arguments,
            m.function.ret.as_ref(),
            into)
}

enum Receiver {
    FreeFunction(syn::Ident),
    StructFunction(syn::Ident, syn::Ident),
    StructMethod(syn::Ident, bool, syn::Ident),
}

fn bindgen(export_name: &syn::Lit,
           generated_name: syn::Ident,
           receiver: Receiver,
           arguments: &[ast::Type],
           ret_type: Option<&ast::Type>,
           into: &mut Tokens) {
    let mut args = vec![];
    let mut arg_conversions = vec![];
    let mut converted_arguments = vec![];
    let ret = syn::Ident::from("_ret");

    let mut malloc = false;
    let mut boxed_str = false;

    let mut offset = 0;
    if let Receiver::StructMethod(class, _, _) = receiver {
        args.push(my_quote! { me: *mut ::std::cell::RefCell<#class> });
        arg_conversions.push(my_quote! {
            assert!(!me.is_null());
            let me = unsafe { &*me };
        });
        offset = 1;
    }

    for (i, ty) in arguments.iter().enumerate() {
        let i = i + offset;
        let ident = syn::Ident::from(format!("arg{}", i));
        match *ty {
            ast::Type::Integer(i) => {
                args.push(my_quote! { #ident: #i });
            }
            ast::Type::BorrowedStr => {
                malloc = malloc || !MALLOC_GENERATED.swap(true, Ordering::SeqCst);
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(my_quote! { #ptr: *const u8 });
                args.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        let slice = ::std::slice::from_raw_parts(#ptr, #len);
                        ::std::str::from_utf8_unchecked(slice)
                    };
                });
            }
            ast::Type::String => {
                malloc = malloc || !MALLOC_GENERATED.swap(true, Ordering::SeqCst);
                let ptr = syn::Ident::from(format!("arg{}_ptr", i));
                let len = syn::Ident::from(format!("arg{}_len", i));
                args.push(my_quote! { #ptr: *mut u8 });
                args.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ident = unsafe {
                        let vec = ::std::vec::Vec::from_raw_parts(#ptr, #len, #len);
                        ::std::string::String::from_utf8_unchecked(vec)
                    };
                });
            }
            ast::Type::ByValue(name) => {
                args.push(my_quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(my_quote! {
                    assert!(!#ident.is_null());
                    let #ident = unsafe {
                        (*#ident).borrow_mut();
                        Box::from_raw(#ident).into_inner()
                    };
                });
            }
            ast::Type::ByRef(name) => {
                args.push(my_quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(my_quote! {
                    assert!(!#ident.is_null());
                    let #ident = unsafe { (*#ident).borrow() };
                    let #ident = &*#ident;
                });
            }
            ast::Type::ByMutRef(name) => {
                args.push(my_quote! { #ident: *mut ::std::cell::RefCell<#name> });
                arg_conversions.push(my_quote! {
                    assert!(!#ident.is_null());
                    let mut #ident = unsafe { (*#ident).borrow_mut() };
                    let #ident = &mut *#ident;
                });
            }
            ast::Type::JsObject => {
                args.push(my_quote! { #ident: u32 });
                arg_conversions.push(my_quote! {
                    let #ident = ::wasm_bindgen::JsObject::__from_idx(#ident);
                });
            }
            ast::Type::JsObjectRef => {
                args.push(my_quote! { #ident: u32 });
                arg_conversions.push(my_quote! {
                    let #ident = ::std::mem::ManuallyDrop::new(
                        ::wasm_bindgen::JsObject::__from_idx(#ident)
                    );
                    let #ident = &*#ident;
                });
            }
        }
        converted_arguments.push(my_quote! { #ident });
    }
    let ret_ty;
    let convert_ret;
    match ret_type {
        Some(&ast::Type::Integer(i)) => {
            ret_ty = my_quote! { -> #i };
            convert_ret = my_quote! { #ret };
        }
        Some(&ast::Type::BorrowedStr) => panic!("can't return a borrowed string"),
        Some(&ast::Type::ByRef(_)) => panic!("can't return a borrowed ref"),
        Some(&ast::Type::ByMutRef(_)) => panic!("can't return a borrowed ref"),
        Some(&ast::Type::String) => {
            boxed_str = !BOXED_STR_GENERATED.swap(true, Ordering::SeqCst);
            ret_ty = my_quote! { -> *mut String };
            convert_ret = my_quote! { Box::into_raw(Box::new(#ret)) };
        }
        Some(&ast::Type::ByValue(name)) => {
            ret_ty = my_quote! { -> *mut ::std::cell::RefCell<#name> };
            convert_ret = my_quote! {
                Box::into_raw(Box::new(::std::cell::RefCell::new(#ret)))
            };
        }
        Some(&ast::Type::JsObject) => {
            ret_ty = my_quote! { -> u32 };
            convert_ret = my_quote! {
                ::wasm_bindgen::JsObject::__into_idx(#ret)
            };
        }
        Some(&ast::Type::JsObjectRef) => {
            panic!("can't return a borrowed ref");
        }
        None => {
            ret_ty = my_quote! {};
            convert_ret = my_quote! {};
        }
    }

    let malloc = if malloc {
        my_quote! {
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
        my_quote! {
        }
    };

    let boxed_str = if boxed_str {
        my_quote! {
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
        my_quote! {
        }
    };

    let tokens = my_quote! {
        #malloc
        #boxed_str

        #[export_name = #export_name]
        #[allow(non_snake_case)]
        pub extern fn #generated_name(#(#args),*) #ret_ty {
            #(#arg_conversions)*
            let #ret = #receiver(#(#converted_arguments),*);
            #convert_ret
        }
    };
    tokens.to_tokens(into);
}

impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Receiver::FreeFunction(name) => name.to_tokens(tokens),
            Receiver::StructFunction(s, name) => {
                s.to_tokens(tokens);
                syn::tokens::Colon2::default().to_tokens(tokens);
                name.to_tokens(tokens);
            }
            Receiver::StructMethod(_, mutable, name) => {
                (my_quote! { me }).to_tokens(tokens);
                syn::tokens::Dot::default().to_tokens(tokens);
                if mutable {
                    syn::Ident::from("borrow_mut").to_tokens(tokens);
                } else {
                    syn::Ident::from("borrow").to_tokens(tokens);
                }
                tokens.append_delimited("(", Default::default(), |_| ());
                syn::tokens::Dot::default().to_tokens(tokens);
                name.to_tokens(tokens);
            }
        }
    }
}

fn bindgen_import(import: &ast::Import, tokens: &mut Tokens) {
    let vis = &import.vis;
    let ret = &import.decl.output;
    let name = &import.ident;
    let fn_token = &import.decl.fn_token;
    let arguments = &import.decl.inputs;

    let mut abi_argument_names = Vec::new();
    let mut abi_arguments = Vec::new();
    let mut arg_conversions = Vec::new();
    let ret_ident = syn::Ident::from("_ret");

    let names = import.decl.inputs
        .iter()
        .map(|i| i.into_item())
        .map(|arg| {
            match *arg {
                syn::FnArg::Captured(ref c) => c,
                _ => panic!("arguments cannot be `self` or ignored"),
            }
        })
        .map(|arg| {
            match arg.pat {
                syn::Pat::Ident(syn::PatIdent {
                    mode: syn::BindingMode::ByValue(_),
                    ident,
                    subpat: None,
                    ..
                }) => {
                    ident
                }
                _ => panic!("unsupported pattern in foreign function"),
            }
        });

    for (ty, name) in import.function.arguments.iter().zip(names) {
        match *ty {
            ast::Type::Integer(i) => {
                abi_argument_names.push(name);
                abi_arguments.push(my_quote! { #name: #i });
                arg_conversions.push(my_quote! {});
            }
            ast::Type::BorrowedStr => {
                let ptr = syn::Ident::from(format!("{}_ptr", name));
                let len = syn::Ident::from(format!("{}_len", name));
                abi_argument_names.push(ptr);
                abi_argument_names.push(len);
                abi_arguments.push(my_quote! { #ptr: *const u8 });
                abi_arguments.push(my_quote! { #len: usize });
                arg_conversions.push(my_quote! {
                    let #ptr = #name.as_ptr();
                    let #len = #name.len();
                });
            }
            ast::Type::JsObject => {
                abi_argument_names.push(name);
                abi_arguments.push(my_quote! { #name: u32 });
                arg_conversions.push(my_quote! {
                    let #name = ::wasm_bindgen::JsObject::__into_idx(#name);
                });
            }
            ast::Type::JsObjectRef => {
                abi_argument_names.push(name);
                abi_arguments.push(my_quote! { #name: u32 });
                arg_conversions.push(my_quote! {
                    let #name = ::wasm_bindgen::JsObject::__get_idx(#name);
                });
            }
            ast::Type::String => panic!("can't use `String` in foreign functions"),
            ast::Type::ByValue(_name) |
            ast::Type::ByRef(_name) |
            ast::Type::ByMutRef(_name) => {
                panic!("can't use struct types in foreign functions yet");
            }
        }
    }
    let abi_ret;
    let convert_ret;
    match import.function.ret {
        Some(ast::Type::Integer(i)) => {
            abi_ret = my_quote! { #i };
            convert_ret = my_quote! { #ret_ident };
        }
        Some(ast::Type::JsObject) => {
            abi_ret = my_quote! { u32 };
            convert_ret = my_quote! {
                ::wasm_bindgen::JsObject::__from_idx(#ret_ident)
            };
        }
        Some(ast::Type::JsObjectRef) => panic!("can't return a borrowed ref"),
        Some(ast::Type::BorrowedStr) => panic!("can't return a borrowed string"),
        Some(ast::Type::ByRef(_)) => panic!("can't return a borrowed ref"),
        Some(ast::Type::ByMutRef(_)) => panic!("can't return a borrowed ref"),
        Some(ast::Type::String) => panic!("can't return a string in foreign functions"),
        Some(ast::Type::ByValue(_)) => panic!("can't return a struct in a foreign function"),
        None => {
            abi_ret = my_quote! { () };
            convert_ret = my_quote! {};
        }
    }

    (quote! {
        #vis #fn_token #name(#arguments) #ret {
            extern {
                fn #name(#(#abi_arguments),*) -> #abi_ret;
            }
            unsafe {
                #(#arg_conversions)*
                let #ret_ident = #name(#(#abi_argument_names),*);
                #convert_ret
            }
        }
    }).to_tokens(tokens);
}

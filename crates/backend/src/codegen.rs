use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use ast;
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use serde_json;
use shared;
use syn;

fn to_ident_name(s: &str) -> Cow<str> {
    if s.chars().all(|c| match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => true,
        _ => false,
    }) {
        return Cow::from(s);
    }

    Cow::from(
        s.chars()
            .map(|c| match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => c,
                _ => '_',
            })
            .collect::<String>(),
    )
}

impl ToTokens for ast::Program {
    // Generate wrappers for all the items that we've found
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for export in self.exports.iter() {
            export.to_tokens(tokens);
        }
        for s in self.structs.iter() {
            s.to_tokens(tokens);
        }
        let mut types = HashSet::new();
        for i in self.imports.iter() {
            if let ast::ImportKind::Type(t) = &i.kind {
                types.insert(t.name.clone());
            }
        }
        for i in self.imports.iter() {
            DescribeImport(&i.kind).to_tokens(tokens);

            if let Some(ns) = &i.js_namespace {
                if types.contains(ns) && i.kind.fits_on_impl() {
                    let kind = &i.kind;
                    (quote! { impl #ns { #kind } }).to_tokens(tokens);
                    continue;
                }
            }

            i.kind.to_tokens(tokens);
        }
        for e in self.enums.iter() {
            e.to_tokens(tokens);
        }
        for a in self.type_aliases.iter() {
            a.to_tokens(tokens);
        }

        // Generate a static which will eventually be what lives in a custom section
        // of the wasm executable. For now it's just a plain old static, but we'll
        // eventually have it actually in its own section.

        static CNT: AtomicUsize = ATOMIC_USIZE_INIT;

        let crate_name = env::var("CARGO_PKG_NAME").expect("should have CARGO_PKG_NAME env var");
        let crate_vers =
            env::var("CARGO_PKG_VERSION").expect("should have CARGO_PKG_VERSION env var");

        let generated_static_name = format!(
            "__WASM_BINDGEN_GENERATED_{}_{}_{}",
            to_ident_name(&crate_name),
            to_ident_name(&crate_vers),
            CNT.fetch_add(1, Ordering::SeqCst)
        );
        let generated_static_name = Ident::new(&generated_static_name, Span::call_site());

        let description = serde_json::to_string(&self.shared()).unwrap();

        // Each JSON blob is prepended with the length of the JSON blob so when
        // all these sections are concatenated in the final wasm file we know
        // how to extract all the JSON pieces, so insert the byte length here.
        let generated_static_length = description.len() + 4;
        let mut bytes = vec![
            (description.len() >> 0) as u8,
            (description.len() >> 8) as u8,
            (description.len() >> 16) as u8,
            (description.len() >> 24) as u8,
        ];
        bytes.extend_from_slice(description.as_bytes());
        let generated_static_value = syn::LitByteStr::new(&bytes, Span::call_site());

        (quote! {
            #[allow(non_upper_case_globals)]
            #[wasm_custom_section = "__wasm_bindgen_unstable"]
            const #generated_static_name: [u8; #generated_static_length] =
                *#generated_static_value;
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let name_str = name.to_string();
        let name_len = name_str.len() as u32;
        let name_chars = name_str.chars().map(|c| c as u32);
        let new_fn = Ident::new(&shared::new_function(&name_str), Span::call_site());
        let free_fn = Ident::new(&shared::free_function(&name_str), Span::call_site());
        (quote! {
            impl ::wasm_bindgen::describe::WasmDescribe for #name {
                fn describe() {
                    use wasm_bindgen::__wbindgen_if_not_std;
                    __wbindgen_if_not_std! {
                        compile_error! {
                            "exporting a class to JS requires the `std` feature to \
                             be enabled in the `wasm-bindgen` crate"
                        }
                    }
                    use wasm_bindgen::describe::*;
                    inform(RUST_STRUCT);
                    inform(#name_len);
                    #(inform(#name_chars);)*
                }
            }

            impl ::wasm_bindgen::convert::IntoWasmAbi for #name {
                type Abi = u32;

                fn into_abi(self, _extra: &mut ::wasm_bindgen::convert::Stack)
                    -> u32
                {
                    use wasm_bindgen::__rt::std::boxed::Box;
                    use wasm_bindgen::__rt::WasmRefCell;
                    Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
                }
            }

            impl ::wasm_bindgen::convert::FromWasmAbi for #name {
                type Abi = u32;

                unsafe fn from_abi(js: u32, _extra: &mut ::wasm_bindgen::convert::Stack)
                    -> Self
                {
                    use wasm_bindgen::__rt::std::boxed::Box;
                    use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};

                    let ptr = js as *mut WasmRefCell<#name>;
                    assert_not_null(ptr);
                    let js = Box::from_raw(ptr);
                    js.borrow_mut(); // make sure no one's borrowing
                    js.into_inner()
                }
            }

            impl ::wasm_bindgen::__rt::core::convert::From<#name> for
                ::wasm_bindgen::JsValue
            {
                #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                fn from(value: #name) -> Self {
                    let ptr = ::wasm_bindgen::convert::IntoWasmAbi::into_abi(
                        value,
                        unsafe { &mut ::wasm_bindgen::convert::GlobalStack::new() },
                    );

                    #[wasm_import_module = "__wbindgen_placeholder__"]
                    extern {
                        fn #new_fn(ptr: u32) -> u32;
                    }

                    unsafe {
                        <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi>
                            ::from_abi(
                                #new_fn(ptr),
                                &mut ::wasm_bindgen::convert::GlobalStack::new(),
                            )
                    }
                }

                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                fn from(_value: #name) -> Self {
                    panic!("cannot convert to JsValue outside of the wasm target")
                }
            }

            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            #[no_mangle]
            pub unsafe extern fn #free_fn(ptr: u32) {
                <#name as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    ptr,
                    &mut ::wasm_bindgen::convert::GlobalStack::new(),
                );
            }

            impl ::wasm_bindgen::convert::RefFromWasmAbi for #name {
                type Abi = u32;
                type Anchor = ::wasm_bindgen::__rt::Ref<'static, #name>;

                unsafe fn ref_from_abi(
                    js: Self::Abi,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::Anchor {
                    let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                    ::wasm_bindgen::__rt::assert_not_null(js);
                    (*js).borrow()
                }
            }

            impl ::wasm_bindgen::convert::RefMutFromWasmAbi for #name {
                type Abi = u32;
                type Anchor = ::wasm_bindgen::__rt::RefMut<'static, #name>;

                unsafe fn ref_mut_from_abi(
                    js: Self::Abi,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::Anchor {
                    let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<#name>;
                    ::wasm_bindgen::__rt::assert_not_null(js);
                    (*js).borrow_mut()
                }
            }
        }).to_tokens(tokens);

        for field in self.fields.iter() {
            field.to_tokens(tokens);
        }
    }
}

impl ToTokens for ast::StructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let struct_name = &self.struct_name;
        let ty = &self.ty;
        let getter = &self.getter;
        let setter = &self.setter;
        let desc = Ident::new(
            &format!("__wbindgen_describe_{}", getter),
            Span::call_site(),
        );
        (quote! {
            #[no_mangle]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            pub unsafe extern fn #getter(js: u32)
                -> <#ty as ::wasm_bindgen::convert::IntoWasmAbi>::Abi
            {
                use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                use wasm_bindgen::convert::{GlobalStack, IntoWasmAbi};

                fn assert_copy<T: Copy>(){}
                assert_copy::<#ty>();

                let js = js as *mut WasmRefCell<#struct_name>;
                assert_not_null(js);
                let val = (*js).borrow().#name;
                <#ty as IntoWasmAbi>::into_abi(
                    val,
                    &mut GlobalStack::new(),
                )
            }

            #[no_mangle]
            #[doc(hidden)]
            pub extern fn #desc() {
                use wasm_bindgen::describe::*;
                <#ty as WasmDescribe>::describe();
            }
        }).to_tokens(tokens);

        if self.opts.readonly() {
            return;
        }

        (quote! {
            #[no_mangle]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            pub unsafe extern fn #setter(
                js: u32,
                val: <#ty as ::wasm_bindgen::convert::FromWasmAbi>::Abi,
            ) {
                use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                use wasm_bindgen::convert::{GlobalStack, FromWasmAbi};

                let js = js as *mut WasmRefCell<#struct_name>;
                assert_not_null(js);
                let val = <#ty as FromWasmAbi>::from_abi(
                    val,
                    &mut GlobalStack::new(),
                );
                (*js).borrow_mut().#name = val;
            }
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Export {
    fn to_tokens(self: &ast::Export, into: &mut TokenStream) {
        let generated_name = self.rust_symbol();
        let export_name = self.export_name();
        let mut args = vec![];
        let mut arg_conversions = vec![];
        let mut converted_arguments = vec![];
        let ret = Ident::new("_ret", Span::call_site());

        let mut offset = 0;
        if self.method {
            let class = self.class.as_ref().unwrap();
            args.push(quote! { me: *mut ::wasm_bindgen::__rt::WasmRefCell<#class> });
            arg_conversions.push(quote! {
                ::wasm_bindgen::__rt::assert_not_null(me);
                let me = unsafe { &*me };
            });
            offset = 1;
        }

        for (i, syn::ArgCaptured { ty, .. }) in self.function.arguments.iter().enumerate() {
            let i = i + offset;
            let ident = Ident::new(&format!("arg{}", i), Span::call_site());
            match *ty {
                syn::Type::Reference(syn::TypeReference {
                    mutability: Some(_),
                    ref elem,
                    ..
                }) => {
                    args.push(quote! {
                        #ident: <#elem as ::wasm_bindgen::convert::RefMutFromWasmAbi>::Abi
                    });
                    arg_conversions.push(quote! {
                        let mut #ident = unsafe {
                            <#elem as ::wasm_bindgen::convert::RefMutFromWasmAbi>
                                ::ref_mut_from_abi(#ident, &mut __stack)
                        };
                        let #ident = &mut *#ident;
                    });
                }
                syn::Type::Reference(syn::TypeReference { ref elem, .. }) => {
                    args.push(quote! {
                        #ident: <#elem as ::wasm_bindgen::convert::RefFromWasmAbi>::Abi
                    });
                    arg_conversions.push(quote! {
                        let #ident = unsafe {
                            <#elem as ::wasm_bindgen::convert::RefFromWasmAbi>
                                ::ref_from_abi(#ident, &mut __stack)
                        };
                        let #ident = &*#ident;
                    });
                }
                _ => {
                    args.push(quote! {
                        #ident: <#ty as ::wasm_bindgen::convert::FromWasmAbi>::Abi
                    });
                    arg_conversions.push(quote! {
                        let #ident = unsafe {
                            <#ty as ::wasm_bindgen::convert::FromWasmAbi>
                                ::from_abi(#ident, &mut __stack)
                        };
                    });
                }
            }
            converted_arguments.push(quote! { #ident });
        }
        let ret_ty;
        let convert_ret;
        match &self.function.ret {
            Some(syn::Type::Reference(_)) => panic!("can't return a borrowed ref"),
            Some(ty) => {
                ret_ty = quote! {
                    -> <#ty as ::wasm_bindgen::convert::IntoWasmAbi>::Abi
                };
                convert_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::IntoWasmAbi>
                        ::into_abi(#ret, &mut unsafe {
                            ::wasm_bindgen::convert::GlobalStack::new()
                        })
                };
            }
            None => {
                ret_ty = quote!();
                convert_ret = quote!();
            }
        }
        let describe_ret = match &self.function.ret {
            Some(ty) => {
                quote! {
                    inform(1);
                    <#ty as WasmDescribe>::describe();
                }
            }
            None => quote! { inform(0); },
        };

        let name = &self.function.name;
        let receiver = match &self.class {
            Some(_) if self.method => {
                if self.mutable {
                    quote! { me.borrow_mut().#name }
                } else {
                    quote! { me.borrow().#name }
                }
            }
            Some(class) => quote! { #class::#name },
            None => quote!{ #name },
        };
        let descriptor_name = format!("__wbindgen_describe_{}", export_name);
        let descriptor_name = Ident::new(&descriptor_name, Span::call_site());
        let nargs = self.function.arguments.len() as u32;
        let argtys = self.function.arguments.iter().map(|arg| &arg.ty);

        let tokens = quote! {
            #[export_name = #export_name]
            #[allow(non_snake_case)]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            pub extern fn #generated_name(#(#args),*) #ret_ty {
                ::wasm_bindgen::__rt::link_this_library();
                let #ret = {
                    let mut __stack = unsafe {
                        ::wasm_bindgen::convert::GlobalStack::new()
                    };
                    #(#arg_conversions)*
                    #receiver(#(#converted_arguments),*)
                };
                #convert_ret
            }

            // In addition to generating the shim function above which is what
            // our generated JS will invoke, we *also* generate a "descriptor"
            // shim. This descriptor shim uses the `WasmDescribe` trait to
            // programmatically describe the type signature of the generated
            // shim above. This in turn is then used to inform the
            // `wasm-bindgen` CLI tool exactly what types and such it should be
            // using in JS.
            //
            // Note that this descriptor function is a purely an internal detail
            // of `#[wasm_bindgen]` and isn't intended to be exported to anyone
            // or actually part of the final was binary. Additionally, this is
            // literally executed when the `wasm-bindgen` tool executes.
            //
            // In any case, there's complications in `wasm-bindgen` to handle
            // this, but the tl;dr; is that this is stripped from the final wasm
            // binary along with anything it references.
            #[no_mangle]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            #[doc(hidden)]
            pub extern fn #descriptor_name() {
                use wasm_bindgen::describe::*;
                inform(FUNCTION);
                inform(#nargs);
                #(<#argtys as WasmDescribe>::describe();)*
                #describe_ret
            }
        };
        tokens.to_tokens(into);
    }
}

impl ToTokens for ast::ImportKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            ast::ImportKind::Function(ref f) => f.to_tokens(tokens),
            ast::ImportKind::Static(ref s) => s.to_tokens(tokens),
            ast::ImportKind::Type(ref t) => t.to_tokens(tokens),
        }
    }
}

impl ToTokens for ast::ImportType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = &self.vis;
        let name = &self.name;
        (quote! {
            #[allow(bad_style)]
            #vis struct #name {
                obj: ::wasm_bindgen::JsValue,
            }

            impl ::wasm_bindgen::describe::WasmDescribe for #name {
                fn describe() {
                    ::wasm_bindgen::JsValue::describe();
                }
            }

            impl ::wasm_bindgen::convert::IntoWasmAbi for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::IntoWasmAbi>::Abi;

                fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
                    self.obj.into_abi(extra)
                }
            }

            impl ::wasm_bindgen::convert::FromWasmAbi for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::FromWasmAbi>::Abi;

                unsafe fn from_abi(
                    js: Self::Abi,
                    extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self {
                    #name {
                        obj: ::wasm_bindgen::JsValue::from_abi(js, extra),
                    }
                }
            }

            impl<'a> ::wasm_bindgen::convert::IntoWasmAbi for &'a #name {
                type Abi = <&'a ::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::IntoWasmAbi>::Abi;

                fn into_abi(self, extra: &mut ::wasm_bindgen::convert::Stack) -> Self::Abi {
                    (&self.obj).into_abi(extra)
                }
            }

            impl ::wasm_bindgen::convert::RefFromWasmAbi for #name {
                type Abi = <::wasm_bindgen::JsValue as
                    ::wasm_bindgen::convert::RefFromWasmAbi>::Abi;
                type Anchor = ::wasm_bindgen::__rt::core::mem::ManuallyDrop<#name>;

                unsafe fn ref_from_abi(
                    js: Self::Abi,
                    extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self::Anchor {
                    let tmp = <::wasm_bindgen::JsValue as ::wasm_bindgen::convert::RefFromWasmAbi>
                        ::ref_from_abi(js, extra);
                    ::wasm_bindgen::__rt::core::mem::ManuallyDrop::new(#name {
                        obj: ::wasm_bindgen::__rt::core::mem::ManuallyDrop::into_inner(tmp),
                    })
                }
            }

            impl From<::wasm_bindgen::JsValue> for #name {
                fn from(obj: ::wasm_bindgen::JsValue) -> #name {
                    #name { obj }
                }
            }

            impl From<#name> for ::wasm_bindgen::JsValue {
                fn from(obj: #name) -> ::wasm_bindgen::JsValue {
                    obj.obj
                }
            }
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::ImportFunction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut class_ty = None;
        let mut is_method = false;
        match self.kind {
            ast::ImportFunctionKind::Method { ref ty, .. } => {
                is_method = true;
                class_ty = Some(ty);
            }
            ast::ImportFunctionKind::JsConstructor { ref ty, .. } => {
                class_ty = Some(ty);
            }
            ast::ImportFunctionKind::Normal => {}
        }
        let vis = &self.function.rust_vis;
        let ret = match &self.function.ret {
            Some(ty) => quote! { -> #ty },
            None => quote!(),
        };

        let mut abi_argument_names = Vec::new();
        let mut abi_arguments = Vec::new();
        let mut arg_conversions = Vec::new();
        let ret_ident = Ident::new("_ret", Span::call_site());

        for (i, syn::ArgCaptured { pat, ty, .. }) in self.function.arguments.iter().enumerate() {
            let name = match pat {
                syn::Pat::Ident(syn::PatIdent {
                    by_ref: None,
                    ident,
                    subpat: None,
                    ..
                }) => ident.clone(),
                _ => panic!("unsupoported pattern in foreign function"),
            };

            abi_argument_names.push(name.clone());
            abi_arguments.push(quote! {
                #name: <#ty as ::wasm_bindgen::convert::IntoWasmAbi>::Abi
            });
            let var = if i == 0 && is_method {
                quote! { self }
            } else {
                quote! { #name }
            };
            arg_conversions.push(quote! {
                let #name = <#ty as ::wasm_bindgen::convert::IntoWasmAbi>
                    ::into_abi(#var, &mut __stack);
            });
        }
        let abi_ret;
        let mut convert_ret;
        match &self.js_ret {
            Some(syn::Type::Reference(_)) => {
                panic!("cannot return references in imports yet");
            }
            Some(ref ty) => {
                abi_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::FromWasmAbi>::Abi
                };
                convert_ret = quote! {
                    <#ty as ::wasm_bindgen::convert::FromWasmAbi>
                        ::from_abi(
                            #ret_ident,
                            &mut ::wasm_bindgen::convert::GlobalStack::new(),
                        )
                };
            }
            None => {
                abi_ret = quote! { () };
                convert_ret = quote! { () };
            }
        }

        let mut exceptional_ret = quote!();
        let exn_data = if self.function.opts.catch() {
            let exn_data = Ident::new("exn_data", Span::call_site());
            let exn_data_ptr = Ident::new("exn_data_ptr", Span::call_site());
            abi_argument_names.push(exn_data_ptr.clone());
            abi_arguments.push(quote! { #exn_data_ptr: *mut u32 });
            convert_ret = quote! { Ok(#convert_ret) };
            exceptional_ret = quote! {
                if #exn_data[0] == 1 {
                    return Err(
                        <
                            ::wasm_bindgen::JsValue as ::wasm_bindgen::convert::FromWasmAbi
                        >::from_abi(#exn_data[1], &mut ::wasm_bindgen::convert::GlobalStack::new())
                    )
                }
            };
            quote! {
                let mut #exn_data = [0; 2];
                let #exn_data_ptr = #exn_data.as_mut_ptr();
            }
        } else {
            quote!()
        };

        let rust_name = &self.rust_name;
        let import_name = &self.shim;
        let attrs = &self.function.rust_attrs;

        let arguments = if is_method {
            &self.function.arguments[1..]
        } else {
            &self.function.arguments[..]
        };

        let me = if is_method {
            quote! { &self, }
        } else {
            quote!()
        };

        let invocation = quote! {
            #(#attrs)*
            #[allow(bad_style)]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            #vis extern fn #rust_name(#me #(#arguments),*) #ret {
                ::wasm_bindgen::__rt::link_this_library();
                #[wasm_import_module = "__wbindgen_placeholder__"]
                extern {
                    fn #import_name(#(#abi_arguments),*) -> #abi_ret;
                }
                unsafe {
                    #exn_data
                    let #ret_ident = {
                        let mut __stack = ::wasm_bindgen::convert::GlobalStack::new();
                        #(#arg_conversions)*
                        #import_name(#(#abi_argument_names),*)
                    };
                    #exceptional_ret
                    #convert_ret
                }
            }

            #(#attrs)*
            #[allow(bad_style, unused_variables)]
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            #vis extern fn #rust_name(#me #(#arguments),*) #ret {
                panic!("cannot call wasm-bindgen imported functions on \
                        non-wasm targets");
            }

        };

        if let Some(class) = class_ty {
            (quote! {
                impl #class {
                    #invocation
                }
            }).to_tokens(tokens);
        } else {
            invocation.to_tokens(tokens);
        }
    }
}

// See comment above in ast::Export for what's going on here.
struct DescribeImport<'a>(&'a ast::ImportKind);

impl<'a> ToTokens for DescribeImport<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let f = match *self.0 {
            ast::ImportKind::Function(ref f) => f,
            ast::ImportKind::Static(_) => return,
            ast::ImportKind::Type(_) => return,
        };
        let describe_name = format!("__wbindgen_describe_{}", f.shim);
        let describe_name = Ident::new(&describe_name, Span::call_site());
        let argtys = f.function.arguments.iter().map(|arg| &arg.ty);
        let nargs = f.function.arguments.len() as u32;
        let inform_ret = match &f.js_ret {
            Some(ref t) => quote! { inform(1); <#t as WasmDescribe>::describe(); },
            None => quote! { inform(0); },
        };
        (quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            #[doc(hidden)]
            pub extern fn #describe_name() {
                use wasm_bindgen::describe::*;
                inform(FUNCTION);
                inform(#nargs);
                #(<#argtys as WasmDescribe>::describe();)*
                #inform_ret
            }
        }).to_tokens(tokens);
    }
}

impl ToTokens for ast::Enum {
    fn to_tokens(&self, into: &mut TokenStream) {
        let enum_name = &self.name;
        let cast_clauses = self.variants.iter().map(|variant| {
            let variant_name = &variant.name;
            quote! {
                if js == #enum_name::#variant_name as u32 {
                    #enum_name::#variant_name
                }
            }
        });
        (quote! {
            impl ::wasm_bindgen::convert::IntoWasmAbi for #enum_name {
                type Abi = u32;

                fn into_abi(self, _extra: &mut ::wasm_bindgen::convert::Stack) -> u32 {
                    self as u32
                }
            }

            impl ::wasm_bindgen::convert::FromWasmAbi for #enum_name {
                type Abi = u32;

                unsafe fn from_abi(
                    js: u32,
                    _extra: &mut ::wasm_bindgen::convert::Stack,
                ) -> Self {
                    #(#cast_clauses else)* {
                        ::wasm_bindgen::throw("invalid enum value passed")
                    }
                }
            }

            impl ::wasm_bindgen::describe::WasmDescribe for #enum_name {
                fn describe() {
                    use wasm_bindgen::describe::*;
                    inform(ENUM);
                }
            }
        }).to_tokens(into);
    }
}

impl ToTokens for ast::ImportStatic {
    fn to_tokens(&self, into: &mut TokenStream) {
        let name = &self.rust_name;
        let ty = &self.ty;
        let shim_name = &self.shim;
        let vis = &self.vis;
        (quote! {
            #[allow(bad_style)]
            #vis static #name: ::wasm_bindgen::JsStatic<#ty> = {
                #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                fn init() -> #ty {
                    #[wasm_import_module = "__wbindgen_placeholder__"]
                    extern {
                        fn #shim_name() -> <#ty as ::wasm_bindgen::convert::FromWasmAbi>::Abi;
                    }
                    unsafe {
                        <#ty as ::wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            #shim_name(),
                            &mut ::wasm_bindgen::convert::GlobalStack::new(),
                        )

                    }
                }
                #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
                fn init() -> #ty {
                    panic!("cannot access imported statics on non-wasm targets")
                }
                static mut _VAL: ::wasm_bindgen::__rt::core::cell::UnsafeCell<Option<#ty>> =
                    ::wasm_bindgen::__rt::core::cell::UnsafeCell::new(None);
                ::wasm_bindgen::JsStatic {
                    __inner: unsafe { &_VAL },
                    __init: init,
                }
            };
        }).to_tokens(into);
    }
}

impl ToTokens for ast::TypeAlias {
    fn to_tokens(&self, into: &mut TokenStream) {
        let vis = &self.vis;
        let dest = &self.dest;
        let src = &self.src;
        (quote! {
            #[allow(non_camel_case_types)]
            #vis type #dest = #src;
        }).to_tokens(into);
    }
}

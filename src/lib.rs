use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn autovec(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);

    let preamble = get_preamble(&ast.sig);
    let fn_generics = ast.sig.generics;

    let new_ident = ast.sig.ident;
    let old_ident = syn::Ident::new(&format!("_{}_autogenerated_autovec", new_ident), new_ident.span());

    let inputs: Vec<_> = ast.sig.inputs.iter().map(|f| match f {
        syn::FnArg::Typed(p) => p,
        _ => panic!("self not allowed"),
    }).collect();

    // edge case santitzation
    // no input args
    if inputs.len() == 0 {
        panic!("expected at least one argument")
    }
    // variadic input
    if ast.sig.variadic.is_some() {
        panic!("variadic input isnt supported");
    }

    let orig_return_type = &ast.sig.output;
    let return_type = match &ast.sig.output {
        syn::ReturnType::Type(_r, t) => quote! { Vec<#t> },
        _ => panic!("Function need to return a type"),
    };

    let transformed_args = get_transformed_args(inputs);
    let old_func_arg_names = transformed_args.0;
    let new_func_arg_names = transformed_args.1;
    let old_func_arg_types = transformed_args.2;
    let new_func_arg_types = transformed_args.3;

    let first_arg_name = &new_func_arg_names[0];

    let block = ast.block;

    TokenStream::from(quote! {
        #preamble fn #old_ident #fn_generics (#(#old_func_arg_names : #old_func_arg_types, )*) #orig_return_type #block

        #preamble fn #new_ident #fn_generics(#(#new_func_arg_names : #new_func_arg_types, )*) -> #return_type {
            let n = #first_arg_name.len();
            #(
                if n != #new_func_arg_names.len() {
                    panic!(concat!(stringify!(#new_func_arg_names), " len does not match with other vectors"));
                }
            )*
            let mut ret: #return_type = Vec::with_capacity(n);

            for i in 0..n {
                ret.push(#old_ident( #(&#new_func_arg_names[i],)* ));
            }
            return ret;
        }
    })
}

fn get_preamble(sig: &syn::Signature) -> TokenStream2 {
    let constness = sig.constness;
    let asyncness = sig.asyncness;
    quote! {
        #constness#asyncness
    }
}

fn get_transformed_args(input: Vec<&syn::PatType>) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<TokenStream2>, Vec<TokenStream2>) {
    let no_args = input.len();
    let mut old_func_arg_names: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut new_func_arg_names: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut new_func_arg_types: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut old_func_arg_types: Vec<TokenStream2> = Vec::with_capacity(no_args);

    for arg in input {
        match *arg.pat {
            syn::Pat::Ident(ref i) => {
                let t = &i.ident;
                old_func_arg_names.push(quote! { #t });
                let argname = syn::Ident::new(&format!("{}_vec", t), t.span());
                new_func_arg_names.push(quote! { #argname });
            },
            syn::Pat::Tuple(ref t) => {
                old_func_arg_names.push(quote! { #t });
                new_func_arg_names.push(quote! { arg1 })
            },
            syn::Pat::Struct(ref t) => {
                old_func_arg_names.push(quote! { #t });
                new_func_arg_names.push(quote! { arg1 })
            },
            _ => unimplemented!(),
        }
        let ty = &arg.ty;
        old_func_arg_types.push(quote! { &#ty });
        new_func_arg_types.push(quote! { Vec<#ty> });
    }

    return (old_func_arg_names, new_func_arg_names, old_func_arg_types, new_func_arg_types)
}

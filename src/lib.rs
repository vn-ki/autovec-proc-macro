use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn autovec(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);

    let preamble = get_preamble(&ast.sig);
    let fn_generics = &ast.sig.generics;

    let new_ident = &ast.sig.ident;

    let inputs: Vec<_> = get_args(&ast);

    confirm_fn_valid(&ast);

    let orig_return_type = get_return_type(&ast);
    let return_type = get_new_return_type(orig_return_type);

    let transformed_args = get_transformed_args(inputs);
    let old_func_arg_names = transformed_args.0;
    let new_func_arg_names = transformed_args.1;
    let new_func_arg_types = transformed_args.2;

    let first_arg_name = &new_func_arg_names[0];
    let rest_arg_names = &new_func_arg_names[1..];
    let zip_map_stmt = get_zip_map_stmts(&new_func_arg_names, &old_func_arg_names, &ast);

    TokenStream::from(quote! {
        #preamble fn #new_ident #fn_generics(#(#new_func_arg_names : #new_func_arg_types, )*) -> #return_type {
            let n = #first_arg_name.len();
            #(
                if n != #rest_arg_names.len() {
                    panic!(concat!(stringify!(#new_func_arg_names), " len does not match with other vectors"));
                }
            )*
            let ret = #zip_map_stmt.collect();

            return ret;
        }
    })
}

// confirms that the function is valid
fn confirm_fn_valid(ast: &syn::ItemFn) {
    // variadic input
    if ast.sig.variadic.is_some() {
        panic!("variadic input isnt supported");
    }
}

fn get_preamble(sig: &syn::Signature) -> TokenStream2 {
    let constness = sig.constness;
    let asyncness = sig.asyncness;
    quote! {
        #constness#asyncness
    }
}

fn get_transformed_args(
    input: Vec<&syn::PatType>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<TokenStream2>) {
    let no_args = input.len();
    let mut old_func_arg_names: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut new_func_arg_names: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut new_func_arg_types: Vec<TokenStream2> = Vec::with_capacity(no_args);
    let mut arg_cnt: i32 = 1;

    for arg in input {
        match *arg.pat {
            syn::Pat::Ident(ref i) => {
                let t = &i.ident;
                old_func_arg_names.push(quote! { #t });
                let argname = syn::Ident::new(&format!("{}_vec", t), t.span());
                new_func_arg_names.push(quote! { #argname });
            }
            syn::Pat::Tuple(_)
            | syn::Pat::Struct(_)
            | syn::Pat::TupleStruct(_)
            | syn::Pat::Slice(_)
            | syn::Pat::Wild(_) => {
                let pat = &arg.pat;
                old_func_arg_names.push(quote! { #pat });
                let arg_ident = syn::Ident::new(&format!("arg_{}", arg_cnt), arg.span());
                new_func_arg_names.push(quote! { #arg_ident });
                arg_cnt += 1;
            }
            _ => unimplemented!(),
        }
        let ty = &arg.ty;
        new_func_arg_types.push(quote! { Vec<#ty> });
    }

    return (old_func_arg_names, new_func_arg_names, new_func_arg_types);
}

fn get_return_type(ast: &syn::ItemFn) -> &syn::ReturnType {
    &ast.sig.output
}

fn get_fn_body(ast: &syn::ItemFn) -> &Box<syn::Block> {
    &ast.block
}

fn get_new_return_type(op: &syn::ReturnType) -> TokenStream2 {
    match &op {
        syn::ReturnType::Type(_r, t) => quote! { Vec<#t> },
        _ => panic!("Function need to return a type"),
    }
}

fn get_args(ast: &syn::ItemFn) -> Vec<&syn::PatType> {
    let args: Vec<_> = ast
        .sig
        .inputs
        .iter()
        .map(|f| match f {
            syn::FnArg::Typed(p) => p,
            _ => panic!("self not allowed"),
        })
        .collect();
    if args.len() == 0 {
        panic!("expected at least one argument")
    }
    args
}

fn get_zip_map_stmts(
    new_func_arg_names: &Vec<TokenStream2>,
    old_func_arg_names: &Vec<TokenStream2>,
    ast: &syn::ItemFn,
) -> TokenStream2 {
    let first_arg_name = &new_func_arg_names[0];
    let return_type = get_return_type(ast);
    let n = new_func_arg_names.len();
    let rest_arg_names = &new_func_arg_names[1..n];
    let block = get_fn_body(ast);
    let zip_map_stmt = if new_func_arg_names.len() == 1 {
        let old_arg = &old_func_arg_names[0];
        quote! {
            #first_arg_name.into_iter().map(|#old_arg| #return_type #block)
        }
    } else {
        let arg1 = &old_func_arg_names[0];
        let arg2 = &old_func_arg_names[1];
        let mut tuplestring = quote! {
            (#arg1, #arg2)
        };
        for arg in &old_func_arg_names[2..n] {
            tuplestring = quote! { (#tuplestring, #arg) }
        }
        quote! {
            #first_arg_name.into_iter()
                #(.zip(#rest_arg_names.into_iter()))*
                .map(|#tuplestring| #return_type #block)
        }
    };
    return zip_map_stmt;
}

use proc_macro::TokenStream;
use std::ops::Deref;
use quote::quote;
use syn::{FnArg, ItemFn, parse, ReturnType, Signature, Type};
use syn::__private::TokenStream2;

#[proc_macro_attribute]
pub fn proc_macro_attribute2(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    let item_func = parse::<ItemFn>(item).expect("'proc_macro_attribute2' is only allowed on functions");
    implement(item_func).into()
}

fn implement(item_func: ItemFn) -> TokenStream2 {
    if !signature_as_expected(&item_func.sig) {
        panic!("'testable_proc_macro_attribute' is only applicable on functions of type (AttributeArgs, TokenStream2) -> TokenStream2")
    }

    let ident = &item_func.sig.ident;
    let block = &item_func.block;
    let params = &item_func.sig.inputs;
    let output = &item_func.sig.output;

    quote! {
        #[proc_macro_attribute]
        pub fn #ident (attributes: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            implementation::#ident(
                syn::parse_macro_input!(attributes as AttributeArgs),
                item.into()
            ).into()
        }

        pub mod implementation {
            use super::*;

            pub fn #ident (#params) #output {
                #block
            }
        }
    }
}

fn signature_as_expected(sig: &Signature) -> bool {
    if sig.inputs.len() != 2 {
        return false;
    }

    let first_param_ok = argument_of_expected_type(&sig.inputs[0], "AttributeArgs");
    let second_param_ok = argument_of_expected_type(&sig.inputs[1], "TokenStream2");
    let output_ok = output_of_expected_type(&sig.output);
    first_param_ok && second_param_ok && output_ok
}

fn argument_of_expected_type(input: &FnArg, expected_type_name: &str) -> bool {
    match input {
        FnArg::Typed(typed) => match typed.ty.deref() {
            Type::Path(p) => p.path.segments
                .last()
                .map(|seg| &seg.ident)
                .map(|ident| ident.to_string() == expected_type_name)
                .unwrap_or(false),
            _ => false
        }
        _ => false,
    }
}

fn output_of_expected_type(output: &ReturnType) -> bool {
    match output {
        ReturnType::Type(_, ty) => match ty.deref() {
            Type::Path(p) => p.path.segments
                .last()
                .map(|seg| &seg.ident)
                .map(|ident| ident.to_string() == "TokenStream2")
                .unwrap_or(false),
            _ => false,
        }
        _ => false,
    }
}
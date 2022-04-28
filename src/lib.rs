use proc_macro::TokenStream;
use std::ops::Deref;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, parse, ReturnType, Signature, Type};

#[proc_macro_attribute]
pub fn testable_proc_macro_attribute(attributes: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(item_func) = parse::<ItemFn>(item) {
        if !signature_as_expected(&item_func.sig) {
            panic!("'testable_proc_macro_attribute' is only applicable on functions of type (AttributeArgs, TokenStream2) -> TokenStream2")
        }

        let ident = &item_func.sig.ident;
        let new_ident = Ident::new(&format!("{}_testable", ident.to_string()), ident.span());
        let block = &item_func.block;
        let params = &item_func.sig.inputs;
        let output = &item_func.sig.output;

        let result = quote! {
            #[proc_macro_attribute]
            pub fn #ident (attributes: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
                #new_ident(
                    syn::parse_macro_input!(attributes as AttributeArgs),
                    item.into()
                ).into()
            }

            fn #new_ident (#params) #output {
                #block
            }
        };

        return result.into()
    }

    panic!("'testable_proc_macro_attribute' is only allowed on functions")
}

fn signature_as_expected(sig: &Signature) -> bool {
    if !sig.inputs.len() == 2 {
        // only two arguments allowed
        return false;
    }

    let first_param_ok = match &sig.inputs[0] {
        FnArg::Receiver(_) => false,
        FnArg::Typed(typed) => if let Type::Path(p) = typed.ty.deref() {
            p.path.get_ident().unwrap().to_string().ends_with("AttributeArgs")
        } else {
            false
        }
    };

    let second_param_ok = match &sig.inputs[1] {
        FnArg::Receiver(_) => false,
        FnArg::Typed(typed) => if let Type::Path(p) = typed.ty.deref() {
            p.path.get_ident().unwrap().to_string().ends_with("TokenStream2")
        } else {
            false
        }
    };

    let output_ok = match &sig.output {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => if let Type::Path(p) = ty.deref() {
            p.path.get_ident().unwrap().to_string().ends_with("TokenStream2")
        } else {
            false
        }
    };

    first_param_ok && second_param_ok && output_ok
}

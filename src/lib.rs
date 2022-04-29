use syn::__private::TokenStream2;
use syn::{Attribute, AttributeArgs, Ident, Item, Path};
use syn::Item::*;

mod inner_mod {
    pub mod implementation {
        use syn::__private::TokenStream2;
        use syn::AttributeArgs;

        pub fn bar(_attr: AttributeArgs, item: TokenStream2) -> TokenStream2 {
            item
        }
    }
}

#[macro_export]
macro_rules! assert_attribute_implementation_as_expected {
    ($base_path:path : $attr:ident, item: {$item:item}  expected: {$($expected:tt)*}) => {
        {
            use $base_path :: {implementation :: $attr};

            let ident = syn::parse2::<syn::Ident>(quote::quote! {$attr}).unwrap();
            let mut item = syn::parse2::<syn::Item>(quote::quote! { $item }).unwrap();
            let expected_ts = quote::quote! { $($expected)* }.to_string();
            let attribute = crate::extract_attribute_from_item(&ident, &mut item);
            let attr_args = crate::extract_attribute_args(attribute);
            let implementation_ts = $attr(attr_args, quote::quote! { $item }).to_string();
            crate::assert_tokens_are_equal(implementation_ts, expected_ts)
        }
    }
}

fn extract_attribute_from_item(ident: &Ident, item: &mut Item) -> Attribute {
    let mut attributes = get_attributes_from_item(item);
    let attribute_index = attributes.iter()
        .enumerate()
        .find(|(_, a)| attribute_has_ident(a, ident))
        .expect("Could not find expected attribute").0;
    attributes.remove(attribute_index)
}

fn get_attributes_from_item(item: &mut Item) -> &mut Vec<Attribute> {
    match item {
        Const(i) => &mut i.attrs,
        Enum(i) => &mut i.attrs,
        ExternCrate(i) => &mut i.attrs,
        Fn(i) => &mut i.attrs,
        ForeignMod(i) => &mut i.attrs,
        Impl(i) => &mut i.attrs,
        Macro(i) => &mut i.attrs,
        Macro2(i) => &mut i.attrs,
        Mod(i) => &mut i.attrs,
        Static(i) => &mut i.attrs,
        Struct(i) => &mut i.attrs,
        Trait(i) => &mut i.attrs,
        TraitAlias(i) => &mut i.attrs,
        Type(i) => &mut i.attrs,
        Union(i) => &mut i.attrs,
        Use(i) => &mut i.attrs,
        _ => panic!("Could not extract attributes")
    }
}

pub fn attribute_has_ident(attribute: &Attribute, ident: &Ident) -> bool {
    path_to_name(&attribute.path) == ident.to_string()
}

fn path_to_name(path: &Path) -> String {
    path.segments.last().map(|seg| seg.ident.to_string()).expect("The given path was not an identifier.")
}

fn extract_attribute_args(attr: Attribute) -> AttributeArgs {
    match attr.parse_meta().unwrap() {
        syn::Meta::List(list) => list.nested.into_iter().collect(),
        _ => vec![]
    }
}

pub fn assert_tokens_are_equal<L, R>(left: L, right: R) where L: AsRef<str>, R: AsRef<str> {
    assert_eq!(remove_whitespace(left.as_ref()), remove_whitespace(right.as_ref()))
}

fn remove_whitespace(string: &str) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        assert_attribute_implementation_as_expected!(
            crate::inner_mod : bar,
            item: {
                #[bar]
                struct S {
                    foo: usize,
                }
            }

            expected: {
                #[bar]
                struct S {
                    foo: usize,
                }
            }
        )
    }
}
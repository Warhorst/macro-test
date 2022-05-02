use quote::__private::Ident;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Attribute, AttributeArgs, Item, Path};
pub use attributes::proc_macro_attribute2;

/// This macro checks if an item with an attribute to test generates the
/// expected token stream. Requires the import of macro-test::compare_implementations.
/// This only works if your attributes uses the 'proc_macro_attribute2' attribute.
/// Explanation below.
///
/// # How it works
/// Imagine you have created an attribute which implements a static method for a struct.
/// Maybe it creates a function that returns 42. To check if it works, use the macro like this:
///
/// ``` text
/// assert_attribute_implementation_as_expected!(
///             crate::my_attribute : create_the_answer,
///             item: {
///                 #[create_the_answer]
///                 struct S {
///                     foo: usize,
///                 }
///             }
///
///             expected: {
///                 struct S {
///                     foo: usize,
///                 }
///
///                 impl S {
///                     fn get_the_answer() -> usize {
///                         42
///                     }
///                 }
///             }
///         )
/// ```
///
/// Currently, a check is created whether the attribute ('create_the_answer' in this case)
/// creates the same token stream as the input of expected. Both token streams are turned
/// into strings and are checked for equality.
///
/// 'crate::my_attribute : create_the_answer' tells where your attribute is and what its named.
/// The single colon is crucial because the path to the testable code will be in
/// 'crate::my_attribute::implementation::create_the_answer'. This implementation module
/// is created by 'proc_macro_attribute2'.
#[macro_export]
macro_rules! assert_attribute_implementation_as_expected {
    ($base_path:path : $attr:ident, item: {$item:item}  expected: {$($expected:tt)*}) => {
        {
            use $base_path :: {implementation :: $attr};

            let ident = syn::parse2::<syn::Ident>(quote::quote! {$attr}).unwrap();
            let item = syn::parse2::<syn::Item>(quote::quote! { $item }).unwrap();
            let expected_ts = quote::quote! { $($expected)* };
            compare_implementations(|args, ts| $attr(args, ts), ident, item, expected_ts)
        }
    }
}

pub fn compare_implementations(
    implementor: fn(AttributeArgs, TokenStream2) -> TokenStream2,
    attribute_ident: Ident,
    mut item: Item,
    expectation: TokenStream2,
) {
    let attribute = extract_attribute_from_item(&attribute_ident, &mut item);
    let attribute_args = transform_attribute_to_attribute_args(attribute);
    let implementation = (implementor)(attribute_args, quote! {#item});
    let remove_whitespace = |s: String| s.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    assert_eq!(remove_whitespace(implementation.to_string()), remove_whitespace(expectation.to_string()))

}

fn extract_attribute_from_item(attribute_ident: &Ident, item: &mut Item) -> Attribute {
    let attributes = get_attributes_from_item(item);
    let attribute_index = attributes.iter()
        .enumerate()
        .find(|(_, a)| attribute_has_ident(a, attribute_ident))
        .expect("Could not find expected attribute").0;
    attributes.remove(attribute_index)
}

fn get_attributes_from_item(item: &mut Item) -> &mut Vec<Attribute> {
    match item {
        Item::Const(i) => &mut i.attrs,
        Item::Enum(i) => &mut i.attrs,
        Item::ExternCrate(i) => &mut i.attrs,
        Item::Fn(i) => &mut i.attrs,
        Item::ForeignMod(i) => &mut i.attrs,
        Item::Impl(i) => &mut i.attrs,
        Item::Macro(i) => &mut i.attrs,
        Item::Macro2(i) => &mut i.attrs,
        Item::Mod(i) => &mut i.attrs,
        Item::Static(i) => &mut i.attrs,
        Item::Struct(i) => &mut i.attrs,
        Item::Trait(i) => &mut i.attrs,
        Item::TraitAlias(i) => &mut i.attrs,
        Item::Type(i) => &mut i.attrs,
        Item::Union(i) => &mut i.attrs,
        Item::Use(i) => &mut i.attrs,
        _ => panic!("Could not extract attributes")
    }
}

fn attribute_has_ident(a: &Attribute, i: &Ident) -> bool {
    *i == path_to_name(&a.path)
}

fn path_to_name(p: &Path) -> String {
    p.segments
        .last()
        .map(|seg| seg.ident.to_string())
        .expect("The given path was not an identifier.")
}

fn transform_attribute_to_attribute_args(attribute: Attribute) -> AttributeArgs {
    match attribute.parse_meta().unwrap() {
        syn::Meta::List(list) => list.nested.into_iter().collect(),
        _ => vec![]
    }
}

#[cfg(test)]
mod tests {
    pub mod implementation {
        use syn::__private::TokenStream2;
        use syn::AttributeArgs;

        pub fn bar(_attr: AttributeArgs, item: TokenStream2) -> TokenStream2 {
            item
        }
    }

    #[test]
    fn foo() {
        use crate::compare_implementations;

        assert_attribute_implementation_as_expected!(
            crate::tests : bar,
            item: {
                #[bar]
                struct S {
                    foo: usize,
                }
            }

            expected: {
                struct S {
                    foo: usize,
                }
            }
        )
    }
}
use syn::{Attribute, AttributeArgs, Ident, Item, Path};
use syn::__private::TokenStream2;
use syn::Item::*;

/// This macro checks if an item with an attribute to test generates the
/// expected token stream.
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
            let mut item = syn::parse2::<syn::Item>(quote::quote! { $item }).unwrap();
            let expected_ts = quote::quote! { $($expected)* };
            let attribute = crate::extract_attribute_from_item(&ident, &mut item);
            let attr_args = crate::extract_attribute_args(attribute);
            let implementation_ts = $attr(attr_args, quote::quote! { #item });
            crate::assert_tokens_are_equal(implementation_ts, expected_ts)
        }
    }
}

fn extract_attribute_from_item(ident: &Ident, item: &mut Item) -> Attribute {
    let attributes = get_attributes_from_item(item);
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

pub fn assert_tokens_are_equal(left: TokenStream2, right: TokenStream2) {
    assert_eq!(remove_whitespace(left.to_string()), remove_whitespace(right.to_string()))
}

fn remove_whitespace(string: String) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
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
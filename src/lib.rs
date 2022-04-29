pub use attributes::proc_macro_attribute2;

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
            // Create the correct import for the attribute to test
            use $base_path :: {implementation :: $attr};

            // Parse item and ident and transform the 'expected' token tree to a token stream
            let ident = syn::parse2::<syn::Ident>(quote::quote! {$attr}).unwrap();
            let mut item = syn::parse2::<syn::Item>(quote::quote! { $item }).unwrap();
            let expected_ts = quote::quote! { $($expected)* };

            // Extract the attribute to test from the item
            let attribute = {
                let attribute_has_ident = |a: &syn::Attribute, i: &syn::Ident| {
                    let path_to_name = |p: &syn::Path| p.segments
                        .last()
                        .map(|seg| seg.ident.to_string())
                        .expect("The given path was not an identifier.");
                    path_to_name(&a.path) == i.to_string()
                };
                let attributes = match &mut item {
                    syn::Item::Const(i) => &mut i.attrs,
                    syn::Item::Enum(i) => &mut i.attrs,
                    syn::Item::ExternCrate(i) => &mut i.attrs,
                    syn::Item::Fn(i) => &mut i.attrs,
                    syn::Item::ForeignMod(i) => &mut i.attrs,
                    syn::Item::Impl(i) => &mut i.attrs,
                    syn::Item::Macro(i) => &mut i.attrs,
                    syn::Item::Macro2(i) => &mut i.attrs,
                    syn::Item::Mod(i) => &mut i.attrs,
                    syn::Item::Static(i) => &mut i.attrs,
                    syn::Item::Struct(i) => &mut i.attrs,
                    syn::Item::Trait(i) => &mut i.attrs,
                    syn::Item::TraitAlias(i) => &mut i.attrs,
                    syn::Item::Type(i) => &mut i.attrs,
                    syn::Item::Union(i) => &mut i.attrs,
                    syn::Item::Use(i) => &mut i.attrs,
                    _ => panic!("Could not extract attributes")
                };

                let attribute_index = attributes.iter()
                    .enumerate()
                    .find(|(_, a)| attribute_has_ident(a, &ident))
                    .expect("Could not find expected attribute").0;
                attributes.remove(attribute_index)
            };

            // Transform the attribute to AttributeArgs
            let attr_args = {
                match attribute.parse_meta().unwrap() {
                    syn::Meta::List(list) => list.nested.into_iter().collect(),
                    _ => vec![]
                }
            };

            // Execute the attribute function
            let implementation_ts = $attr(attr_args, quote::quote! { #item });

            // Assert that both token streams are equal
            let remove_whitespace = |s: String| s.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            assert_eq!(remove_whitespace(implementation_ts.to_string()), remove_whitespace(expected_ts.to_string()))
        }
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
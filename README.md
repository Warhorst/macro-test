# macro-test

A crate to make proc_macro_attributes easily testable.

Provides an attribute to generate testable code for proc_macro_attributes and a macro_rule to test an annotated item against an expectation.

## Example
``` rust
use syn::AttributeArgs;
use syn::__private::TokenStream2;

#[proc_macro_attribute2]
pub fn generate_answer(attributes: AttributeArgs, item: TokenStream2) -> TokenStream2 {
    // generate a function for a struct item which returns "the answer"
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_attribute_implementation_as_expected!(
            crate : generate_answer,
            item: {
                #[generate_answer]
                pub struct Foo;
            },
            expected: {
                pub struct Foo;
                
                impl Foo {
                    pub fn get_answer() -> usize { 42 }
                }
            }
        )
    }
}
```

The test checks if the attribute function creates the same token stream as provided by 'expected'.

## How it works
The attribute 'proc_macro_attribute2'
``` rust
#[proc_macro_attribute2]
pub fn generate_answer(attributes: AttributeArgs, item: TokenStream2) -> TokenStream2 {
    // generate a function for a struct item which returns "the answer"
}
```

creates an implementation equivalent to

``` rust 
pub fn generate_answer(attributes: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implementation::generate_answer(
        syn::parse_macro_input!(attributes as AttributeArgs),
        item.into()
    ).into()
}

pub (in crate) mod implementation {
    use super::*;
    
    pub fn generate_answer(attributes: AttributeArgs, item: TokenStream2) -> TokenStream2 {
        // generate a function for a struct item which returns "the answer"
    }
}
```

The code in 'mod implementation' is testable, as it doesn't use proc_macro. assert_attribute_implementation_as_expected! uses this implementation for testing purposes.

The attribute is only applicable on functions with the signature (attributes: AttributeArgs, item: TokenStream2) -> TokenStream2 (therefore the name proc_macro_attribute2).
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitFloat, ItemStruct, DeriveInput};

#[proc_macro_derive(Planet, attributes(orbital_period))]
pub fn derive_planet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Find the orbital_period helper attribute! (it is required!)
    let attr = input.attrs.iter().filter(|attr| attr.path.is_ident("orbital_period")).next().unwrap();

    // Parse the argument to the attribute as a float
    let orbital_period: LitFloat = attr.parse_args().unwrap();
    
    // The generated impl for Planet using the orbital_period from the derive helper attribute
    let expanded = quote! {
        impl crate::Planet for #name {
            const ORBITAL_PERIOD: f64 = #orbital_period;
        }
    };

    // Return the generated trait impl back to the compiler
    TokenStream::from(expanded)
}

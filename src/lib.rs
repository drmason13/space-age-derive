use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitFloat, ItemStruct};

#[proc_macro_attribute]
pub fn orbital_period(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    // Parse the argument to the attribute as a float
    let orbital_period = parse_macro_input!(args as LitFloat);

    // expand to the struct and an imnpl for Planet using the orbital_period from the attribute
    let expanded = quote! {
        #input
        
        impl crate::Planet for #name {
            const ORBITAL_PERIOD: f64 = #orbital_period;
        }
    };

    // Hand the resulting struct and trait impl back to the compiler
    TokenStream::from(expanded)
}

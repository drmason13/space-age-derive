use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitFloat};

#[proc_macro_derive(Planet, attributes(orbital_period))]
pub fn derive_planet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Find the orbital_period helper attribute! (it is required!)
    let attr = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("orbital_period"));

    if attr.is_none() {
        return syn::Error::new_spanned(
            &name,
            "Missing orbital_period attribute, e.g. `#[orbital_period = 1.0]`",
        )
        .to_compile_error()
        .into();
    }

    // we've just checked it isn't None
    let attr = attr.unwrap();

    // Parse the value of the attribute as a float
    let orbital_period: LitFloat = match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(name_value) => {
            if let syn::Lit::Float(float) = name_value.lit {
                float
            } else {
                return syn::Error::new_spanned(
                    &name_value.lit,
                    "expected a float value, e.g. `#[orbital_period = 1.0]`",
                )
                .to_compile_error()
                .into();
            }
        }
        anything_else => {
            return syn::Error::new_spanned(
                &anything_else,
                "expected a `name = value` style syntax, e.g. `#[orbital_period = 1.0]`",
            )
            .to_compile_error()
            .into()
        }
    };

    let expanded = quote! {
        impl space_age_core::Planet for #name {
            const ORBITAL_PERIOD: f64 = #orbital_period;
        }
    };

    // Return the generated trait impl back to the compiler
    TokenStream::from(expanded)
}

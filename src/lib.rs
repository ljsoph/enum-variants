use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumVariants)]
pub fn variants_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        syn::Data::Enum(data_enum) => data_enum.variants.into_iter().map(|v| v.ident),
        _ => panic!("EnumVariants only works on Enums, champ..."),
    };

    let expanded = quote! {
        impl #name {
            pub fn variants() -> &'static[#name] {
                &[#(#name::#variants),*]
            }
        }
    };

    expanded.into()
}

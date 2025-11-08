use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input: syn::ItemEnum = syn::parse_macro_input!(input);
    let name = &input.ident;
    let variants = input.variants.into_iter().map(|v| v.ident);

    let expanded = quote! {
        impl #name {
            pub fn variants() -> &'static[#name] {
                &[#(#name::#variants),*]
            }
        }
    };

    expanded.into()
}

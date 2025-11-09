use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input: syn::ItemEnum = syn::parse_macro_input!(input);
    let name = &input.ident;

    let variants: Vec<proc_macro2::TokenStream> = input
        .variants
        .into_iter()
        .map(|variant| {
            let ident = variant.ident;

            match variant.fields {
                syn::Fields::Unit => {
                    quote! { #ident }
                }
                syn::Fields::Unnamed(unnamed) => {
                    let fields = unnamed.unnamed.into_iter().map(|field| {
                        let ty = field.ty;
                        quote! { <#ty>::default() }
                    });

                    quote! { #ident(#(#fields),*)}
                }
                syn::Fields::Named(named) => {
                    let fields = named.named.into_iter().map(|field| {
                        let ty = &field.ty;
                        let f = &field.ident;
                        quote! { #f: <#ty>::default() }
                    });

                    quote! { #ident { #(#fields),* } }
                }
            }
        })
        .collect();

    let count = variants.len();
    let expanded = quote! {
        impl #name {
            pub fn variants() -> [#name; #count] {
                [#(#name::#variants),*]
            }
        }
    };

    expanded.into()
}

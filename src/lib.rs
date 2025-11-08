use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input: syn::ItemEnum = syn::parse_macro_input!(input);
    let name = &input.ident;

    let variants = input.variants.into_iter().map(|variant| {
        let ident = variant.ident;

        match variant.fields {
            syn::Fields::Unit => {
                quote! { #ident }
            }
            syn::Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.into_iter().map(|field| {
                    // Trying to insert `default()` here directly doesn't work as it is considered
                    // temporary data owned by the current function.
                    //
                    // Manually constructing works e.g., `quote! { #field { a: 22 } }`
                    //
                    // I'm assuming this works as the data is Copy and
                    // this would also fail to compile if I used a String instead of an i32 here.
                    // Though I'm not sure what makes `default()` here a local reference.
                    quote! { #field::default() }
                });

                quote! { #ident(#(#fields),*)}
            }
            syn::Fields::Named(_fields_named) => unimplemented!(),
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn variants() -> &'static[#name] {
                &[#(#name::#variants),*]
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(EnumVariants_Original)]
pub fn enum_variants_original(input: TokenStream) -> TokenStream {
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

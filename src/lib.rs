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

#[proc_macro_derive(EnumDiscriminants)]
pub fn discriminants(input: TokenStream) -> TokenStream {
    let input: syn::ItemEnum = syn::parse_macro_input!(input);
    let ident = &input.ident;
    let name = syn::Ident::new(&format!("{ident}Discriminant"), ident.span());

    let variants = input.variants.into_iter().map(|variant| {
        let ident = variant.ident;
        let data = match variant.fields {
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_named) => quote! { { .. } },
            syn::Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.into_iter().map(|_field| quote! { _ });
                quote! { (#(#fields),*)}
            }
        };

        (quote! { #ident }, data)
    });

    let (variants, data): (Vec<_>, Vec<_>) = variants.unzip();
    let def = discriminant_definition(&name, &variants);
    let from_impl = from_impl(&ident, &name, &variants, &data);

    let expanded = quote! {
        #def
        #from_impl
    };

    expanded.into()
}

fn discriminant_definition(
    name: &syn::Ident,
    variants: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn from_impl(
    original_name: &syn::Ident,
    impl_name: &syn::Ident,
    variants: &Vec<proc_macro2::TokenStream>,
    data: &Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        impl From<#original_name> for #impl_name {
            fn from(value: #original_name) -> #impl_name {
                match value {
                    #(#original_name::#variants #data => #impl_name::#variants),*
                }
            }
        }
    }
}

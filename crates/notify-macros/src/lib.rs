use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

fn find_crate() -> Ident {
    use proc_macro_crate::{crate_name, FoundCrate};

    let found_crate = crate_name("notify").expect("notify couldn't be found in Cargo.toml");

    match found_crate {
        FoundCrate::Itself => Ident::new("crate", Span::call_site()),
        FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    }
}

#[proc_macro_derive(EmailNotification)]
pub fn derive_email_notification(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_name = find_crate();

    let template_manager = quote!(#crate_name::template::TemplateManager);
    let template = quote!(#crate_name::template::Template);
    let email_template = quote!(#crate_name::template::EmailTemplate);
    let error = quote!(#crate_name::template::Error);

    let expanded = quote! {
        impl #impl_generics #template for #name #ty_generics #where_clause {
            fn register(manager: &mut #template_manager) -> Result<(), #error> {
                <Self as #email_template>::register(manager)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

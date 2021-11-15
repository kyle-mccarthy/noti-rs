use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EmailNotification)]
pub fn derive_email_notification(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Template for #name #ty_generics #where_clause {
            fn register(manager: &mut TemplateManager) -> Result<(), super::Error> {
                <Self as EmailTemplate>::register(manager)    
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

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

    let rendered_template_type = quote!(#crate_name::template::RenderedTemplate);

    let notification_trait = quote!(#crate_name::notification::Notification);
    let email_notification_trait = quote!(#crate_name::notification::EmailNotification);

    let message = quote!(#crate_name::message::Message);

    let channel_type = quote!(#crate_name::channel::ChannelType);
    let email_channel_type = quote!(#crate_name::channel::ChannelType::Email);

    let expanded = quote! {
        impl #impl_generics #template for #name #ty_generics #where_clause {
            fn register(manager: &mut #template_manager) -> Result<(), #error> {
                <Self as #email_template>::register(manager)
            }
        }

        impl #impl_generics #notification_trait for #name #ty_generics #where_clause {
            const CHANNEL_TYPE: &'static #channel_type = &#email_channel_type;

            fn into_message(self, rendered_template: #rendered_template_type) -> #message {
                let rendered = rendered_template.into_email().expect("Rendered template was not the expected type");
                <Self as #email_notification_trait>::build(&self, rendered).into_message()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EmailProvider)]
pub fn derive_channel_for_email_provider(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_name = find_crate();

    let channel_type = quote!(#crate_name::channel::ChannelType);
    let email_channel_type = quote!(#crate_name::channel::ChannelType::Email);
    let channel = quote!(#crate_name::channel::Channel);
    let provider = quote!(#crate_name::channel::email::EmailProvider);
    let message = quote!(#crate_name::channel::Message);

    let error = quote!(#crate_name::channel::Error);

    let expanded = quote! {
        #[async_trait::async_trait]
        impl #impl_generics #channel for #name #ty_generics #where_clause {
            fn channel_type(&self) -> &'static #channel_type {
                &#email_channel_type
            }

            async fn send(&self, message: #message) -> Result<(), #error> {
                let message_channel = message.channel();

                if message_channel != self.channel_type() {
                    return Err(#error::InvalidMessageChannel { expected: self.channel_type(), found: message_channel });
                }

                // safety: we verified the type of the channel above, so we should be able to
                // safely unwrap it.
                let email = message.into_email().unwrap();

                <Self as #provider>::send(self, email).await?;

                Ok(())
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

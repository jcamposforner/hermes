use proc_macro::TokenStream;

#[proc_macro_derive(EventMetadata)]
pub fn metadata_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let expanded = quote::quote! {
        impl hermes::event::EventWithMetadata for #name {
            fn add_metadata(&mut self, key: String, value: String) {
                self.metadata.add(key, value);
            }

            fn get_metadata(&self, key: &str) -> Option<&String> {
                self.metadata.get(key)
            }

            fn metadata(&self) -> &hermes::event::EventMetadata {
                &self.metadata
            }

            fn drain_metadata(&mut self) -> hermes::event::EventMetadata {
                std::mem::take(&mut self.metadata)
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let event_name = convert_case::Casing::to_case(&name.to_string(), convert_case::Case::Snake);
    let event_name_literal = syn::LitStr::new(&event_name, name.span());

    let expanded = quote::quote! {
        impl hermes::event::Event for #name {
            fn event_name(&self) -> &'static str {
                <Self as hermes::event::EventName>::static_event_name()
            }
        }

        impl hermes::event::EventName for #name {
            fn static_event_name() -> &'static str {
                #event_name_literal
            }
        }
    };
    TokenStream::from(expanded)
}

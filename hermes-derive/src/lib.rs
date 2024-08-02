use proc_macro::TokenStream;

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let event_name = convert_case::Casing::to_case(&name.to_string(), convert_case::Case::Snake);

    let expanded = quote::quote! {
        impl hermes::event::Event for #name {
            fn event_name(&self) -> &'static str {
                <Self as hermes::event::EventName>::static_event_name()
            }
        }

        impl hermes::event::EventName for #name {
            fn static_event_name() -> &'static str {
                #event_name
            }
        }
    };
    TokenStream::from(expanded)
}
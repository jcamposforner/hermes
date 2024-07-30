use proc_macro::TokenStream;

#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let expanded = quote::quote! {
        impl hermes::event::Event for #name {}
    };
    TokenStream::from(expanded)
}
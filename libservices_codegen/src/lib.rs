mod attachable_service;

#[proc_macro_derive(AttachableService, attributes(service))]
pub fn attachable_service(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attachable_service::derive_attachable_service(input)
}

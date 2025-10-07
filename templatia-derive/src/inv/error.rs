pub(crate) fn generate_compile_error(msg: &str) -> proc_macro2::TokenStream {
    let error = syn::Error::new(proc_macro2::Span::call_site(), msg);
    error.to_compile_error().into()
}
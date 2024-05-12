use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = proc_macro2::TokenStream::from(attribute);
    let item = proc_macro2::TokenStream::from(item);

    TokenStream::from(quote_spanned!(attribute.span()=>
        ::core::compile_error!("Quanto is not implemented yet");

        #item
    ))
}

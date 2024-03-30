use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn main(attribute: TokenStream, _item: TokenStream) -> TokenStream {
    quote_spanned!(attribute.span()=>
        ::core::compile_error!("Quanto is not implemented yet");
    )
}

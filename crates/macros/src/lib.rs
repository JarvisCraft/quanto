use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, LitStr, Token,
};

#[proc_macro_attribute]
pub fn main(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = proc_macro2::TokenStream::from(attribute);
    let item = proc_macro2::TokenStream::from(item);

    TokenStream::from(quote_spanned!(attribute.span()=>
        ::core::compile_error!("Quanto is not implemented yet");

        #item
    ))
}

#[proc_macro]
pub fn execute(tokens: TokenStream) -> TokenStream {
    let ExecuteMacroInput {
        src,
        _coma_1,
        args,
        span,
    } = parse_macro_input!(tokens as ExecuteMacroInput);
    let rt = quote! {::quanto::runtime};
    let converted_args: Punctuated<_, Token![,]> = args
        .iter()
        .map(|arg| {
            quote_spanned! {arg.span()=>
                ::core::convert::Into::<#rt::value::Value>::into(#arg)
            }
        })
        .collect();
    let args_count = converted_args.len();

    TokenStream::from(quote_spanned! {span=>
        {
            const PROGRAM: #rt::Program<#args_count> = #rt::Program::parse(#src);
            #rt::rt::SyncQuantoRuntime::execute(
                #rt::global::sync_rt(),
                PROGRAM.bind(&[#converted_args]),
            )
        }
    })
}

struct ExecuteMacroInput {
    src: LitStr,
    _coma_1: Token![,],
    args: Punctuated<Expr, Token![,]>,

    span: proc_macro2::Span,
}

impl Parse for ExecuteMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            src: input.parse()?,
            _coma_1: input.parse()?,
            args: input.parse_terminated(Expr::parse, Token![,])?,
            span: input.span(),
        })
    }
}

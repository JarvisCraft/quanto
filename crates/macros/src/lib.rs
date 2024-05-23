use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, LitStr, Path, Token,
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
        namespace,
        _semicolon_1,
        src,
        _coma_1,
        args,
        span,
    } = parse_macro_input!(tokens as ExecuteMacroInput);
    let rt = quote! { #namespace::quanto_runtime };
    let converted_args: Punctuated<_, Token![,]> = args
        .iter()
        .map(
            |ExecuteMacroInputVar {
                 quantum_sigil,
                 key,
                 _eq,
                 value,
             }| {
                let value_type = if quantum_sigil.is_some() {
                    quote_spanned!(quantum_sigil.span()=> #rt::value::quantum::Value)
                } else {
                    quote_spanned!(quantum_sigil.span()=> #rt::value::scalar::Value)
                };

                let key = quote_spanned! {key.span()=>
                    ::core::stringify!(#key)
                };
                // TODO: "wide: spans
                let value = quote_spanned! {value.span()=>
                    ::core::convert::Into::<#rt::value::Value>::into(
                        ::core::convert::Into::<#value_type>::into(#value)
                    ),
                };

                quote! { (#key, #value) }
            },
        )
        .collect();
    let args_count = converted_args.len();

    TokenStream::from(if let Err(error) = quanto_parser::validate(&src.value()) {
        let error = format!("Syntax error in Quanto expression:\n{error}");
        quote_spanned! {span=>
            {
                ::core::compile_error!(#error)
            }
        }
    } else {
        quote_spanned! {span=>
            {
                const PROGRAM: #rt::Program<#args_count> = #rt::Program::parse(#src);
                #rt::rt::SyncQuantoRuntime::execute(
                    #rt::global::sync_rt(),
                    PROGRAM.bind(&[#converted_args]),
                )
            }
        }
    })
}

struct ExecuteMacroInput {
    namespace: Path,
    _semicolon_1: Token![,],
    src: LitStr,
    _coma_1: Token![,],
    args: Punctuated<ExecuteMacroInputVar, Token![,]>,

    span: proc_macro2::Span,
}

impl Parse for ExecuteMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            namespace: input.parse()?,
            _semicolon_1: input.parse()?,
            src: input.parse()?,
            _coma_1: input.parse()?,
            args: input.parse_terminated(Parse::parse, Token![,])?,
            span: input.span(),
        })
    }
}

struct ExecuteMacroInputVar {
    quantum_sigil: Option<Token![@]>,
    key: Ident,
    _eq: Token![=],
    value: Expr,
}

impl Parse for ExecuteMacroInputVar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            quantum_sigil: input.parse()?,
            key: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

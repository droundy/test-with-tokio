use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    punctuated::Punctuated,
    token::{Comma, Semi},
    ExprLet, FnArg, Pat,
};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

fn fnarg_to_ident(f: &FnArg) -> Ident {
    let FnArg::Typed(f) = f else {
        panic!("self no good");
    };
    let Pat::Ident(f) = &*f.pat else {
        panic!("should have an identifier");
    };
    f.ident.clone()
}

#[proc_macro_attribute]
pub fn with(args: TokenStream, item: TokenStream) -> TokenStream {
    let lets = syn::parse_macro_input!(args with Punctuated::<ExprLet, Semi>::parse_terminated);

    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let mut input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    input.sig.asyncness = None;
    let function_name = input.sig.ident.clone();
    input.sig.ident = syn::Ident::new(&format!("{function_name}_internal"), function_name.span());
    let function_name_internal = input.sig.ident.clone();
    let function_args: Punctuated<Ident, Comma> = input
        .sig
        .inputs
        .pairs()
        .map(|p| fnarg_to_ident(p.value().clone()))
        .collect();

    let result = quote! {
        // #[::core::prelude::v1::test]
        #input

        #[::core::prelude::v1::test]
        fn #function_name() {
            #lets;
            ::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    #function_name_internal(#function_args)
                })
        }
    };

    result.into()
}

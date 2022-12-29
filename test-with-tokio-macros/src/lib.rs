use proc_macro::TokenStream;
use quote::quote;
use syn::Stmt;

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn please(_args: TokenStream, item: TokenStream) -> TokenStream {
    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let mut input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    let last_block = input.block.stmts.pop().unwrap();
    let last_statement: Stmt = syn::parse2(quote! {
        ::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(#last_block);
    })
    .unwrap();
    input.block.stmts.push(last_statement);

    let result = quote! {
        #[::core::prelude::v1::test]
        #input
    };

    result.into()
}

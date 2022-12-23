use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned, ToTokens};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

#[proc_macro_attribute]
pub fn test_with(args: TokenStream, item: TokenStream) -> TokenStream {
    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let mut input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    input.sig.asyncness = None;

    // If type mismatch occurs, the current rustc points to the last statement.
    let (last_stmt_start_span, last_stmt_end_span) = {
        let mut last_stmt = input
            .block
            .stmts
            .last()
            .map(ToTokens::into_token_stream)
            .unwrap_or_default()
            .into_iter();
        // `Span` on stable Rust has a limitation that only points to the first
        // token, not the whole tokens. We can work around this limitation by
        // using the first/last span of the tokens like
        // `syn::Error::new_spanned` does.
        let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
        let end = last_stmt.last().map_or(start, |t| t.span());
        (start, end)
    };

    let guard: syn::Expr = match syn::parse(args.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    let crate_ident = Ident::new("tokio", last_stmt_start_span);

    let body = &input.block;
    let brace_token = input.block.brace_token;
    let body_ident = quote! { body };
    let block_expr = quote_spanned! {last_stmt_end_span=>
        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
            return ::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(#body_ident);
        }
    };

    // For test functions pin the body to the stack and use `Pin<&mut dyn
    // Future>` to reduce the amount of `Runtime::block_on` (and related
    // functions) copies we generate during compilation due to the generic
    // parameter `F` (the future to block on). This could have an impact on
    // performance, but because it's only for testing it's unlikely to be very
    // large.
    //
    // We don't do this for the main function as it should only be used once so
    // there will be no benefit.
    let body = {
        let output_type = match &input.sig.output {
            // For functions with no return value syn doesn't print anything,
            // but that doesn't work as `Output` for our boxed `Future`, so
            // default to `()` (the same type as the function output).
            syn::ReturnType::Default => quote! { () },
            syn::ReturnType::Type(_, ret_type) => quote! { #ret_type },
        };
        quote! {
            let body = async #body;
            #crate_ident::pin!(body);
            let body: ::std::pin::Pin<&mut dyn ::std::future::Future<Output = #output_type>> = body;
        }
    };

    input.block = syn::parse2(quote! {
        {
            let mut _guard = #guard;
            #body
            #block_expr
        }
    })
    .expect("Parsing failure");
    input.block.brace_token = brace_token;

    let result = quote! {
        #[::core::prelude::v1::test]
        #input
    };

    result.into()
}

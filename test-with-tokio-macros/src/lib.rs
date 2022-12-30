use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
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
    let mut cases: Vec<(syn::Expr, syn::Expr, String)> = Vec::new();
    for stmt in input.block.stmts.iter() {
        if let Stmt::Local(local) = stmt {
            if let Some((_, e)) = &local.init {
                if let syn::Expr::Match(m) = e.as_ref() {
                    if let syn::Expr::Path(p) = m.expr.as_ref() {
                        if let Some(i) = p.path.get_ident() {
                            if format!("{i}") == "CASE" {
                                for arm in m.arms.iter() {
                                    if let syn::Pat::Lit(p) = &arm.pat {
                                        if let syn::Expr::Lit(e) = p.expr.as_ref() {
                                            if let syn::Lit::Str(s) = &e.lit {
                                                if s.value()
                                                    .chars()
                                                    .any(|c| !c.is_alphanumeric() && c != '_')
                                                {
                                                    return quote_spanned! {
                                                        s.span() =>
                                                        compile_error!("not a valid identifier");
                                                    }
                                                    .into();
                                                }
                                                cases.push((
                                                    (*p.expr).clone(),
                                                    (*arm.body).clone(),
                                                    s.value(),
                                                ));
                                            } else {
                                                return quote_spanned! {
                                                    e.span() =>
                                                    compile_error!("expected string literal");
                                                }
                                                .into();
                                            }
                                        } else {
                                            return quote_spanned! {
                                                p.expr.span() =>
                                                compile_error!("expected string literal");
                                            }
                                            .into();
                                        }
                                    } else {
                                        return quote_spanned! {
                                            arm.pat.span() =>
                                            compile_error!("expected string literal");
                                        }
                                        .into();
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    let Some(last_block) = input.block.stmts.pop() else {
        return quote_spanned! {
            input.block.span() =>
            compile_error!("expected function to end with an async block");
        }
        .into();
    };
    let last_statement: Stmt = syn::parse2(quote! {
        ::tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(#last_block);
    })
    .unwrap();
    input.block.stmts.push(last_statement);
    if cases.is_empty() {
        let result = quote! {
            #[::core::prelude::v1::test]
            #input
        };
        result.into()
    } else {
        let mut functions = Vec::new();
        for (e, b, n) in cases.into_iter() {
            let mut f = input.clone();
            f.sig.ident = syn::Ident::new(&format!("{}_{n}", f.sig.ident), f.sig.ident.span());
            for stmt in f.block.stmts.iter_mut() {
                if let Stmt::Local(local) = stmt {
                    if let Some((_, e)) = &mut local.init {
                        let is_case_match = if let syn::Expr::Match(m) = e.as_mut() {
                            if let syn::Expr::Path(p) = m.expr.as_ref() {
                                if let Some(i) = p.path.get_ident() {
                                    format!("{i}") == "CASE"
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        if is_case_match {
                            *e = Box::new(b);
                            break;
                        }
                    }
                }
            }
            f.block.stmts.insert(
                0,
                syn::parse2(quote! {
                    const CASE: &str = #e;
                })
                .unwrap(),
            );
            functions.push(quote! {
               #[::core::prelude::v1::test]
               #f
            });
        }
        let result = quote! {
            #( #functions )*
        };
        result.into()
    }
}

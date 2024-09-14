use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, ExprMethodCall, ExprTry, Ident, ItemFn, LitStr, ReturnType, Signature, Token, Type};

struct RouteArgs {
    method: syn::Ident,
    path: syn::LitStr,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method = input.parse()?;
        input.parse::<Token![,]>()?;
        let path = input.parse()?;
        Ok(RouteArgs { method, path })
    }
}

fn transform_serve_call(expr: &Expr) -> Option<quote::__private::TokenStream> {
    match expr {
        Expr::MethodCall(ExprMethodCall { receiver, method, args, .. }) => {
            if method.to_string() == "serve" {
                Some(quote! { #receiver.#method(#args).await })
            } else {
                None
            }
        }
        Expr::Try(ExprTry { expr, .. }) => {
            if let Expr::MethodCall(ExprMethodCall { receiver, method, args, .. }) = expr.as_ref() {
                if method.to_string() == "serve" {
                    Some(quote! { #receiver.#method(#args).await? })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(item);
    let Signature { ident, generics, inputs, output, .. } = sig;

    let return_type = match output {
        ReturnType::Default => quote! { ::std::io::Result<()> },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let mut new_body = Vec::new();

    for stmt in block.stmts.iter() {
        match stmt {
            syn::Stmt::Expr(expr, _) => {
                if let Some(new_expr) = transform_serve_call(expr) {
                    new_body.push(new_expr);
                } else {
                    new_body.push(stmt.to_token_stream());
                }
            }
            _ => new_body.push(stmt.to_token_stream()),
        }
    }

    // for stmt in block.stmts.iter() {
    //     if let syn::Stmt::Expr(Expr::MethodCall(ExprMethodCall { receiver, method, args, .. }), _) = stmt {
    //         if method.to_string() == "serve" {
    //             new_body.push(quote! { #receiver.#method(#args).await });
    //         } else if method.to_string() == "service" {
    //             let new_arg = quote! { #args() };
    //             new_body.push(quote! { router.service(#new_arg); });
    //         } else {
    //             new_body.push(stmt.to_token_stream());
    //         }
    //     } else {
    //         new_body.push(stmt.to_token_stream());
    //     }
    // }

    let gen = quote! {
        #(#attrs)*
        #vis fn #ident #generics(#inputs) -> #return_type {
            let rt = ::tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                #(#new_body)*;
                Ok(())
            })
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as RouteArgs);
    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(item as ItemFn);
    let Signature { ident, generics, inputs, output, .. } = sig.to_owned();
    let method = &args.method;
    let path = &args.path;

    let (is_result, _) = match &output {
        ReturnType::Default => (false, None),
        ReturnType::Type(_, ty) => {
            if let Type::Path(type_path) = &**ty {
                if type_path.path.segments.last().map_or(false, |s| s.ident == "Result") {
                    (true, Some(quote! { #ty }))
                } else {
                    (false, Some(quote! { #ty }))
                }
            } else {
                (false, Some(quote! { #ty }))
            }
        }
    };

    // Ensure the function is async
    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig, "Route handler must be async").to_compile_error().into();
    }

    let route_fn_ident = format_ident!("__ROUTE_{}", ident.to_string().to_uppercase());

    let handler_body = if is_result {
        quote! {
            #ident(req).await
        }
    } else {
        quote! {
            Ok(#ident(req).await)
        }
    };

    let gen = quote! {
        #(#attrs)*
        #vis async fn #ident #generics(#inputs) #output #block

        pub fn #route_fn_ident(router: &mut ::server::Router) {
            router.service(::server::Method::#method, #path.to_string(),
            |req: ::server::Request| Box::pin(async move {
                #handler_body
            }));
        }
    };

    gen.into()
}

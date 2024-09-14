use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned, ToTokens};

use syn::{
    parse::Parse,
    parse::ParseStream,
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, Eq},
    Expr, ExprMethodCall, ExprTry, Ident, ItemFn, Pat, ReturnType, Signature, Type,
};

struct RouteArgs {
    method: syn::Ident,
    path: syn::LitStr,
    default: syn::LitBool,
}

struct RoutesInput {
    routes: Punctuated<Ident, Comma>,
}

impl Parse for RoutesInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let routes = Punctuated::<Ident, Comma>::parse_terminated(input)?;
        Ok(RoutesInput { routes })
    }
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut method: Option<syn::Ident> = None;
        let mut path: Option<syn::LitStr> = None;
        let mut default: syn::LitBool = syn::LitBool::new(false, input.span());

        while !input.is_empty() {
            if input.peek(syn::Ident) && input.peek2(Eq) {
                let ident: syn::Ident = input.parse()?;
                if ident == "default" {
                    input.parse::<Eq>()?;
                    default = input.parse()?;
                }
            } else if method.is_none() && input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                let method_str = ident.to_string().to_uppercase();
                method = Some(syn::Ident::new(&method_str, ident.span()));
            } else if path.is_none() && input.peek(syn::LitStr) {
                path = Some(input.parse()?);
            }

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            } else {
                break;
            }
        }

        let method = method.unwrap_or_else(|| syn::Ident::new("ALL", input.span()));
        let path = path.unwrap_or_else(|| syn::LitStr::new("/", input.span()));

        Ok(RouteArgs { method, path, default })
    }
}

fn transform_serve_call(expr: &Expr) -> Option<quote::__private::TokenStream> {
    match expr {
        Expr::MethodCall(ExprMethodCall { receiver, method, args, .. }) => {
            if method.to_string() == "serve" {
                Some(quote! { #receiver.#method(#args).await })
            } else if method.to_string() == "service" {
                if let Expr::Path(path) = &args[0] {
                    let ident = &path.path.segments.last().unwrap().ident;
                    let route_fn_ident = format_ident!("__ROUTE_{}", ident.to_string().to_uppercase());
                    Some(quote! { #route_fn_ident(&mut #receiver); })
                } else {
                    None
                }
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
    let Signature { ident, inputs, output, .. } = sig.clone();

    let method = &args.method;
    let path = &args.path;
    let is_default = &args.default;

    let is_result = match &output {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Path(type_path) if type_path.path.segments.last().map_or(false, |s| s.ident == "Result" || s.ident == "HttpResponse")),
    };

    let parameters: Vec<_> = path
        .value()
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                Some(segment[1..segment.len() - 1].to_string())
            } else {
                None
            }
        })
        .collect();

    if inputs.len() != parameters.len() + 1 {
        return syn::Error::new_spanned(sig, format!("Route handler must have {} arguments", parameters.len() + 1))
            .to_compile_error()
            .into();
    }

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig, "Route handler must be async").to_compile_error().into();
    }

    let route_fn_ident = quote::format_ident!("__ROUTE_{}", ident.to_string().to_uppercase());

    // Generate parameter extraction
    let mut param_extractions = Vec::new();
    let mut function_args = vec![quote!(req.clone())];
    for (_, input) in inputs.iter().enumerate().skip(1) {
        if let syn::FnArg::Typed(pat_type) = input {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let param_name = &pat_ident.ident;
                let param_str = param_name.to_string();
                param_extractions.push(quote! {
                    let #param_name = req.route_param(#param_str)
                        .or_else(|| req.query_param(#param_str))
                        .cloned()
                        .unwrap_or_default();
                });
                function_args.push(quote!(#param_name));
            }
        }
    }

    let param_extraction = quote! { #(#param_extractions)* };
    let function_call = quote! { #ident(#(#function_args),*) };

    let handler_body = if is_result {
        quote_spanned! {Span::call_site()=>
            #param_extraction
            match #function_call.await {
                Ok(responder) => Ok(Box::new(responder) as Box<dyn crate::Responder>),
                Err(err) => Err(err),
            }
        }
    } else {
        quote_spanned! {Span::call_site()=>
            #param_extraction
            Ok(Box::new(#function_call.await) as Box<dyn crate::Responder>)
        }
    };

    let gen = quote! {
        #(#attrs)*
        #vis #sig #block

        pub fn #route_fn_ident(router: &mut crate::Router) {
            if #is_default {
                router.add_default(|req: crate::Request| Box::pin(async move {
                    #handler_body
                }));
            } else {
                router.add(crate::Method::#method, #path.to_string(),
                |req: crate::Request| Box::pin(async move {
                    #handler_body
                }));
            }
        }
    };

    gen.into()
}

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RoutesInput);
    let routes = input.routes;

    let route_services = routes.iter().map(|route| {
        let route_fn_ident = format_ident!("__ROUTE_{}", route.to_string().to_uppercase());
        quote! {
            #route_fn_ident(&mut router);
        }
    });

    let gen = quote! {
        {
            let mut router = crate::Router::new();
            #(#route_services)*
            router
        }
    };

    gen.into()
}

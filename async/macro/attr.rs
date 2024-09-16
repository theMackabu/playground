use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType, Signature};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn { attrs, vis, sig, block } = parse_macro_input!(item);
    let Signature { ident, generics, inputs, output, .. } = sig;

    let return_type = match output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let gen = quote! {
        #(#attrs)*
        #vis fn #ident #generics(#inputs) -> #return_type {
            let mini_tokio = ::mini_tokio::MiniTokio::new();
            mini_tokio.spawn(async { #block });
            mini_tokio.run();
        }
    };

    gen.into()
}

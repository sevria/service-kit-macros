use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    quote! {
        #[::service_kit::tokio::main(crate = "::service_kit::tokio")]
        #func
    }
    .into()
}

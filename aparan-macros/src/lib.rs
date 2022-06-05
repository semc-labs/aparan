use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _metadata = parse_macro_input!(metadata as TokenStream);
    let input = parse_macro_input!(input as ItemFn);

    let content = input.block;

    let output: TokenStream = {
        quote! {
            fn main() {
                let system = aparan::system::System::new();
                #content
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

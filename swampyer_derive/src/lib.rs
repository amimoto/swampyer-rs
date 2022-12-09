#![allow(unused_imports, unreachable_code)]
#![allow(unused_variables, dead_code, unused_must_use)]

extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Wamp)]
pub fn derive_decode_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let ident_str = ident.to_string();

    let tokens = quote! {

        impl WampSerializable for #ident {
            fn encode(&self, encoder:&mut Encoder<&mut WampWrite> ) {
                encoder.encode(self);
            }

            fn debug_name(&self) -> &str {
                #ident_str
            }
        }

        impl From<Arc< #ident >> for WampData {
            fn from(d:Arc< #ident >) -> Self {
                WampData::Serializable(d)
            }
        }

        impl From< #ident > for WampData {
            fn from(d: #ident ) -> Self {
                WampData::Serializable(Arc::new(d))
            }
        }
    };

    TokenStream::from(tokens)
}




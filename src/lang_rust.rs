use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::tokens::ReversedOrderStream;

pub fn lang_rust(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as ReversedOrderStream).0.into()
}

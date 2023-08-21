use proc_macro::TokenStream;

use syn::parse_macro_input;

use crate::tokens::ReversedTextStream;

pub fn gnal_tsur(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as ReversedTextStream).0.into()
}

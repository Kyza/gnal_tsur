#![doc = include_str!("../README.md")]
#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;

mod gnal_tsur;
mod lang_rust;
mod tokens;

#[proc_macro]
/// Reverses the order of the characters.
pub fn gnal_tsur(input: TokenStream) -> TokenStream {
	gnal_tsur::gnal_tsur(input)
}

#[proc_macro]
/// Reverses the order of the tokens.
pub fn lang_rust(input: TokenStream) -> TokenStream {
	lang_rust::lang_rust(input)
}

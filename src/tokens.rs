use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};

use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseBuffer, ParseStream, Parser};

fn stream_to_punct(stream: &TokenStream) -> Result<Punct, syn::Error> {
	Parser::parse2(
		|input: &ParseBuffer<'_>| input.parse::<Punct>(),
		stream.clone(),
	)
}

/// [Punct] uses [Spacing] to determine when to put them next to each other.
/// Since the stream gets reversed, this gets messed up.
/// This function iterates over a non-reversed layer and inverts the [Spacing] of joint punctuations.
fn fix_punct_spacing(layer: Vec<TokenStream>) -> Vec<TokenStream> {
	let mut fixed: Vec<TokenStream> = vec![];

	let mut in_joint = false;

	for stream in layer.clone() {
		if let Ok(punct) = stream_to_punct(&stream) {
			if in_joint {
				fixed.push(
					Punct::new(punct.as_char(), Spacing::Joint)
						.to_token_stream(),
				)
			} else {
				fixed.push(
					Punct::new(punct.as_char(), Spacing::Alone)
						.to_token_stream(),
				)
			}

			in_joint = punct.spacing() == Spacing::Joint;
		} else {
			in_joint = false;
			fixed.push(stream);
		}
	}

	fixed
}

fn reverse_deep(layer: Vec<TokenTree>, only_order: bool) -> TokenStream {
	let mut reversed_layer = vec![];

	for tree in layer.clone() {
		reversed_layer.push(match tree {
			TokenTree::Group(value) => {
				let reversed = reverse_deep(
					value.stream().into_iter().collect::<Vec<_>>(),
					only_order,
				);
				match value.delimiter() {
					proc_macro2::Delimiter::Parenthesis => {
						quote! { ( #reversed ) }
					}
					proc_macro2::Delimiter::Brace => quote! { { #reversed } },
					proc_macro2::Delimiter::Bracket => {
						quote! { [ #reversed ] }
					}
					proc_macro2::Delimiter::None => reversed,
				}
			}
			TokenTree::Ident(value) => {
				if only_order {
					value.into_token_stream()
				} else {
					value
						.to_string()
						.chars()
						.rev()
						.collect::<String>()
						.parse()
						.unwrap()
				}
			}
			// Punct is always one character.
			TokenTree::Punct(value) => value.into_token_stream(),
			TokenTree::Literal(value) => {
				if only_order {
					value.into_token_stream()
				} else {
					value
						.to_string()
						.chars()
						.rev()
						.collect::<String>()
						.parse()
						.unwrap()
				}
			}
		});
	}

	let mut fixed_reversed: Vec<TokenStream> =
		fix_punct_spacing(reversed_layer);

	fixed_reversed.reverse();

	TokenStream::from_iter(fixed_reversed)
}

#[derive(Debug, Clone)]
pub struct ReversedOrderStream(pub TokenStream);

impl Parse for ReversedOrderStream {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut tokens = vec![];

		while !input.is_empty() {
			tokens.push(input.parse::<TokenTree>()?);
		}

		Ok(ReversedOrderStream(reverse_deep(tokens, true)))
	}
}

#[derive(Debug, Clone)]
pub struct ReversedTextStream(pub TokenStream);

impl Parse for ReversedTextStream {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut tokens = vec![];

		while !input.is_empty() {
			tokens.push(input.parse::<TokenTree>()?);
		}

		Ok(ReversedTextStream(reverse_deep(tokens, false)))
	}
}

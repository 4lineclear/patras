//! tokio-postgres-macro

use proc_macro::TokenStream;

pub(crate) mod parse;

/// Create postgres macros
#[proc_macro]
pub fn topoma(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    println!("{:?}", tokens.next());
    println!("{:?}", tokens.next());
    println!("{:?}", tokens.next());
    println!("{:?}", tokens.next());

    TokenStream::new()
}

fn parse_paths() {}

// TODO: Everything

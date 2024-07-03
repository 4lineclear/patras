//! tokio-postgres-macro

use parse::SqlMonolith;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Parses the tokenstream
pub(crate) mod parse;
/// Transforms the parse tokens back into tokens
pub(crate) mod sql;

/// Create postgres macros
#[proc_macro]
pub fn topoma(input: TokenStream) -> TokenStream {
    let sql = parse_macro_input!(input as SqlMonolith);
    [sql.table.to_tokens(), sql.drop.to_tokens()]
        .into_iter()
        .chain(sql.queries.into_iter().map(|s| s.to_tokens()))
        .collect::<proc_macro2::TokenStream>()
        .into()
}

// TODO: Everything

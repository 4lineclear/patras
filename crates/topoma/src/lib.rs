//! tokio-postgres-macro

use parse::{SqlMonolith, SqlUnit};
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Parses the tokenstream
pub(crate) mod parse;

/// Does simple validation on the given sql statements
///
/// # Syntax
///
/// A custom syntax is provided:
///
/// ```ignore
/// # fn main() {
/// topoma! [
///     "Comment which retains fromatting within the macro",
///     $STATEMENT_NAME_ONE >> r"$SQL",
///     /// Another comment which breaks rust formatting within the macro
///     $STATEMENT_NAME_TWO << "$PATH_TO_FILE",
///     // ...
/// ];
/// # }
///
/// ```
///
/// `$SQL` refers to an sql file in the string,
///
/// `$PATH_TO_FILE` refers to an sql file's path relative to `CARGO_MANIFEST_DIR`,
#[proc_macro]
pub fn topoma(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as SqlMonolith)
        .statements
        .into_iter()
        .map(SqlUnit::into_tokens)
        .collect::<proc_macro2::TokenStream>()
        .into()
}

use std::{
    env::{self, VarError},
    fmt::{Debug, Display},
    fs,
    io::{self, Error as IoError},
    path::PathBuf,
};

use proc_macro2::TokenStream;
use quote::quote;
use sqlparser::{
    ast::Statement,
    dialect::PostgreSqlDialect,
    parser::{Parser as SqlParser, ParserError},
};
use syn::{parse::Parse, Ident, LitStr, Token};

#[derive(Debug)]
pub struct SqlMonolith {
    pub table: SqlUnit,
    pub drop: SqlUnit,
    pub queries: Vec<SqlUnit>,
}

impl Parse for SqlMonolith {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tokens = input
            .parse_terminated(SqlUnit::parse, Token![,])?
            .into_iter();
        let input_error = || input.error("Expect table create and table drop statements");

        let table = tokens.next().ok_or_else(input_error)?;
        let drop = tokens.next().ok_or_else(input_error)?;
        let queries = tokens.collect();

        Ok(Self {
            table,
            drop,
            queries,
        })
    }
}

#[derive(Debug)]
pub struct SqlUnit {
    pub name: Ident,
    #[allow(dead_code)]
    pub sql: Vec<Statement>,
}

impl Parse for SqlUnit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        macro_rules! parse {
            ($t: tt) => {
                input.parse::<Token![$t]>()?
            };
            () => {
                input.parse()?
            };
        }
        let name: Ident = parse!();

        if input.peek(Token![>]) {
            parse!(>);
            parse!(>);
            let string = input.parse::<LitStr>()?;
            let source = string.value();
            let sql = parse_embeded(&source)
                .map_err(|e| syn::Error::new_spanned(string, e.to_string()))?;
            Ok(Self { name, sql })
        } else if input.peek(Token![<]) {
            parse!(<);
            parse!(<);
            let string = input.parse::<LitStr>()?;
            let path = string.value();
            let sql = parse_from_path(&path)
                .map_err(|e| syn::Error::new_spanned(string, e.to_string()))?;
            Ok(Self { name, sql })
        } else {
            const MESSAGE: &str = "Unexpected token: \
                valid tokens are '>>' for embeded sql or '<<' to pull from file.";
            Err(input.error(MESSAGE))
        }
    }
}

impl SqlUnit {
    pub fn to_tokens(self) -> TokenStream {
        let name = self.name;
        let mut sql = String::new();
        for s in self.sql.into_iter() {
            sql.push_str(&s.to_string());
            sql.push(' ');
        }
        quote! {
            pub const #name: &str = #sql;
        }
    }
}

fn parse_sql(source: &str) -> Result<Vec<Statement>, ParserError> {
    let statements = SqlParser::parse_sql(&PostgreSqlDialect {}, source)?;
    Ok(statements)
}

fn parse_embeded(source: &str) -> Result<Vec<Statement>, ParseEmbededError> {
    Ok(parse_sql(source)?)
}

fn parse_from_path(path: &str) -> Result<Vec<Statement>, ParsePathError> {
    if path.is_empty() {
        return Err(ParsePathError::EmptyString);
    }
    let path = {
        let mut buf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        if path.ends_with(".sql") {
            buf.push(path);
        } else {
            buf.push(path.to_owned() + ".sql");
        };
        buf
    };

    if !path.exists() {
        let error = format!("Unable to find specified file '{}'", path.to_string_lossy());
        return Err(io::Error::new(io::ErrorKind::NotFound, error).into());
    }

    Ok(parse_sql(&fs::read_to_string(path)?)?)
}

macro_rules! impl_error {
    ($e:ident, from = [$($from:ident ( $($n:ident)? $($m:literal)? )),*]) => {
        impl Display for $e {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {$(
                    $e::$from $(($n))? => Display::fmt($($n)? $($m)?, f),
                )*}
            }
        }
        $( impl_error!($e, $from, $($n)?); )*
    };
    ($e:ident, $from:ident, $n:tt) => {
        impl From<$from> for $e {
            fn from(value: $from) -> Self {
                $e::$from(value)
            }
        }
    };
    // branch for no 'impl From'
    ($e:ident, $from:ident,) => {};
}

pub enum ParseEmbededError {
    ParserError(ParserError),
}

impl_error!(ParseEmbededError, from = [ParserError(e)]);

pub enum ParsePathError {
    ParserError(ParserError),
    IoError(IoError),
    VarError(VarError),
    EmptyString,
}

impl_error!(
    ParsePathError,
    from = [
        ParserError(e),
        IoError(e),
        VarError(e),
        EmptyString("Empty path not allowed")
    ]
);

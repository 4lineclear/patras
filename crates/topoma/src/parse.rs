use std::{
    env::{self, VarError},
    fmt::{Debug, Display},
    fs,
    io::{self, Error as IoError},
    path::PathBuf,
};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use sqlparser::{
    ast::Statement,
    dialect::PostgreSqlDialect,
    parser::{Parser as SqlParser, ParserError},
};
use syn::{parse::Parse, Attribute, Ident, LitStr, Token};

#[derive(Debug)]
pub struct SqlMonolith {
    pub statements: Vec<SqlUnit>,
}

impl Parse for SqlMonolith {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            statements: input
                .parse_terminated(SqlUnit::parse, Token![,])?
                .into_iter()
                .collect(),
        })
    }
}

#[derive(Debug)]
pub struct SqlUnit {
    pub name: Ident,
    pub attributes: Vec<Attribute>,
    pub sql: Vec<Statement>,
}

impl Parse for SqlUnit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        macro_rules! parse {
            ($t: tt) => {
                input.parse::<Token![$t]>()
            };
            () => {
                input.parse()
            };
        }
        let comments = Attribute::parse_outer(input)?;
        let name: Ident = parse!()?;

        if input.peek(Token![>]) {
            parse!(>)?;
            parse!(>)?;

            let string: LitStr = parse!()?;
            let source = string.value();
            let sql = parse_embeded(&source)
                .map_err(|e| syn::Error::new_spanned(string, e.to_string()))?;
            Ok(Self {
                name,
                attributes: comments,
                sql,
            })
        } else if input.peek(Token![<]) {
            parse!(<)?;
            parse!(<)?;

            let string: LitStr = parse!()?;
            let path = string.value();
            let sql = parse_from_path(&path)
                .map_err(|e| syn::Error::new_spanned(string, e.to_string()))?;
            Ok(Self {
                name,
                attributes: comments,
                sql,
            })
        } else {
            const MESSAGE: &str = "Unexpected token: \
                valid tokens are '>>' for embeded sql or '<<' to pull from file.";
            Err(input.error(MESSAGE))
        }
    }
}

impl SqlUnit {
    pub fn into_tokens(self) -> TokenStream {
        let name = self.name;
        let mut attributes = TokenStream::new();
        for attr in self.attributes {
            attr.to_tokens(&mut attributes);
        }
        let mut sql = String::new();
        for s in self.sql {
            sql.push_str(&s.to_string());
            sql.push(' ');
        }
        quote! {
            #attributes
            pub const #name: &str = #sql;
        }
    }
}

fn parse_sql(source: &str) -> Result<Vec<Statement>, ParserError> {
    let statements = SqlParser::parse_sql(&PostgreSqlDialect {}, source)?;
    Ok(statements)
}

fn parse_embeded(source: &str) -> Result<Vec<Statement>, EmbededError> {
    Ok(parse_sql(source)?)
}

fn parse_from_path(path: &str) -> Result<Vec<Statement>, PathError> {
    if path.is_empty() {
        return Err(PathError::EmptyString);
    }
    let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    if !path.exists() {
        let error = format!("Unable to find specified file '{}'", path.to_string_lossy());
        return Err(io::Error::new(io::ErrorKind::NotFound, error).into());
    }

    Ok(parse_sql(&fs::read_to_string(path)?)?)
}

// probably a needless usage of a macro
macro_rules! impl_error {
    // somewhat whacky branching done here
    ($e:ident, from = [$($from:ident ( $($n:ident)? $($m:literal)? )),*]) => {
        impl Display for $e {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {$(
                    $e::$from $(($n))? => Display::fmt($($n)? $($m)?, f),
                )*}
            }
        }
        $( impl_error!($e, $from $(,$n)?); )*
    };
    ($e:ident, $from:ident, $n:tt) => {
        impl From<$from> for $e {
            fn from(value: $from) -> Self {
                $e::$from(value)
            }
        }
    };
    // branch for no 'impl From'
    ($e:ident, $from:ident) => {};
}

pub enum EmbededError {
    ParserError(ParserError),
}

impl_error!(EmbededError, from = [ParserError(e)]);

pub enum PathError {
    ParserError(ParserError),
    IoError(IoError),
    VarError(VarError),
    EmptyString,
}

impl_error!(
    PathError,
    from = [
        ParserError(e),
        IoError(e),
        VarError(e),
        EmptyString("Empty path not allowed")
    ]
);

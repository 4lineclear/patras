use syn::{parse::Parse, Ident};

pub struct SqlMonolith {
    table: (),
    drop: (),
}

pub struct SqlUnit {
    name: Ident,
}

impl Parse for SqlMonolith {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            table: (),
            drop: (),
        })
    }
}

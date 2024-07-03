use crate::parse::SqlMonolith;

#[allow(dead_code)]
pub fn parse(SqlMonolith { .. }: SqlMonolith) -> Result<(), ()> {
    // match table.sql.first() {
    //     Some(&Statement::CreateTable { ref columns, .. }) => {
    //         let _ = columns;
    //     }
    //     _ => todo!(),
    // }
    Ok(())
}

use crate::query::statement::*;
#[derive(Default)]
pub struct Table {
    pub value: Statement,
    pub alias: Option<String>,
}

impl Table {
    pub fn new<T>(value: T) -> Table
    where T: ToStatement {
        Table {
            value: value.to_statement(),
            alias: None
        }
    }

    pub fn empty() -> Table{
        Table {
            value: Statement::NoStatement,
            alias: None
        }
    }
}
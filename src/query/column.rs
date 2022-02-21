use crate::query::statement::*;

pub struct Column {
    pub value: Statement,
    pub alias: Option<String>,
}

impl Column {
    pub fn new<T>(value: T) -> Column
    where T: ToStatement {
        Column {
            value: value.to_statement(),
            alias: None
        }
    }

    pub fn empty() -> Column{
        Column {
            value: Statement::NoStatement,
            alias: None
        }
    }
}
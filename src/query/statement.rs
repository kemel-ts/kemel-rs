use crate::{query::{Query, Raw}, ForceValue};

pub enum Statement {
    NoStatement,
    Null,
    Column(String),
    StringValue(String),
    Query(Box<Query>),
    Raw(String),
}

pub trait ToStatement {
    fn to_statement(self) -> Statement;
}

impl ToStatement for Statement {
    fn to_statement(self) -> Statement {
        self
    }
}

impl ToStatement for String {
    fn to_statement(self) -> Statement {
        Statement::StringValue(self)
    }
}

impl ToStatement for &str {    
    fn to_statement(self) -> Statement {
        Statement::StringValue(self.into())
    }
}

impl ToStatement for Query {    
    fn to_statement(self) -> Statement {
        Statement::Query(Box::new(self))
    }
}

impl ToStatement for Raw {
    fn to_statement(self) -> Statement {
        Statement::Raw(self.value)
    }
}

impl ToStatement for ForceValue {
    fn to_statement(self) -> Statement {
        match self {
            ForceValue::Null => Statement::Null,
            ForceValue::Column(column) => Statement::Column(column),
            ForceValue::StringValue(value) => Statement::StringValue(value),
        }
    }
}

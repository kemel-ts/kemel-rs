use crate::{Statement, ToStatement};

pub struct Raw {
    pub value: String,
    pub statements: Vec<Statement>
}

impl Raw {
    pub fn new(value: String) -> Raw {
        Raw { value, statements: Vec::new() }
    }

    pub fn with_parameter<T>(value: String, arg: T) -> Raw
    where T: ToStatement {
        let mut raw = Raw::new(value);

        raw.statements.push(arg.to_statement());

        raw
    }

    pub fn with_parameters<T>(value: String, args: Vec<T>) -> Raw
    where T: ToStatement {
        let mut raw = Raw::new(value);

        for arg in args {
            raw.statements.push(arg.to_statement());
        }

        raw
    }
}
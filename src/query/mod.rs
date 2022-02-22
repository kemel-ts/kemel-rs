mod statement;
mod table;
mod column;
mod raw;
mod force_value;
mod constraint;

use std::{rc::Rc, cell::RefCell};

pub use statement::*;
pub use table::*;
pub use column::*;
pub use raw::*;
pub use force_value::*;
pub use constraint::*;


pub struct Query {
    pub table: Table,
    pub columns: Vec<Column>,
    pub constraints: Vec<Rc<Constraint>>,
}

impl Query {

    pub fn new() -> Query {
        Query {
            table: Table::empty(),
            columns: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn from<T>(mut self, table: T) -> Query
    where T: ToStatement
    {
        self.table = Table::new(table);

        self
    }

    pub fn select<T>(mut self, column: T) -> Query
    where T: ToStatement
    {
        self.columns.push(Column::new(column));

        self
    }

    pub fn select_range<T>(mut self, columns: Vec<T>) -> Query
    where T: ToStatement
    {
        for column in columns {
            self.columns.push(Column::new(column));
        }

        self
    }

    pub fn condition<T>(self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        let queryRef = RefCell::new(Rc::new(self));        
        
        let constraint;
        {
            constraint = Rc::new(Constraint::new(
                &queryRef.borrow_mut().clone(),
                if queryRef.borrow_mut().constraints.len() == 0 { ConstraintType::NoType } else { ConstraintType::And },
                value,
            ));
    
        }

        queryRef.borrow_mut().clone().constraints.push(constraint.clone());

        constraint
    }

    pub fn and<T>(self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        self.condition(value)
    }

    pub fn or<T>(self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        Rc::new(Constraint::new(
            &Rc::new(self),
            ConstraintType::Or,
            value,
        ))
    }
    
}

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;

    use crate::query::*;

    #[test]
    fn should_store_string_table() {
        let query = Query::new().from("tableName");

        if let Statement::StringValue(tb) = query.table.value {
            assert_eq!(tb, String::from("tableName"));
        } else {
            panic!("Unknown TableContainer");
        }
    }

    #[test]
    fn should_store_string_column() {
        let query = Query::new().select("columnName");

        if let Some(column) = query.columns.get(0) {
            if let Statement::StringValue(tb) = column.value.borrow().clone() {
                assert_eq!(tb.to_owned(), String::from("columnName"));
            } else {
                panic!("Unknown TableContainer");
            }
        } else {
            panic!("Column array is empty");
        }
    }

    #[test]
    fn should_store_string_columns() {
        let query = Query::new().select_range(vec!["columnName1", "columnName2"]);

        let mut i = 1;
        for column in query.columns {
            if let Statement::StringValue(tb) = column.value {
                assert_eq!(tb, String::from(format!("columnName{}", i)));
            } else {
                panic!("Unknown TableContainer");
            }
            i += 1;
        }
    }
}

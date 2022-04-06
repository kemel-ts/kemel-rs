mod statement;
mod table;
mod column;
mod raw;
mod force_value;
mod constraint;

use std::{rc::Rc, cell::RefCell, borrow::{BorrowMut, Borrow}};

pub use statement::*;
pub use table::*;
pub use column::*;
pub use raw::*;
pub use force_value::*;
pub use constraint::*;

#[derive(Default)]
pub struct Query {
    pub table: Table,
    pub columns: Vec<Column>,
    pub constraints: Vec<Constraint>,
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

    /*pub fn condition<T>(&mut self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        let constraint_type = if self.constraints.len() == 0 { ConstraintType::NoType } else { ConstraintType::And };
        
        let constraint;
        {
            constraint = Rc::new(Constraint::new(
                constraint_type,
                value,
            ));
    
        }

        let ret = Rc::clone(&constraint);

        self.constraints.push(constraint);
        
        ret
    }

    pub fn where_<T>(&mut self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        self.condition(value)
    }

    pub fn and<T>(&mut self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        self.condition(value)
    }

    pub fn or<T>(&mut self, value: T) -> Rc<Constraint>
    where T: ToStatement {
        Rc::new(Constraint::new(
            ConstraintType::Or,
            value,
        ))
    }*/







    pub fn and_equal<T, Y>(&mut self, left_value: Y, right_value: T) -> &Query
    where T: ToStatement, Y: ToStatement {
        self.constraints.push(
            Constraint::and_equal(left_value, right_value)
        );
        self
    }

    // pub fn and_different<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_greater_than<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_less_than<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_greater_than_or_equal<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_less_than_or_equal<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_between<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_like<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_not_like<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_in_<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_not_in<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_is_null<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    // pub fn and_is_not_null<T, Y>(mut self, rightValue: T, leftValue: Y) -> Query
    // where T: ToStatement, Y: ToStatement {
    //     self.and(rightValue).equal(leftValue);
    //     self
    // }

    
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

    #[test]
    fn should_store_equal_constraint() {
        let mut query = Query::new()
            .from("table1")
            .select_range(vec!["columnName1", "columnName2"]);

        query.and_equal("columnA", 35);

        let mut i = 1;
        for column in query.columns {
            if let Statement::StringValue(tb) = &column.value {
                assert_eq!(*tb, String::from(format!("columnName{}", i)));
            } else {
                panic!("Unknown TableContainer");
            }
            i += 1;
        }
    }
}

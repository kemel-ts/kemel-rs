mod statement;
mod table;
mod column;
mod raw;
mod force_value;

pub use statement::*;
pub use table::*;
pub use column::*;
pub use raw::*;
pub use force_value::*;

pub struct Query {
    table: Table,
    columns: Vec<Column>,
}

impl Query {

    pub fn new() -> Query {
        Query {
            table: Table::empty(),
            columns: Vec::new(),
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

    pub fn condition<T>(mut self, a: T) {

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

use std::{rc::Rc, cell::RefCell};

use crate::{query::statement::*, Query};

pub enum ConstraintType {
    NoType,
    And,
    Or
}

pub enum Comparison {
    NoComparison,
    Equal,
    Different,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Between,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

pub struct Constraint {
    pub parent: Option<Rc<RefCell<Query>>>,
    pub constraint_type: ConstraintType,
    pub right_value: Statement,
    pub comparison: Comparison,
    pub left_values: Vec<Statement>,
}

impl Constraint {
    pub fn new<T>(parent: Rc<RefCell<Query>>, constraint_type: ConstraintType, value: T) -> Constraint
    where T: ToStatement {
        let constraint = Constraint {
            parent: Some(parent),
            constraint_type,
            right_value: value.to_statement(),
            comparison: Comparison::NoComparison,
            left_values: Vec::new(),
        };

        constraint
    }

    pub fn and<T>(parent: Rc<RefCell<Query>>, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::And, value)
    }
    
    pub fn or<T>(parent: Rc<RefCell<Query>>, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::Or, value)
    }

    pub fn get_parent(self) -> Rc<RefCell<Query>> {
        match self.parent {
            Some(query_ref) => Rc::clone(&query_ref),
            None => panic!("The parent gone"),
        }
    }

    fn set_comparison_no_value(mut self, comparison: Comparison) -> Rc<RefCell<Query>> {
        self.comparison = comparison;

        self.get_parent()
    }

    fn set_comparison<T>(mut self, comparison: Comparison, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.comparison = comparison;
        self.left_values.push(value.to_statement());

        self.get_parent()
    }
    
    fn set_comparison_vec<T>(mut self, comparison: Comparison, values: Vec<T>) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.comparison = comparison;

        for value in values {
            self.left_values.push(value.to_statement());
        }

        Rc::clone(& self.get_parent())
    }

    pub fn equal<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::Equal, value)
    }

    pub fn different<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::Different, value)
    }

    pub fn greater_than<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::GreaterThan, value)
    }

    pub fn less_than<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::LessThan, value)
    }

    pub fn greater_than_or_equal<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::GreaterThanOrEqual, value)
    }

    pub fn less_than_or_equal<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::LessThanOrEqual, value)
    }

    pub fn between<T>(self, start: T, end: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison_vec(Comparison::Between, vec![start, end])
    }

    pub fn like<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::Like, value)
    }

    pub fn not_like<T>(self, value: T) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison(Comparison::NotLike, value)
    }

    pub fn in_<T>(self, values: Vec<T>) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison_vec(Comparison::In, values)
    }

    pub fn not_in<T>(self, values: Vec<T>) -> Rc<RefCell<Query>>
    where T: ToStatement {
        self.set_comparison_vec(Comparison::NotIn, values)
    }

    pub fn is_null(self) -> Rc<RefCell<Query>> {
        self.set_comparison_no_value(Comparison::IsNull)
    }

    pub fn is_not_null(self) -> Rc<RefCell<Query>> {
        self.set_comparison_no_value(Comparison::IsNotNull)
    }
}
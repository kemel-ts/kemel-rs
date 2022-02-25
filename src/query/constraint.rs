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
        let mut constraint = Constraint {
            parent: Option::None,
            constraint_type,
            right_value: value.to_statement(),
            comparison: Comparison::NoComparison,
            left_values: Vec::new(),
        };

        constraint.parent = Some(parent);

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

    pub fn equal<T>(mut self, value: T) -> Rc<RefCell<Query>> 
    where T: ToStatement {
        self.comparison = Comparison::Equal;
        self.left_values.push(value.to_statement());

        match self.parent {
            Some(query_ref) => Rc::clone(&query_ref),
            None => panic!("The parent gone"),
        }
    }
}
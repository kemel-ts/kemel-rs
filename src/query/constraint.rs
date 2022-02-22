use std::{rc::{Rc, Weak}, cell::RefCell};

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
    pub parent: RefCell<Option<Weak<Query>>>,
    pub constraint_type: ConstraintType,
    pub rightValue: Statement,
    pub comparison: Comparison,
    pub leftValues: Vec<Statement>,
}

impl Constraint {
    pub fn new<T>(parent: &Rc<Query>, constraint_type: ConstraintType, value: T) -> Constraint
    where T: ToStatement {
        let constraint = Constraint {
            parent: RefCell::new(None),
            constraint_type,
            rightValue: value.to_statement(),
            comparison: Comparison::NoComparison,
            leftValues: Vec::new(),
        };

        constraint.parent.borrow_mut().replace(Rc::downgrade(parent));

        constraint
    }

    pub fn and<T>(parent: &Rc<Query>, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::And, value)
    }
    
    pub fn or<T>(parent: &Rc<Query>, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::Or, value)
    }

    pub fn equal<T>(mut self, value: T) -> Weak<Query> 
    where T: ToStatement {
        self.comparison = Comparison::Equal;
        self.leftValues.push(value.to_statement());

        let t = self.parent.borrow();
        match t.as_ref() {
            Some(queryRef) => queryRef.clone(),
            None => panic!("The parent gone"),
        }
    }
}
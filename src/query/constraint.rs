use crate::query::statement::*;

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
    pub parent: Box<Query>,
    pub constraint_type: ConstraintType,
    pub rightValue: Statement,
    pub comparison: Comparison,
    pub leftValues: Vec<Statement>,
}

impl Constraint {
    pub fn new<T>(parent: &Query, constraint_type: ConstraintType, value: T) -> Constraint
    where T: ToStatement {
        Constraint {
            parent: Box::new(*parent),
            constraint_type,
            rightValue: value.to_statement(),
            comparison: Comparison::NoComparison,
            leftValues: Vec::new(),
        }
    }

    pub fn and<T>(parent: &Query, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::And, value)
    }
    
    pub fn or<T>(parent: &Query, value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(parent, ConstraintType::Or, value)
    }
}
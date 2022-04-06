use crate::query::statement::*;


#[derive(Clone, Copy)]
pub enum ConstraintType {
    NoType,
    And,
    Or
}


#[derive(Clone, Copy)]
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
    pub constraint_type: ConstraintType,
    pub right_value: Statement,
    pub comparison: Comparison,
    pub left_values: Vec<Statement>,
}

impl Constraint {
    pub fn new<T>(constraint_type: ConstraintType, value: T) -> Constraint
    where T: ToStatement {
        let constraint = Constraint {
            constraint_type,
            right_value: value.to_statement(),
            comparison: Comparison::NoComparison,
            left_values: Vec::new(),
        };

        constraint
    }

    pub fn and<T>(value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(ConstraintType::And, value)
    }
    
    pub fn or<T>(value: T) -> Constraint
    where T: ToStatement {
        Constraint::new(ConstraintType::Or, value)
    }    

    fn set_comparison_no_value(&mut self, comparison: Comparison) -> &Constraint {
        self.comparison = comparison;
        self
    }

    fn set_comparison<T>(&mut self, comparison: Comparison, value: T) -> &Constraint
    where T: ToStatement {
        self.comparison = comparison;
        self.left_values.push(value.to_statement());
        self
    }
    
    fn set_comparison_vec<T>(&mut self, comparison: Comparison, values: Vec<T>) -> &Constraint
    where T: ToStatement {
        self.comparison = comparison;

        for value in values {
            self.left_values.push(value.to_statement());
        }
        self
    }

    pub fn equal<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::Equal, value)
    }

    pub fn different<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::Different, value)
    }

    pub fn greater_than<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::GreaterThan, value)
    }

    pub fn less_than<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::LessThan, value)
    }

    pub fn greater_than_or_equal<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::GreaterThanOrEqual, value)
    }

    pub fn less_than_or_equal<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::LessThanOrEqual, value)
    }

    pub fn between<T>(&mut self, start: T, end: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison_vec(Comparison::Between, vec![start, end])
    }

    pub fn like<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::Like, value)
    }

    pub fn not_like<T>(&mut self, value: T) -> &Constraint
    where T: ToStatement {
        self.set_comparison(Comparison::NotLike, value)
    }

    pub fn in_<T>(&mut self, values: Vec<T>) -> &Constraint
    where T: ToStatement {
        self.set_comparison_vec(Comparison::In, values)
    }

    pub fn not_in<T>(&mut self, values: Vec<T>) -> &Constraint
    where T: ToStatement {
        self.set_comparison_vec(Comparison::NotIn, values)
    }

    pub fn is_null(&mut self) -> &Constraint {
        self.set_comparison_no_value(Comparison::IsNull)
    }

    pub fn is_not_null(&mut self) -> &Constraint {
        self.set_comparison_no_value(Comparison::IsNotNull)
    }

    pub fn and_equal<T, Y>(left_value: T, right_value: Y) -> Constraint
    where T: ToStatement, Y: ToStatement {
        let mut constraint = Constraint::and(left_value);
        constraint.equal(right_value);
        constraint
    }


}
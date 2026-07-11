use std::{collections::HashSet, fmt::Debug};

use crate::{
    types::attributes::Attributes,
    types::{attributes::Dependency, values::Value},
};

#[derive(Clone, Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Value(Value),
    Variable(Dependency),
    Operation(Operation),
}

impl Expression {
    pub fn evaluate(&self, attrs: &Attributes) -> Value {
        match self {
            Expression::Value(v) => v.to_simplest(attrs),
            Expression::Variable(v) => {
                let attr = attrs.map.get(v).expect("linked dep doesn't exist");
                attr.expression.evaluate(attrs)
            }
            Expression::Operation(operation) => operation.evaluate(attrs),
        }
    }

    pub fn get_dependencies(&self) -> HashSet<Dependency> {
        match self {
            Expression::Value(_) => HashSet::new(),
            Expression::Variable(dep) => {
                let mut deps = HashSet::new();
                deps.insert(dep.clone());
                deps
            }
            Expression::Operation(operation) => operation.get_dependencies(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Operation {
    value1: Box<Expression>,
    operator: Operator,
    value2: Box<Expression>,
}

impl Operation {
    pub fn new(value1: Box<Expression>, operator: Operator, value2: Box<Expression>) -> Self {
        Operation {
            value1,
            operator,
            value2,
        }
    }
}

impl Operation {
    pub fn evaluate(&self, attrs: &Attributes) -> Value {
        let v1 = self.value1.evaluate(attrs);
        let v2 = self.value2.evaluate(attrs);

        let res = match self.operator {
            Operator::Addition => v1.evaluate_addition(&v2),
            Operator::Subtraction => v1.evaluate_subtraction(&v2),
            Operator::Multiplication => v1.evaluate_multiplication(&v2),
            Operator::Division => v1.evaluate_division(&v2),
        };

        res
    }
    pub fn get_dependencies(&self) -> HashSet<Dependency> {
        let mut deps1 = self.value1.get_dependencies();
        let deps2 = self.value2.get_dependencies();

        deps1.extend(deps2);
        deps1
    }
}

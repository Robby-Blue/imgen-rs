use std::{collections::HashSet, fmt::Debug};

use crate::{
    layout_plan::Attributes,
    types::{
        attributes::Dependency,
        units::{Unit, UnitValue},
    },
};

#[derive(Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
pub enum Expression<T> {
    Value(UnitValue<T>),
    Variable(Dependency),
    Operation(Operation<T>),
}

impl<T: Unit<T>> Expression<T> {
    pub fn evaluate(&self, attrs: &Attributes<T>) -> UnitValue<T> {
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

impl<T> Debug for Expression<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Expression").finish()
    }
}

#[derive(Clone)]
pub struct Operation<T> {
    value1: Box<Expression<T>>,
    operator: Operator,
    value2: Box<Expression<T>>,
}

impl<T> Operation<T> {
    pub fn new(value1: Box<Expression<T>>, operator: Operator, value2: Box<Expression<T>>) -> Self {
        Operation {
            value1,
            operator,
            value2,
        }
    }
}

impl<T: Unit<T>> Operation<T> {
    pub fn evaluate(&self, attrs: &Attributes<T>) -> UnitValue<T> {
        let v1 = self.value1.evaluate(attrs);
        let v1_val = v1.value;

        let v2 = self.value2.evaluate(attrs);
        let v2_val = v2.value;

        let unit = v1.unit;

        let res = match self.operator {
            Operator::Addition => v1_val + v2_val,
            Operator::Subtraction => v1_val - v2_val,
            Operator::Multiplication => v1_val * v2_val,
            Operator::Division => v1_val / v2_val,
        };

        UnitValue::new_direct(res, unit)
    }
    pub fn get_dependencies(&self) -> HashSet<Dependency> {
        let mut deps1 = self.value1.get_dependencies();
        let deps2 = self.value2.get_dependencies();

        deps1.extend(deps2);
        deps1
    }
}

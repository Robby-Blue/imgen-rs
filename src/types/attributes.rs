use std::{collections::HashSet, fmt::Display};

use crate::types::{expressions::Expression, mults::AttributeDirection, units::Unit};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AttributeKey {
    pub attr_type: AttributeType,
    pub direction: AttributeDirection,
}

impl Display for AttributeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.attr_type, self.direction)
    }
}

impl AttributeKey {
    pub fn new(attr_type: AttributeType, direction: AttributeDirection) -> Self {
        AttributeKey {
            attr_type,
            direction,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Attribute<T> {
    pub dependencies: HashSet<Dependency>,
    pub expression: Expression<T>,
    pub value: Option<i32>,
}

impl<T: Unit<T>> Attribute<T> {
    pub fn new(expr: Expression<T>) -> Self {
        let deps = expr.get_dependencies();

        Attribute {
            dependencies: deps,
            expression: expr,
            value: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Dependency {
    pub id: String,
    pub key: AttributeKey,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.key)
    }
}

impl Dependency {
    pub fn new_direct(id: String, key: AttributeKey) -> Self {
        Dependency { id, key }
    }
    pub fn new(id: String, attr_type: AttributeType, dir: AttributeDirection) -> Self {
        let key = AttributeKey::new(attr_type, dir);
        Dependency { id, key }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AttributeType {
    Position,
    ChildPosition,
    Size,
    ChildSize,
}

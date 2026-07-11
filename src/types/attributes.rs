use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::types::{expressions::Expression, mults::AttributeDirection, values::Value};

pub struct Attributes {
    pub map: HashMap<Dependency, Attribute>,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            map: HashMap::new(),
        }
    }

    pub fn get_computed_value(
        &self,
        id: &str,
        attr_type: AttributeType,
        attr_dir: AttributeDirection,
    ) -> Value {
        self.map
            .get(&Dependency::new(id.to_string(), attr_type, attr_dir))
            .expect("not found")
            .value
            .clone()
            .expect("not computed yet")
    }

    pub fn get_computed_custom_value(&self, id: &str, name: &str) -> Value {
        self.map
            .get(&Dependency::new(
                id.to_string(),
                AttributeType::Extra(name.to_string()),
                AttributeDirection::None,
            ))
            .expect("not found")
            .value
            .clone()
            .expect("not computed yet")
    }
}

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

    pub fn new_extra(name: &str) -> Self {
        AttributeKey::new(
            AttributeType::Extra(name.to_string()),
            AttributeDirection::None,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub dependencies: HashSet<Dependency>,
    pub expression: Expression,
    pub value: Option<Value>,
}

impl Attribute {
    pub fn new(expr: Expression) -> Self {
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
    Extra(String),
}

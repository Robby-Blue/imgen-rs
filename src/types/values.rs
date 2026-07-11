use std::collections::HashMap;

use crate::{
    layout_plan::LayoutComponentContext,
    types::{
        attributes::{Attribute, AttributeKey, AttributeType, Attributes, Dependency},
        expressions::Expression,
        mults::{AttributeDirection, Directions},
    },
};

#[derive(Clone)]
pub struct PosPx {
    pub x: i32,
    pub y: i32,
}

impl PosPx {
    pub fn new(x: i32, y: i32) -> Self {
        PosPx { x, y }
    }
}

#[derive(Clone)]
pub struct SizePx {
    pub w: u32,
    pub h: u32,
}

impl SizePx {
    pub fn new(w: u32, h: u32) -> Self {
        SizePx { w, h }
    }
}

// TODO: turn Px into Size, add a SizeValue, with px, mm, cm, etc
#[derive(Clone, Debug)]
pub enum Value {
    Px(f64),
    Scalar(f64),
    Text(String),
    HexColor(u32),
}

impl Value {
    pub fn to_simplest(&self, attrs: &Attributes) -> Value {
        self.clone()
    }

    pub fn evaluate_addition(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Scalar(v1), Value::Scalar(v2)) => Value::Scalar(v1 + v2),
            (Value::Px(v1), Value::Px(v2)) => Value::Px(v1 + v2),
            (Value::Text(v1), Value::Text(v2)) => Value::Text(v1.to_string() + v2),

            (_, _) => panic!("bad addition"),
        }
    }

    pub fn evaluate_subtraction(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Scalar(v1), Value::Scalar(v2)) => Value::Scalar(v1 - v2),
            (Value::Px(v1), Value::Px(v2)) => Value::Px(v1 - v2),
            (_, _) => panic!("bad subtraction"),
        }
    }

    pub fn evaluate_multiplication(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Scalar(v1), Value::Scalar(v2)) => Value::Scalar(v1 * v2),
            (Value::Px(v1), Value::Px(v2)) => Value::Px(v1 * v2),
            (Value::Px(v1), Value::Scalar(v2)) => Value::Px(v1 * v2),
            (Value::Scalar(v1), Value::Px(v2)) => Value::Px(v1 * v2),
            (_, _) => panic!("bad multiplication"),
        }
    }

    pub fn evaluate_division(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Scalar(v1), Value::Scalar(v2)) => Value::Scalar(v1 / v2),
            (Value::Px(v1), Value::Px(v2)) => Value::Px(v1 / v2),
            (Value::Px(v1), Value::Scalar(v2)) => Value::Px(v1 / v2),
            (_, _) => panic!("bad division"),
        }
    }
}

#[macro_export]
macro_rules! expect_unit {
    ($unit:path, $stream:expr) => {{
        match $stream {
            $unit(data) => Some(data),
            _ => None, // TODO err handling stuff
        }
    }};
}

#[derive(Clone)]
pub enum Position {
    Auto,
    Float(Expression),
}

impl Directions<Position> {
    pub fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute> {
        let mut attrs = HashMap::new();

        attrs.insert(
            AttributeKey::new(AttributeType::Position, AttributeDirection::Horizontal),
            Attribute::new(
                self.horizontal
                    .get_expression(AttributeDirection::Horizontal, ctx),
            ),
        );
        attrs.insert(
            AttributeKey::new(AttributeType::Position, AttributeDirection::Vertical),
            Attribute::new(
                self.vertical
                    .get_expression(AttributeDirection::Vertical, ctx),
            ),
        );

        attrs
    }
}

impl Position {
    pub fn get_expression(
        &self,
        direction: AttributeDirection,
        ctx: &LayoutComponentContext,
    ) -> Expression {
        match self {
            Position::Auto => Expression::Variable(Dependency::new(
                ctx.parent_id.clone().expect("no auto without parent"),
                AttributeType::ChildPosition,
                direction,
            )),
            Position::Float(e) => e.clone(),
        }
    }
}

#[derive(Clone)]
pub enum Size {
    Max,
    Min,
    Set(Expression),
}

impl Directions<Size> {
    pub fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute> {
        let mut attrs = HashMap::new();

        attrs.insert(
            AttributeKey::new(AttributeType::Size, AttributeDirection::Horizontal),
            Attribute::new(
                self.horizontal
                    .get_expression(AttributeDirection::Horizontal, ctx),
            ),
        );
        attrs.insert(
            AttributeKey::new(AttributeType::Size, AttributeDirection::Vertical),
            Attribute::new(
                self.vertical
                    .get_expression(AttributeDirection::Vertical, ctx),
            ),
        );

        attrs
    }
}

impl Size {
    pub fn get_expression(
        &self,
        direction: AttributeDirection,
        ctx: &LayoutComponentContext,
    ) -> Expression {
        match self {
            Size::Max => Expression::Variable(Dependency::new(
                ctx.parent_id.clone().expect("max size without parent"),
                AttributeType::Size,
                direction,
            )),
            Size::Min => {
                todo!("min expr")
            }
            Size::Set(e) => e.clone(),
        }
    }
}

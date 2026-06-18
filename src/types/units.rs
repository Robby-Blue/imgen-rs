use std::collections::HashMap;

use crate::{
    layout_plan::{Attributes, LayoutComponentContext},
    types::{
        attributes::{Attribute, AttributeKey, AttributeType, Dependency},
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

#[derive(Clone)]
pub struct UnitValue<T> {
    pub value: i32,
    pub unit: Option<T>,
}

impl<T: Unit<T>> UnitValue<T> {
    pub fn new(value: i32, unit: T) -> Self {
        UnitValue {
            value,
            unit: Some(unit),
        }
    }

    pub fn new_direct(value: i32, unit: Option<T>) -> Self {
        UnitValue { value, unit }
    }

    pub fn new_unitless(value: i32) -> Self {
        UnitValue { value, unit: None }
    }

    pub fn new_px(value: i32) -> UnitValue<SizeUnit> {
        UnitValue::new(value, SizeUnit::Px)
    }

    pub fn to_simplest(&self, attrs: &Attributes<T>) -> UnitValue<T> {
        match &self.unit {
            Some(u) => u.to_simplest(self.value, attrs),
            None => UnitValue::new_unitless(self.value),
        }
    }
}

pub trait Unit<T> {
    fn to_simplest(&self, value: i32, attrs: &Attributes<T>) -> UnitValue<T>;
}

#[derive(Clone)]
pub enum SizeUnit {
    Px,
    Percent,
}

impl Unit<SizeUnit> for SizeUnit {
    fn to_simplest(&self, value: i32, _attrs: &Attributes<SizeUnit>) -> UnitValue<SizeUnit> {
        let px_count = match self {
            SizeUnit::Px => value,
            SizeUnit::Percent => todo!("percents, and deps for percents"),
        };

        UnitValue::<SizeUnit>::new_px(px_count)
    }
}

#[derive(Clone)]
pub enum Position {
    Auto,
    Float(Expression<SizeUnit>),
}

impl Directions<Position> {
    pub fn get_attributes(
        &self,
        ctx: &LayoutComponentContext,
    ) -> HashMap<AttributeKey, Attribute<SizeUnit>> {
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
    ) -> Expression<SizeUnit> {
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
    Set(Expression<SizeUnit>),
}

impl Directions<Size> {
    pub fn get_attributes(
        &self,
        ctx: &LayoutComponentContext,
    ) -> HashMap<AttributeKey, Attribute<SizeUnit>> {
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
    ) -> Expression<SizeUnit> {
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

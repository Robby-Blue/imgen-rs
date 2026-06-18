use std::collections::HashMap;

use crate::component::Component;
use crate::layout_plan::Attributes;
use crate::layout_plan::LayoutComponentContext;
use crate::primitives::primitives::Primitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::attributes::AttributeType;
use crate::types::attributes::Dependency;
use crate::types::expressions::Expression;
use crate::types::expressions::Operation;
use crate::types::expressions::Operator;
use crate::types::mults::AttributeDirection;
use crate::types::mults::Directions;
use crate::units::Size;
use crate::units::SizeUnit;
use crate::units::UnitValue;

pub struct Root {
    size: Directions<Size>,
}

impl Root {
    pub fn new(size: Directions<Size>) -> Self {
        Root { size }
    }
}

impl Component for Root {
    fn to_primitives(&self, _id: String, _attrs: &Attributes<SizeUnit>) -> Vec<Box<dyn Primitive>> {
        vec![]
    }
    fn get_attributes(
        &self,
        ctx: &LayoutComponentContext,
    ) -> HashMap<AttributeKey, Attribute<SizeUnit>> {
        let id = ctx.id.clone();
        let mut attrs = self.size.get_attributes(ctx);
        attrs.insert(
            AttributeKey::new(AttributeType::Position, AttributeDirection::Horizontal),
            Attribute::new(Expression::Operation(Operation::new(
                Box::new(Expression::Variable(Dependency::new(
                    id.clone(),
                    AttributeType::Size,
                    AttributeDirection::Vertical,
                ))),
                Operator::Division,
                Box::new(Expression::Value(UnitValue::new_unitless(2))),
            ))),
        );
        attrs.insert(
            AttributeKey::new(AttributeType::Position, AttributeDirection::Vertical),
            Attribute::new(Expression::Operation(Operation::new(
                Box::new(Expression::Variable(Dependency::new(
                    id.clone(),
                    AttributeType::Size,
                    AttributeDirection::Vertical,
                ))),
                Operator::Division,
                Box::new(Expression::Value(UnitValue::new_unitless(2))),
            ))),
        );
        attrs
    }
}

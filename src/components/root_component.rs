use std::collections::HashMap;

use crate::component::Component;
use crate::layout_plan::LayoutComponentContext;
use crate::parser::component_builder::ArgumentsMap;
use crate::primitives::primitives::Primitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::attributes::AttributeType;
use crate::types::attributes::Attributes;
use crate::types::attributes::Dependency;
use crate::types::expressions::Expression;
use crate::types::expressions::Operation;
use crate::types::expressions::Operator;
use crate::types::mults::AttributeDirection;
use crate::types::mults::Directions;
use crate::types::values::Value;
use crate::values::Size;

pub struct Root {
    size: Directions<Size>,
}

impl Root {
    pub fn new(size: Directions<Size>) -> Self {
        Root { size }
    }
    pub fn new_from_args(args: ArgumentsMap) -> Self {
        let width = args.get("width", Expression::Value(Value::Px(100.0)));
        let height = args.get("height", Expression::Value(Value::Px(100.0)));

        let size = Directions::new(Size::Set(width), Size::Set(height));

        Self::new(size)
    }
}

impl Component for Root {
    fn to_primitives(&self, _id: String, _attrs: &Attributes) -> Vec<Box<dyn Primitive>> {
        vec![]
    }
    fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute> {
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
                Box::new(Expression::Value(Value::Scalar(2.0))),
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
                Box::new(Expression::Value(Value::Scalar(2.0))),
            ))),
        );
        attrs
    }
}

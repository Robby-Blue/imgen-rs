use std::collections::HashMap;

use image::Rgba;

use crate::component::Component;
use crate::expect_unit;
use crate::layout_plan::LayoutComponentContext;
use crate::parser::component_builder::ArgumentsMap;
use crate::primitives::primitives::Primitive;
use crate::primitives::primitives::RectPrimitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::attributes::AttributeType;
use crate::types::attributes::Attributes;
use crate::types::expressions::Expression;
use crate::types::mults::AttributeDirection;
use crate::types::mults::Directions;
use crate::types::values::Value;
use crate::values::PosPx;
use crate::values::Position;
use crate::values::Size;
use crate::values::SizePx;

pub struct RectComponent {
    position: Directions<Position>,
    size: Directions<Size>,
    color: Expression,
    border_radius: Expression,
    margin: Expression,
    padding: Expression,
}

impl RectComponent {
    pub fn new(
        position: Directions<Position>,
        size: Directions<Size>,
        color: Expression,
        border_radius: Expression,
        margin: Expression,
        padding: Expression,
    ) -> Self {
        RectComponent {
            position,
            color,
            size,
            border_radius,
            margin,
            padding,
        }
    }
    pub fn new_from_args(args: ArgumentsMap) -> Self {
        let x = args.get("x", Expression::Value(Value::Px(0.0)));
        let y = args.get("y", Expression::Value(Value::Px(0.0)));

        let pos = Directions::new(Position::Float(x), Position::Float(y));

        let width = args.get("width", Expression::Value(Value::Px(100.0)));
        let height = args.get("height", Expression::Value(Value::Px(100.0)));

        let size = Directions::new(Size::Set(width), Size::Set(height));
        let color = args.get("color", Expression::Value(Value::HexColor(0)));
        let border_radius = args.get("border_radius", Expression::Value(Value::Px(0.0)));
        let margin = args.get("margin", Expression::Value(Value::Px(0.0)));
        let padding = args.get("padding", Expression::Value(Value::Px(0.0)));

        Self::new(pos, size, color, border_radius, margin, padding)
    }
}

impl Component for RectComponent {
    fn to_primitives(&self, id: String, attrs: &Attributes) -> Vec<Box<dyn Primitive>> {
        let x_mid =
            attrs.get_computed_value(&id, AttributeType::Position, AttributeDirection::Horizontal);
        let y_mid =
            attrs.get_computed_value(&id, AttributeType::Position, AttributeDirection::Vertical);
        let w = attrs.get_computed_value(&id, AttributeType::Size, AttributeDirection::Horizontal);
        let h = attrs.get_computed_value(&id, AttributeType::Size, AttributeDirection::Vertical);

        let x_mid = expect_unit!(Value::Px, x_mid).unwrap();
        let y_mid = expect_unit!(Value::Px, y_mid).unwrap();
        let w = expect_unit!(Value::Px, w).unwrap();
        let h = expect_unit!(Value::Px, h).unwrap();

        let size = SizePx::new(w as u32, h as u32);
        let pos = PosPx::new((x_mid - (w / 2.0)) as i32, (y_mid - (h / 2.0)) as i32);

        // TODO: border_radius

        let hex = attrs.get_computed_custom_value(&id, "color_hex");
        let hex = expect_unit!(Value::HexColor, hex).unwrap();
        let color = Rgba::from(hex.to_be_bytes());

        let rect = RectPrimitive::new(pos, size, color, 0);

        vec![Box::new(rect)]
    }
    fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute> {
        let mut attrs = self.size.get_attributes(ctx);
        let pos_attrs = self.position.get_attributes(ctx);

        attrs.extend(pos_attrs);

        attrs.insert(
            AttributeKey::new_extra("color_hex"),
            Attribute::new(self.color.clone()),
        );

        attrs
    }
}

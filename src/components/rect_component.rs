use std::collections::HashMap;

use image::Rgba;

use crate::component::Component;
use crate::layout_plan::Attributes;
use crate::layout_plan::LayoutComponentContext;
use crate::primitives::primitives::Primitive;
use crate::primitives::primitives::RectPrimitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::attributes::AttributeType;
use crate::types::mults::AttributeDirection;
use crate::types::mults::Directions;
use crate::units::PosPx;
use crate::units::Position;
use crate::units::Size;
use crate::units::SizePx;
use crate::units::SizeUnit;

pub struct RectComponent {
    position: Directions<Position>,
    size: Directions<Size>,
    color: Rgba<u8>,
    border_radius: u32,
    margin: u32,
    padding: u32,
}

impl RectComponent {
    pub fn new(
        position: Directions<Position>,
        size: Directions<Size>,
        color: Rgba<u8>,
        border_radius: u32,
        margin: u32,
        padding: u32,
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
}

impl Component for RectComponent {
    fn to_primitives(&self, id: String, attrs: &Attributes<SizeUnit>) -> Vec<Box<dyn Primitive>> {
        let x_mid =
            attrs.get_computed_value(&id, AttributeType::Position, AttributeDirection::Horizontal);
        let y_mid =
            attrs.get_computed_value(&id, AttributeType::Position, AttributeDirection::Vertical);
        let w = attrs.get_computed_value(&id, AttributeType::Size, AttributeDirection::Horizontal);
        let h = attrs.get_computed_value(&id, AttributeType::Size, AttributeDirection::Vertical);

        let size = SizePx::new(w as u32, h as u32);
        let pos = PosPx::new(x_mid - (w / 2), y_mid - (h / 2));

        let rect = RectPrimitive::new(pos, size, self.color, self.border_radius);

        vec![Box::new(rect)]
    }
    fn get_attributes(
        &self,
        ctx: &LayoutComponentContext,
    ) -> HashMap<AttributeKey, Attribute<SizeUnit>> {
        let mut attrs = self.size.get_attributes(ctx);
        let pos_attrs = self.position.get_attributes(ctx);

        attrs.extend(pos_attrs);
        attrs
    }
}

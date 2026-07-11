use std::collections::HashMap;

use crate::component::Component;
use crate::types::attributes::Attributes;
use crate::layout_plan::LayoutComponentContext;
use crate::primitives::primitives::Primitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::mults::Directions;
use crate::values::Position;
use crate::values::Size;

pub struct ContainerComponent {
    position: Directions<Position>,
    size: Directions<Size>,
}

impl ContainerComponent {
    pub fn new(position: Directions<Position>, size: Directions<Size>) -> Self {
        ContainerComponent { position, size }
    }
}

impl Component for ContainerComponent {
    fn to_primitives(&self, _id: String, _attrs: &Attributes) -> Vec<Box<dyn Primitive>> {
        vec![]
    }
    fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute> {
        let mut attrs = self.size.get_attributes(ctx);
        let pos_attrs = self.position.get_attributes(ctx);

        attrs.extend(pos_attrs);
        attrs
    }
}

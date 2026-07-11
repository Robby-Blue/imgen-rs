use std::collections::HashMap;

use crate::layout_plan::LayoutComponentContext;
use crate::primitives::primitives::Primitive;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeKey;
use crate::types::attributes::Attributes;

pub trait Component {
    fn to_primitives(&self, id: String, attrs: &Attributes) -> Vec<Box<dyn Primitive>>;
    fn get_attributes(&self, ctx: &LayoutComponentContext) -> HashMap<AttributeKey, Attribute>;
}

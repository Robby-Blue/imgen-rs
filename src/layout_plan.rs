use std::fmt::Debug;

use crate::component::Component;
use crate::types::attributes::Attribute;
use crate::types::attributes::AttributeType;
use crate::types::attributes::Attributes;
use crate::types::attributes::Dependency;
use crate::types::expressions::Expression;
use crate::types::mults::AttributeDirection;

pub struct LayoutPlan {}

impl LayoutPlan {
    pub fn new() -> Self {
        LayoutPlan {}
    }

    pub fn plan(&self, components: &Vec<LayoutComponent>) -> Attributes {
        let mut attributes = Attributes::new();

        for component in components {
            let ctx = LayoutComponentContext::new(&component);

            let attrs = component.component.get_attributes(&ctx);

            let id = component.id.clone();
            attributes.map.extend(
                attrs
                    .into_iter()
                    .map(|f| (Dependency::new_direct(id.clone(), f.0), f.1)),
            );

            attributes.map.insert(
                Dependency::new(
                    id.clone(),
                    AttributeType::ChildPosition,
                    AttributeDirection::Vertical,
                ),
                Attribute::new(Expression::Variable(Dependency::new(
                    id.clone(),
                    AttributeType::Position,
                    AttributeDirection::Vertical,
                ))),
            );
            attributes.map.insert(
                Dependency::new(
                    id.clone(),
                    AttributeType::ChildPosition,
                    AttributeDirection::Horizontal,
                ),
                Attribute::new(Expression::Variable(Dependency::new(
                    id.clone(),
                    AttributeType::Position,
                    AttributeDirection::Horizontal,
                ))),
            );
        }

        // TODO: use cache etc, build tree
        for key in attributes.map.keys().cloned().collect::<Vec<Dependency>>() {
            let val = {
                let attr = attributes.map.get(&key).unwrap();
                attr.expression.evaluate(&attributes)
            };

            attributes.map.get_mut(&key).unwrap().value = Some(val);
        }

        attributes
    }
}

pub struct LayoutComponent {
    pub component: Box<dyn Component>,
    pub id: String,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
}

impl LayoutComponent {
    pub fn new(
        component: Box<dyn Component>,
        id: String,
        parent_id: Option<String>,
        children_ids: Vec<String>,
    ) -> Self {
        LayoutComponent {
            component: component,
            id,
            parent_id,
            children_ids,
        }
    }
}

impl Debug for LayoutComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutComponent")
            .field("id", &self.id)
            .field("parent_id", &self.parent_id)
            .field("children_ids", &self.children_ids)
            .finish()
    }
}

pub struct LayoutComponentContext {
    pub id: String,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
}

impl LayoutComponentContext {
    pub fn new(component: &LayoutComponent) -> Self {
        LayoutComponentContext {
            id: component.id.clone(),
            parent_id: component.parent_id.clone(),
            children_ids: component.children_ids.clone(),
        }
    }
}
